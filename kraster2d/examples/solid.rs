use kdev::out;
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

    let out_dir = out::example_output_dir("solid").expect("failed to create output directory");
    let path = out_dir.join("frame0000.ppm");
    io::write::write_ppm(&frame, path).expect("failed to write PPM");
}
