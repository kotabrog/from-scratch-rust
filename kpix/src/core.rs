//! Core types: Color and Surface.

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to packed little-endian RGBA u32.
    #[inline]
    pub fn to_u32(self) -> u32 {
        u32::from_le_bytes([self.r, self.g, self.b, self.a])
    }

    /// Construct from packed little-endian RGBA u32.
    #[inline]
    pub fn from_u32(packed: u32) -> Self {
        let [r, g, b, a] = packed.to_le_bytes();
        Self { r, g, b, a }
    }
}

#[derive(Clone, Debug)]
pub struct Surface {
    width: usize,
    height: usize,
    pixels: Vec<u32>, // packed RGBA little-endian per pixel
}

impl Surface {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width.saturating_mul(height);
        Self {
            width,
            height,
            pixels: vec![0; len],
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Fill entire surface with a color.
    pub fn clear(&mut self, color: Color) {
        let v = color.to_u32();
        self.pixels.fill(v);
    }

    /// Set a pixel with clipping. Out-of-bounds coordinates are ignored.
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        self.pixels[idx] = color.to_u32();
    }

    /// Get a pixel if in-bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<Color> {
        if x < 0 || y < 0 {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        Some(Color::from_u32(self.pixels[idx]))
    }

    /// Read-only access to internal packed pixel buffer.
    pub fn pixels(&self) -> &[u32] {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Color
    #[test]
    fn color_pack_unpack_roundtrip() {
        let c = Color::rgba(10, 20, 30, 40);
        let p = c.to_u32();
        let d = Color::from_u32(p);
        assert_eq!(c, d);
    }

    // Surface
    #[test]
    fn surface_new_dimensions_and_clear() {
        let mut s = Surface::new(4, 3);
        assert_eq!(s.width(), 4);
        assert_eq!(s.height(), 3);

        let red = Color::rgba(255, 0, 0, 255);
        s.clear(red);
        for &px in s.pixels() {
            assert_eq!(px, red.to_u32());
        }
    }

    #[test]
    fn set_pixel_in_bounds_and_oob_ignored() {
        let mut s = Surface::new(2, 2);
        let c = Color::rgba(1, 2, 3, 4);
        s.set_pixel(1, 1, c);
        assert_eq!(s.get_pixel(1, 1), Some(c));

        // out-of-bounds should be ignored and not panic
        s.set_pixel(-1, 0, c);
        s.set_pixel(0, -1, c);
        s.set_pixel(2, 0, c);
        s.set_pixel(0, 2, c);
        // untouched pixel remains default (0)
        assert_eq!(s.get_pixel(0, 0), Some(Color::from_u32(0)));
    }
}
