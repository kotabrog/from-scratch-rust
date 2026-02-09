use kdev::out;
use kmath::{Transform2D, Vec2, Vec3};
use kpix::Color;
use kraster2d::core::texture::Texture;
use kraster2d::raster::{Vertex, draw_triangle_textured};
use kraster2d::{Frame, io};

fn make_checker_tex(w: usize, h: usize, cell: usize) -> Texture {
    let mut px = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            let cx = (x / cell) % 2;
            let cy = (y / cell) % 2;
            let c = if (cx ^ cy) == 0 {
                Color::rgba(220, 220, 220, 255)
            } else {
                Color::rgba(40, 40, 40, 255)
            };
            px.push(c.to_u32());
        }
    }
    Texture::from_rgba_le(px, w, h)
}

fn main() {
    let mut frame = Frame::new(256, 256);
    let tex = make_checker_tex(64, 64, 8);
    let center = Vec2::new(128.0, 128.0);

    // Resolve output directory via kdev helper
    let out_dir =
        out::example_output_dir("rotating_quad").expect("failed to create output directory");

    // Quad centered at (128,128)
    let half = 60.0;
    let base = [
        Vec2::new(-half, -half),
        Vec2::new(half, -half),
        Vec2::new(half, half),
        Vec2::new(-half, half),
    ];
    let uvs = [
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];

    let frames = 60;
    for i in 0..frames {
        frame.clear(Color::rgba(20, 30, 50, 255));
        let t = i as f32 / frames as f32;
        let angle = t * std::f32::consts::TAU; // 0..2pi
        let tr = Transform2D::new(center, angle, Vec2::ONE);

        // Transform quad vertices
        let mut pts = [Vec2::ZERO; 4];
        for (k, p) in base.iter().enumerate() {
            pts[k] = tr.transform_point(*p);
        }

        // Emit two triangles: (0,1,2) and (0,2,3)
        let make_v = |p: Vec2, uv: Vec2| Vertex {
            pos: Vec3::new(p.x, p.y, 0.0),
            uv,
            color: [1.0; 3],
        };
        let v0 = make_v(pts[0], uvs[0]);
        let v1 = make_v(pts[1], uvs[1]);
        let v2 = make_v(pts[2], uvs[2]);
        let v3 = make_v(pts[3], uvs[3]);

        draw_triangle_textured(&mut frame, v0, v1, v2, &tex);
        draw_triangle_textured(&mut frame, v0, v2, v3, &tex);

        let path = out_dir.join(format!("frame{:04}.ppm", i));
        io::write::write_ppm(&frame, path).expect("failed to write PPM");
    }
}
