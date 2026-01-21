//! Drawing helpers. More primitives will be added incrementally.

use crate::core::{Color, Surface};

/// Clear the surface to a color (wrapper around `Surface::clear`).
pub fn clear(surface: &mut Surface, color: Color) {
    surface.clear(color);
}

/// Set a single pixel with clipping.
pub fn set_pixel(surface: &mut Surface, x: i32, y: i32, color: Color) {
    surface.set_pixel(x, y, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrappers_delegate_to_surface() {
        let mut s = Surface::new(2, 2);
        let c = Color::rgba(1, 2, 3, 255);
        clear(&mut s, c);
        for &px in s.pixels() {
            assert_eq!(px, c.to_u32());
        }
        let d = Color::rgba(4, 5, 6, 255);
        set_pixel(&mut s, 1, 1, d);
        assert_eq!(s.get_pixel(1, 1), Some(d));
    }
}
