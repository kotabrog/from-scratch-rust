use std::io;
use std::path::Path;

use crate::core::frame::Frame;

/// Write the current frame as binary PPM (P6). Alpha is ignored.
pub fn write_ppm(frame: &Frame, path: impl AsRef<Path>) -> io::Result<()> {
    let s = frame.surface();
    kimgfmt::ppm::write_ppm_from_rgba_le(s.pixels(), s.width(), s.height(), path)
}

/// Write PPM to any writer. Useful for testing.
pub fn write_ppm_to_writer(frame: &Frame, mut w: impl io::Write) -> io::Result<()> {
    let s = frame.surface();
    kimgfmt::ppm::write_ppm_from_rgba_le_to_writer(s.pixels(), s.width(), s.height(), &mut w)
}

/// Optionally write as 24-bit BMP (BGR, BI_RGB, top-down).
pub fn write_bmp(frame: &Frame, path: impl AsRef<Path>) -> io::Result<()> {
    let s = frame.surface();
    kimgfmt::bmp::write_bmp24_from_rgba_le(s.pixels(), s.width(), s.height(), path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kpix::Color;

    #[test]
    fn ppm_header_and_data() {
        let mut f = crate::Frame::new(2, 1);
        f.set_pixel(0, 0, Color::rgba(10, 20, 30, 255));
        f.set_pixel(1, 0, Color::rgba(40, 50, 60, 255));

        let mut buf = Vec::new();
        write_ppm_to_writer(&f, &mut buf).unwrap();
        let header = b"P6\n2 1\n255\n";
        assert!(buf.starts_with(header));
        let payload = &buf[header.len()..];
        assert_eq!(payload, &[10, 20, 30, 40, 50, 60]);
    }
}
