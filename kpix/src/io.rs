use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use crate::core::{Color, Surface};

/// Write the surface as binary PPM (P6). Alpha is ignored.
/// Layout: header "P6\n<width> <height>\n255\n" followed by width*height RGB bytes.
pub fn write_ppm(surface: &Surface, path: impl AsRef<Path>) -> io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    write_ppm_to_writer(surface, &mut w)
}

/// Write PPM to any writer. Useful for testing.
pub fn write_ppm_to_writer(surface: &Surface, mut w: impl Write) -> io::Result<()> {
    let width = surface.width();
    let height = surface.height();
    // Header
    write!(w, "P6\n{} {}\n255\n", width, height)?;

    // Payload: RGB per pixel, ignoring alpha
    for &px in surface.pixels() {
        let Color { r, g, b, .. } = Color::from_u32(px);
        w.write_all(&[r, g, b])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppm_header_and_data() {
        let mut s = Surface::new(2, 1);
        s.set_pixel(0, 0, Color::rgba(10, 20, 30, 255));
        s.set_pixel(1, 0, Color::rgba(40, 50, 60, 255));

        let mut buf = Vec::new();
        write_ppm_to_writer(&s, &mut buf).unwrap();

        // Header should be: P6\n2 1\n255\n
        let header = b"P6\n2 1\n255\n";
        assert!(buf.starts_with(header));
        let payload = &buf[header.len()..];
        assert_eq!(payload, &[10, 20, 30, 40, 50, 60]);
    }
}
