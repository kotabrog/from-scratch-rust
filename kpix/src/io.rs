use std::io::{self, Write};
use std::path::Path;

use crate::core::Surface;

/// Write the surface as binary PPM (P6). Alpha is ignored.
/// Delegates to `kimgfmt` for encoding.
pub fn write_ppm(surface: &Surface, path: impl AsRef<Path>) -> io::Result<()> {
    kimgfmt::ppm::write_ppm_from_rgba_le(surface.pixels(), surface.width(), surface.height(), path)
}

/// Write PPM to any writer. Useful for testing. Delegates to `kimgfmt`.
pub fn write_ppm_to_writer(surface: &Surface, mut w: impl Write) -> io::Result<()> {
    kimgfmt::ppm::write_ppm_from_rgba_le_to_writer(
        surface.pixels(),
        surface.width(),
        surface.height(),
        &mut w,
    )
}

/// Write the surface as 24-bit BMP (BGR, BI_RGB, top-down).
/// Delegates to `kimgfmt::bmp` for encoding.
pub fn write_bmp(surface: &Surface, path: impl AsRef<Path>) -> io::Result<()> {
    kimgfmt::bmp::write_bmp24_from_rgba_le(
        surface.pixels(),
        surface.width(),
        surface.height(),
        path,
    )
}

/// Write BMP to any writer.
pub fn write_bmp_to_writer(surface: &Surface, mut w: impl Write) -> io::Result<()> {
    kimgfmt::bmp::write_bmp24_from_rgba_le_to_writer(
        surface.pixels(),
        surface.width(),
        surface.height(),
        &mut w,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

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

    #[test]
    fn bmp_header_and_data() {
        let mut s = Surface::new(2, 1);
        s.set_pixel(0, 0, Color::rgba(10, 20, 30, 255));
        s.set_pixel(1, 0, Color::rgba(40, 50, 60, 255));

        let mut buf = Vec::new();
        write_bmp_to_writer(&s, &mut buf).unwrap();

        // Signature, headers, and BGR payload with row padding
        assert_eq!(&buf[0..2], b"BM");
        // Pixel data starts at 54
        let data = &buf[54..];
        // 2x1 -> row_bytes=6, padding=2 -> total=8
        assert_eq!(data.len(), 8);
        assert_eq!(&data[0..3], &[30, 20, 10]);
        assert_eq!(&data[3..6], &[60, 50, 40]);
        assert_eq!(&data[6..8], &[0, 0]);
    }
}
