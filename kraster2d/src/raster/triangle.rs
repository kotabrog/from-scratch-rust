use kmath::{Vec2, Vec3, num::EPS};
use kpix::Color;

use super::triangle_setup::{bbox_clamped, edge_function, is_top_left, signed_area};
use crate::core::frame::Frame;

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
    let p0 = Vec2::new(v0.pos.x, v0.pos.y);
    let p1 = Vec2::new(v1.pos.x, v1.pos.y);
    let p2 = Vec2::new(v2.pos.x, v2.pos.y);

    // Signed area (double area). Skip degenerate triangles.
    let area = signed_area(p0, p1, p2);
    if area.abs() <= EPS {
        return;
    }

    // For consistent inside test, make area positive by flipping orientation if needed.
    let (p0, p1, p2) = if area < 0.0 {
        (p0, p2, p1)
    } else {
        (p0, p1, p2)
    };

    // Bounding box, clamped to surface.
    let (min_x, min_y, max_x, max_y) = bbox_clamped(p0, p1, p2, frame.width(), frame.height());

    // Precompute edge Top-Left flags for strict/loose comparisons.
    let tl0 = is_top_left(p1, p2);
    let tl1 = is_top_left(p2, p0);
    let tl2 = is_top_left(p0, p1);

    // Iterate pixels; sample at pixel center.
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let p = Vec2::new(px, py);

            let w0 = edge_function(p1, p2, p);
            let w1 = edge_function(p2, p0, p);
            let w2 = edge_function(p0, p1, p);

            // Top-Left rule: allow equality when the edge is top-left, otherwise require strictly positive.
            let inside0 = if tl0 { w0 >= 0.0 } else { w0 > 0.0 };
            let inside1 = if tl1 { w1 >= 0.0 } else { w1 > 0.0 };
            let inside2 = if tl2 { w2 >= 0.0 } else { w2 > 0.0 };

            if inside0 && inside1 && inside2 {
                frame.set_pixel(x, y, color);
            }
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
}
