use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

const FILE_HEADER_SIZE: u32 = 14;
const INFO_HEADER_SIZE: u32 = 40; // BITMAPINFOHEADER
const PIXEL_DATA_OFFSET: u32 = FILE_HEADER_SIZE + INFO_HEADER_SIZE; // 54

/// Write the given RGBA little-endian pixels as 24-bit BMP (BGR, BI_RGB) to a file.
/// Top-down orientation (negative height) to match row-major top-left origin.
pub fn write_bmp24_from_rgba_le(
    pixels: &[u32],
    width: usize,
    height: usize,
    path: impl AsRef<Path>,
) -> io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    write_bmp24_from_rgba_le_to_writer(pixels, width, height, &mut w)
}

/// Core BMP (24-bit, BI_RGB) writer to any `Write`.
/// - BGR order per pixel, rows written top-to-bottom (negative height in header)
/// - Rows padded to 4-byte boundaries with zeros
pub fn write_bmp24_from_rgba_le_to_writer(
    pixels: &[u32],
    width: usize,
    height: usize,
    mut w: impl Write,
) -> io::Result<()> {
    // Validate and compute sizes, guarding against overflow
    let count = width
        .checked_mul(height)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "width*height overflow"))?;
    if pixels.len() < count {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "pixels buffer is smaller than width*height",
        ));
    }
    let row_bytes = width
        .checked_mul(3)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "row bytes overflow"))?;
    let padding = (4 - (row_bytes % 4)) % 4;
    let row_padded = row_bytes + padding;
    let image_size = row_padded
        .checked_mul(height)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "image size overflow"))?
        as u32;
    let file_size = (PIXEL_DATA_OFFSET)
        .checked_add(image_size)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "file size overflow"))?;

    // BITMAPFILEHEADER (14 bytes)
    // Signature 'BM'
    w.write_all(b"BM")?;
    w.write_all(&file_size.to_le_bytes())?; // file size
    w.write_all(&0u16.to_le_bytes())?; // reserved1
    w.write_all(&0u16.to_le_bytes())?; // reserved2
    w.write_all(&PIXEL_DATA_OFFSET.to_le_bytes())?; // pixel data offset

    // BITMAPINFOHEADER (40 bytes)
    let width_i32 = i32::try_from(width)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "width too large"))?;
    let height_i32 = i32::try_from(height)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "height too large"))?;
    w.write_all(&INFO_HEADER_SIZE.to_le_bytes())?; // header size
    w.write_all(&width_i32.to_le_bytes())?; // width
    w.write_all(&(-height_i32).to_le_bytes())?; // negative height => top-down
    w.write_all(&1u16.to_le_bytes())?; // planes
    w.write_all(&24u16.to_le_bytes())?; // bit count
    w.write_all(&0u32.to_le_bytes())?; // compression = BI_RGB
    w.write_all(&image_size.to_le_bytes())?; // image size
    w.write_all(&0u32.to_le_bytes())?; // x pixels per meter
    w.write_all(&0u32.to_le_bytes())?; // y pixels per meter
    w.write_all(&0u32.to_le_bytes())?; // colors used
    w.write_all(&0u32.to_le_bytes())?; // important colors

    // Pixel data: top-down, each row padded to 4-byte boundary. Per pixel: B, G, R (alpha ignored)
    let pad_bytes = [0u8; 3]; // up to 3 bytes of padding
    for y in 0..height {
        let start = y * width;
        let row = &pixels[start..start + width];
        for &px in row.iter() {
            let [r, g, b, _a] = px.to_le_bytes();
            w.write_all(&[b, g, r])?;
        }
        if padding != 0 {
            w.write_all(&pad_bytes[..padding])?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_u16_le(b: &[u8], off: usize) -> u16 {
        u16::from_le_bytes([b[off], b[off + 1]])
    }
    fn parse_u32_le(b: &[u8], off: usize) -> u32 {
        u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
    }
    fn parse_i32_le(b: &[u8], off: usize) -> i32 {
        i32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
    }

    #[test]
    fn bmp_1x1_header_and_pixel() {
        // 1x1: red pixel (r=200,g=100,b=50)
        let px = [u32::from_le_bytes([200, 100, 50, 255])];
        let mut buf = Vec::new();
        write_bmp24_from_rgba_le_to_writer(&px, 1, 1, &mut buf).unwrap();

        // File header
        assert_eq!(&buf[0..2], b"BM");
        let file_size = parse_u32_le(&buf, 2);
        let off_bits = parse_u32_le(&buf, 10);
        assert_eq!(off_bits, 54);

        // Info header
        let header_size = parse_u32_le(&buf, 14);
        assert_eq!(header_size, 40);
        let w = parse_i32_le(&buf, 18);
        let h = parse_i32_le(&buf, 22);
        assert_eq!(w, 1);
        assert_eq!(h, -1); // top-down
        assert_eq!(parse_u16_le(&buf, 26), 1); // planes
        assert_eq!(parse_u16_le(&buf, 28), 24); // bpp
        assert_eq!(parse_u32_le(&buf, 30), 0); // compression BI_RGB

        let image_size = parse_u32_le(&buf, 34);
        // row_bytes=3, padding=1 -> image_size=4
        assert_eq!(image_size, 4);
        assert_eq!(file_size as usize, 54 + 4);

        // Pixel data (B,G,R) with one padding byte
        let data = &buf[54..];
        assert_eq!(data.len(), 4);
        assert_eq!(&data[0..3], &[50, 100, 200]);
    }

    #[test]
    fn bmp_3x1_row_padding_and_order() {
        // 3x1: pixels rgb = (1,2,3), (10,20,30), (100,150,200)
        let px = [
            u32::from_le_bytes([1, 2, 3, 0]),
            u32::from_le_bytes([10, 20, 30, 0]),
            u32::from_le_bytes([100, 150, 200, 0]),
        ];
        let mut buf = Vec::new();
        write_bmp24_from_rgba_le_to_writer(&px, 3, 1, &mut buf).unwrap();
        // Row bytes=9, padding= (4 - 1) % 4 = 3
        let data = &buf[54..];
        assert_eq!(data.len(), 12);
        // First pixel B,G,R
        assert_eq!(&data[0..3], &[3, 2, 1]);
        assert_eq!(&data[3..6], &[30, 20, 10]);
        assert_eq!(&data[6..9], &[200, 150, 100]);
        assert_eq!(&data[9..12], &[0, 0, 0]); // padding
    }
}
