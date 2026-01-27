use std::env;
use std::path::PathBuf;

use kpix::{Color, Surface, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (w, h) = (256usize, 256usize);
    let out = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "gradient.ppm".into());

    let mut s = Surface::new(w, h);

    for y in 0..h {
        for x in 0..w {
            let r = x as u8;
            let g = y as u8;
            let b = ((x + y) as u8).wrapping_div(2);
            s.set_pixel(x as i32, y as i32, Color::rgba(r, g, b, 255));
        }
    }

    // Write PPM
    io::write_ppm(&s, &out).expect("failed to write PPM");
    // Also write BMP next to it (derive path by replacing/adding .bmp)
    let bmp_path = derive_bmp_path(&out);
    io::write_bmp(&s, &bmp_path).expect("failed to write BMP");
}

fn derive_bmp_path(out: &str) -> PathBuf {
    let mut p = PathBuf::from(out);
    if p.extension().map(|e| e == "ppm").unwrap_or(false) {
        p.set_extension("bmp");
        p
    } else {
        let mut with_ext = out.to_string();
        with_ext.push_str(".bmp");
        PathBuf::from(with_ext)
    }
}
