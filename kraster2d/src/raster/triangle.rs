use kmath::{Vec2, Vec3, num::EPS};
use kpix::Color;

use super::triangle_setup::{bbox_clamped, edge_function, is_top_left, signed_area};
use crate::core::frame::Frame;
use crate::core::texture::Texture;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: Vec3,       // screen space (x,y in pixels), z reserved
    pub uv: Vec2,        // for later phases
    pub color: [f32; 3], // for later phases
}

impl Vertex {
    pub fn new(pos: Vec3) -> Self {
        Self {
            pos,
            uv: Vec2::ZERO,
            color: [1.0, 1.0, 1.0],
        }
    }
}

/// Draw a filled triangle with a solid color using barycentric edge functions and Top-Left rule.
pub fn draw_triangle_solid(frame: &mut Frame, v0: Vertex, v1: Vertex, v2: Vertex, color: Color) {
    let (w, h) = (frame.width(), frame.height());
    raster_core(w, h, v0, v1, v2, |x, y, _a, _b, _c| {
        frame.set_pixel(x, y, color);
    });
}

/// Draw a filled triangle with vertex color interpolation.
pub fn draw_triangle_vertex_color(frame: &mut Frame, v0: Vertex, v1: Vertex, v2: Vertex) {
    let (w, h) = (frame.width(), frame.height());
    raster_core(w, h, v0, v1, v2, |x, y, a, b, c| {
        let r = (a * v0.color[0] + b * v1.color[0] + c * v2.color[0]).clamp(0.0, 1.0);
        let g = (a * v0.color[1] + b * v1.color[1] + c * v2.color[1]).clamp(0.0, 1.0);
        let bch = (a * v0.color[2] + b * v1.color[2] + c * v2.color[2]).clamp(0.0, 1.0);
        let to_u8 = |v: f32| -> u8 { (v * 255.0 + 0.5).floor().clamp(0.0, 255.0) as u8 };
        let col = Color::rgba(to_u8(r), to_u8(g), to_u8(bch), 255);
        frame.set_pixel(x, y, col);
    });
}

/// Draw a textured triangle with nearest sampling (uv interpolation, clamp).
pub fn draw_triangle_textured(
    frame: &mut Frame,
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    tex: &Texture,
) {
    let (w, h) = (frame.width(), frame.height());
    raster_core(w, h, v0, v1, v2, |x, y, a, b, c| {
        let uv = Vec2::new(
            a * v0.uv.x + b * v1.uv.x + c * v2.uv.x,
            a * v0.uv.y + b * v1.uv.y + c * v2.uv.y,
        );
        let col = tex.sample_nearest(uv);
        frame.set_pixel(x, y, col);
    });
}

