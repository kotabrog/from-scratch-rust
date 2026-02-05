use kdev::out;
use kpix::{Color, Surface, io};

fn main() {
    let (w, h) = (256usize, 256usize);

    let mut s = Surface::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let r = x as u8;
            let g = y as u8;
            let b = ((x + y) as u8).wrapping_div(2);
            s.set_pixel(x as i32, y as i32, Color::rgba(r, g, b, 255));
        }
    }

    // Write outputs under target/examples/gradient
    let out_dir = out::example_output_dir("gradient").expect("failed to create output directory");
    io::write_ppm(&s, out_dir.join("gradient.ppm")).expect("failed to write PPM");
    io::write_bmp(&s, out_dir.join("gradient.bmp")).expect("failed to write BMP");
}
