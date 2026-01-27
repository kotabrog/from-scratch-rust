use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

/// Write the given RGBA little-endian pixel buffer as binary PPM (P6).
/// - `pixels`: slice of packed RGBA `u32` in little-endian order per pixel.
/// - Layout: row-major, top-left origin, width x height.
/// - Alpha is ignored; only RGB bytes are written.
pub fn write_ppm_from_rgba_le(
    pixels: &[u32],
    width: usize,
    height: usize,
    path: impl AsRef<Path>,
) -> io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    write_ppm_from_rgba_le_to_writer(pixels, width, height, &mut w)
}

/// Core PPM (P6) writer to any `Write`.
/// Writes header `P6\n<w> <h>\n255\n` then width*height RGB bytes.
pub fn write_ppm_from_rgba_le_to_writer(
    pixels: &[u32],
    width: usize,
    height: usize,
    mut w: impl Write,
) -> io::Result<()> {
    // Validate buffer size (avoid overflow on multiplication)
    let count = width
        .checked_mul(height)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "width*height overflow"))?;
    if pixels.len() < count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "pixels buffer is smaller than width*height",
        ));
    }

    // Header
    write!(w, "P6\n{} {}\n255\n", width, height)?;

    // Payload: RGB per pixel, ignoring alpha (RGBA little-endian -> [r,g,b,a])
    for &px in pixels.iter().take(count) {
        let [r, g, b, _a] = px.to_le_bytes();
        w.write_all(&[r, g, b])?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppm_header_and_data_from_rgba_le_module() {
        // 2x1 image
        let pixels = [
            u32::from_le_bytes([10, 20, 30, 255]),
            u32::from_le_bytes([40, 50, 60, 128]),
        ];
        let mut buf = Vec::new();
        write_ppm_from_rgba_le_to_writer(&pixels, 2, 1, &mut buf).unwrap();
        let header = b"P6\n2 1\n255\n";
        assert!(buf.starts_with(header));
        assert_eq!(&buf[header.len()..], &[10, 20, 30, 40, 50, 60]);
    }
}
