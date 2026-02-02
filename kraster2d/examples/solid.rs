use kpix::Color;
use kraster2d::{Frame, io};

fn main() {
    let mut frame = Frame::new(256, 256);
    frame.clear(Color::rgba(30, 60, 90, 255));

    // draw a simple diagonal pattern for visual confirmation
    let white = Color::rgba(255, 255, 255, 255);
    for i in 0..256i32 {
        frame.set_pixel(i, i, white);
        frame.set_pixel(255 - i, i, white);
    }

    io::write::write_ppm(&frame, "frame0000.ppm").expect("failed to write PPM");
}