/// Shared rasterization core: handles orientation, bbox, Top-Left rule, and barycentric weights.
/// The callback receives integer pixel coords and normalized weights (a,b,c) summing to ~1.
fn raster_core(
    width: usize,
    height: usize,
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    mut shade: impl FnMut(i32, i32, f32, f32, f32),
) {
    let p0 = Vec2::new(v0.pos.x, v0.pos.y);
    let p1 = Vec2::new(v1.pos.x, v1.pos.y);
    let p2 = Vec2::new(v2.pos.x, v2.pos.y);

    let area = signed_area(p0, p1, p2);
    if area.abs() <= EPS {
        return;
    }
    let sign = if area < 0.0 { -1.0 } else { 1.0 };
    let inv_area = 1.0 / area.abs();

    let (min_x, min_y, max_x, max_y) = bbox_clamped(p0, p1, p2, width, height);

    let tl0 = is_top_left(p1, p2);
    let tl1 = is_top_left(p2, p0);
    let tl2 = is_top_left(p0, p1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let w0 = sign * edge_function(p1, p2, p);
            let w1 = sign * edge_function(p2, p0, p);
            let w2 = sign * edge_function(p0, p1, p);

            let inside0 = if tl0 { w0 >= 0.0 } else { w0 > 0.0 };
            let inside1 = if tl1 { w1 >= 0.0 } else { w1 > 0.0 };
            let inside2 = if tl2 { w2 >= 0.0 } else { w2 > 0.0 };
            if !(inside0 && inside1 && inside2) {
                continue;
            }

            let a = w0 * inv_area;
            let b = w1 * inv_area;
            let c = w2 * inv_area;
            shade(x, y, a, b, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Frame;

    #[test]
    fn solid_triangle_renders_some_pixels() {
        let mut f = Frame::new(32, 32);
        let c = Color::rgba(200, 50, 50, 255);
        let v0 = Vertex::new(Vec3::new(5.0, 5.0, 0.0));
        let v1 = Vertex::new(Vec3::new(25.0, 6.0, 0.0));
        let v2 = Vertex::new(Vec3::new(10.0, 20.0, 0.0));
        draw_triangle_solid(&mut f, v0, v1, v2, c);
        let mut count = 0;
        for &px in f.pixels() {
            if px == c.to_u32() {
                count += 1;
            }
        }
        assert!(count > 0);
    }

    #[test]
    fn two_triangles_make_filled_rect_no_cracks() {
        let mut f = Frame::new(16, 16);
        let c = Color::rgba(80, 160, 200, 255);
        // Rectangle from (2,2) to (13,13)
        let a = Vec3::new(2.0, 2.0, 0.0);
        let b = Vec3::new(13.0, 2.0, 0.0);
        let d = Vec3::new(2.0, 13.0, 0.0);
        let cpt = Vec3::new(13.0, 13.0, 0.0);
        draw_triangle_solid(&mut f, Vertex::new(a), Vertex::new(b), Vertex::new(cpt), c);
        draw_triangle_solid(&mut f, Vertex::new(a), Vertex::new(cpt), Vertex::new(d), c);

        // With Top-Left rule, right/bottom edges are excluded: expect fill on [2..13) x [2..13)
        let mut ok = true;
        for y in 2..13 {
            for x in 2..13 {
                let idx = (y as usize) * f.width() + (x as usize);
                if f.pixels()[idx] != c.to_u32() {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        assert!(ok);
    }

    #[test]
    fn vertex_color_matches_solid_when_all_equal() {
        let mut f_solid = Frame::new(32, 32);
        let mut f_interp = Frame::new(32, 32);
        let v0 = Vertex {
            pos: Vec3::new(4.0, 4.0, 0.0),
            uv: Vec2::ZERO,
            color: [1.0, 1.0, 1.0],
        };
        let v1 = Vertex {
            pos: Vec3::new(28.0, 5.0, 0.0),
            uv: Vec2::ZERO,
            color: [1.0, 1.0, 1.0],
        };
        let v2 = Vertex {
            pos: Vec3::new(10.0, 26.0, 0.0),
            uv: Vec2::ZERO,
            color: [1.0, 1.0, 1.0],
        };
        let c = Color::rgba(255, 255, 255, 255);
        draw_triangle_solid(&mut f_solid, v0, v1, v2, c);
        draw_triangle_vertex_color(&mut f_interp, v0, v1, v2);
        assert_eq!(f_solid.pixels(), f_interp.pixels());
    }

    #[test]
    fn textured_triangle_constant_uv_samples_texel() {
        // Build a small texture: 2x2 with red at TL; sample uv=(0,0)
        let tex = Texture::from_rgba_le(
            vec![
                Color::rgba(255, 0, 0, 255).to_u32(),
                Color::rgba(0, 255, 0, 255).to_u32(),
                Color::rgba(0, 0, 255, 255).to_u32(),
                Color::rgba(255, 255, 255, 255).to_u32(),
            ],
            2,
            2,
        );
        let mut f = Frame::new(16, 16);
        let v0 = Vertex {
            pos: Vec3::new(2.0, 2.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
            color: [0.0; 3],
        };
        let v1 = Vertex {
            pos: Vec3::new(12.0, 2.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
            color: [0.0; 3],
        };
        let v2 = Vertex {
            pos: Vec3::new(2.0, 12.0, 0.0),
            uv: Vec2::new(0.0, 0.0),
            color: [0.0; 3],
        };
        draw_triangle_textured(&mut f, v0, v1, v2, &tex);
        // Find at least one filled pixel and assert it's red
        let red = Color::rgba(255, 0, 0, 255).to_u32();
        assert!(f.pixels().iter().any(|&px| px == red));
    }
}
