use kmath::Vec3;
use kpix::Color;
use kraster2d::{
    Frame,
    raster::{Vertex, draw_triangle_solid},
};

fn main() {
    let mut frame = Frame::new(256, 256);
    frame.clear(Color::rgba(20, 30, 50, 255));

    let red = Color::rgba(220, 80, 80, 255);
    let v0 = Vertex::new(Vec3::new(30.0, 30.0, 0.0));
    let v1 = Vertex::new(Vec3::new(220.0, 40.0, 0.0));
    let v2 = Vertex::new(Vec3::new(60.0, 200.0, 0.0));
    draw_triangle_solid(&mut frame, v0, v1, v2, red);

    kraster2d::io::write::write_ppm(&frame, "tri_solid.ppm").expect("failed to write PPM");
}
