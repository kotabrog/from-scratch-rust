use kpix::{Color, Surface};

/// Framebuffer wrapper for kraster2d.
/// Phase A: color buffer only (depth is reserved for later phases).
#[derive(Clone, Debug)]
pub struct Frame {
    surface: Surface,
    #[allow(dead_code)]
    depth: Option<Vec<f32>>, // reserved for Z-buffer (Phase F)
}

impl Frame {
    /// Create a new frame with the given dimensions.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            surface: Surface::new(width, height),
            depth: None,
        }
    }

    /// Frame width in pixels.
    #[inline]
    pub fn width(&self) -> usize {
        self.surface.width()
    }

    /// Frame height in pixels.
    #[inline]
    pub fn height(&self) -> usize {
        self.surface.height()
    }

    /// Clear the color buffer.
    #[inline]
    pub fn clear(&mut self, color: Color) {
        self.surface.clear(color);
    }

    /// Set a pixel (clipped to bounds).
    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.surface.set_pixel(x, y, color);
    }

    /// Access the underlying packed RGBA little-endian buffer.
    #[inline]
    pub fn pixels(&self) -> &[u32] {
        self.surface.pixels()
    }

    /// Internal access to the surface (for IO helpers).
    #[inline]
    pub fn surface(&self) -> &Surface {
        &self.surface
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_and_clear_and_set_pixel() {
        let mut f = Frame::new(3, 2);
        assert_eq!(f.width(), 3);
        assert_eq!(f.height(), 2);

        let red = Color::rgba(255, 0, 0, 255);
        f.clear(red);
        for &px in f.pixels() {
            assert_eq!(px, red.to_u32());
        }

        let c = Color::rgba(1, 2, 3, 255);
        f.set_pixel(1, 1, c);
        // `Surface` exposes get_pixel, but here we check via raw pixels index
        let idx = f.width() + 1;
        assert_eq!(f.pixels()[idx], c.to_u32());
    }
}
