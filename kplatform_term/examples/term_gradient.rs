use std::time::Duration;

use kpix::{Color, Surface};
use kplatform_core::{Platform, WindowConfig};
use kplatform_term::TermPlatform;

fn main() {
    let (w, h) = (80u32, 24u32);
    let cfg = WindowConfig::new("term-gradient", w, h);
    let mut plat = TermPlatform::new(&cfg).expect("term platform");

    let mut surf = Surface::new(w as usize, h as usize);
    for frame in 0..30u32 {
        // Simple animated gradient
        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let r = ((x + frame as i32) as u8).wrapping_mul(3);
                let g = ((y + frame as i32) as u8).wrapping_mul(5);
                let b = ((x + y + frame as i32) as u8).wrapping_mul(7);
                surf.set_pixel(x, y, Color::rgba(r, g, b, 255));
            }
        }
        plat.present_rgba_le(w, h, surf.pixels())
            .expect("present to term");
        plat.sleep(Duration::from_millis(33));
    }
}
