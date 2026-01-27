use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub mod bmp;

/// Image format selector for save helpers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Format {
    Ppm,
    Bmp24,
}

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

/// Save RGBA little-endian pixels to a file in the specified format.
pub fn save_rgba_le(
    pixels: &[u32],
    width: usize,
    height: usize,
    path: impl AsRef<Path>,
    format: Format,
) -> io::Result<()> {
    match format {
        Format::Ppm => write_ppm_from_rgba_le(pixels, width, height, path),
        Format::Bmp24 => bmp::write_bmp24_from_rgba_le(pixels, width, height, path),
    }
}

/// Save RGBA little-endian pixels to any writer in the specified format.
pub fn save_rgba_le_to_writer(
    pixels: &[u32],
    width: usize,
    height: usize,
    format: Format,
    mut w: impl Write,
) -> io::Result<()> {
    match format {
        Format::Ppm => write_ppm_from_rgba_le_to_writer(pixels, width, height, &mut w),
        Format::Bmp24 => bmp::write_bmp24_from_rgba_le_to_writer(pixels, width, height, &mut w),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ppm_header_and_data_from_rgba_le() {
        // 2x1 image: pixels = [(10,20,30,255), (40,50,60,128)]
        let pixels = [
            u32::from_le_bytes([10, 20, 30, 255]),
            u32::from_le_bytes([40, 50, 60, 128]),
        ];

        let mut buf = Vec::new();
        write_ppm_from_rgba_le_to_writer(&pixels, 2, 1, &mut buf).unwrap();

        let header = b"P6\n2 1\n255\n";
        assert!(buf.starts_with(header));
        let payload = &buf[header.len()..];
        assert_eq!(payload, &[10, 20, 30, 40, 50, 60]);
        // Total size must be header + 2*1*3 bytes
        assert_eq!(buf.len(), header.len() + 6);
    }

    #[test]
    fn ppm_buffer_size_validation() {
        let pixels = [u32::from_le_bytes([0, 0, 0, 0]); 1];
        // width*height=2 but buffer len=1 -> error
        let mut sink = Vec::new();
        let err = write_ppm_from_rgba_le_to_writer(&pixels, 2, 1, &mut sink).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn save_dispatch_ppm_and_bmp() {
        let px = [u32::from_le_bytes([1, 2, 3, 255])];
        // PPM
        let mut a = Vec::new();
        super::save_rgba_le_to_writer(&px, 1, 1, super::Format::Ppm, &mut a).unwrap();
        assert!(a.starts_with(b"P6\n1 1\n255\n"));
        assert_eq!(&a[a.len() - 3..], &[1, 2, 3]);
        // BMP
        let mut b = Vec::new();
        super::save_rgba_le_to_writer(&px, 1, 1, super::Format::Bmp24, &mut b).unwrap();
        assert_eq!(&b[0..2], b"BM");
        assert_eq!(&b[54..57], &[3, 2, 1]); // B,G,R
    }
}
