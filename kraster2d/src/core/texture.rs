use kmath::Vec2;
use kpix::Color;

#[derive(Clone, Debug)]
pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<u32>, // packed RGBA little-endian
}

impl Texture {
    pub fn from_rgba_le(pixels: Vec<u32>, width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "Texture dimensions must be > 0");
        assert_eq!(pixels.len(), width.saturating_mul(height));
        Self {
            width,
            height,
            pixels,
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

    /// Nearest sampling with clamp. uv expected in [0,1]. Origin at top-left (v grows downward).
    pub fn sample_nearest(&self, uv: Vec2) -> Color {
        let u = uv.x.clamp(0.0, 1.0);
        let v = uv.y.clamp(0.0, 1.0);
        let w = self.width as f32;
        let h = self.height as f32;
        let txf = (u * (w - 1.0) + 0.5).floor().clamp(0.0, w - 1.0);
        let tyf = (v * (h - 1.0) + 0.5).floor().clamp(0.0, h - 1.0);
        let tx = txf as usize;
        let ty = tyf as usize;
        let idx = ty * self.width + tx;
        Color::from_u32(self.pixels[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_sampling_corners_and_clamp() {
        // 2x2: TL=red, TR=green, BL=blue, BR=white
        let px = vec![
            Color::rgba(255, 0, 0, 255).to_u32(),
            Color::rgba(0, 255, 0, 255).to_u32(),
            Color::rgba(0, 0, 255, 255).to_u32(),
            Color::rgba(255, 255, 255, 255).to_u32(),
        ];
        let tex = Texture::from_rgba_le(px, 2, 2);
        let tl = tex.sample_nearest(Vec2::new(0.0, 0.0));
        let tr = tex.sample_nearest(Vec2::new(1.0, 0.0));
        let bl = tex.sample_nearest(Vec2::new(0.0, 1.0));
        let br = tex.sample_nearest(Vec2::new(1.0, 1.0));
        assert_eq!(tl, Color::rgba(255, 0, 0, 255));
        assert_eq!(tr, Color::rgba(0, 255, 0, 255));
        assert_eq!(bl, Color::rgba(0, 0, 255, 255));
        assert_eq!(br, Color::rgba(255, 255, 255, 255));

        // Clamp outside
        let tl2 = tex.sample_nearest(Vec2::new(-1.0, -1.0));
        assert_eq!(tl2, Color::rgba(255, 0, 0, 255));
    }

    #[test]
    #[should_panic]
    fn zero_width_is_disallowed() {
        let _ = Texture::from_rgba_le(Vec::new(), 0, 1);
    }

    #[test]
    #[should_panic]
    fn zero_height_is_disallowed() {
        let _ = Texture::from_rgba_le(Vec::new(), 1, 0);
    }
}
