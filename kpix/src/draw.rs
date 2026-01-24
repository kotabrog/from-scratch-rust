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

/// Draw a line from (x0,y0) to (x1,y1) using Bresenham's integer algorithm.
/// Endpoints are included. Clipping is delegated to `Surface::set_pixel`.
pub fn draw_line(surface: &mut Surface, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy; // error term

    loop {
        surface.set_pixel(x0, y0, color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Draw rectangle outline. `(x, y)` is a corner; `w`, `h` may be negative.
/// Uses half-open semantics: draws the border of [x0, x1) x [y0, y1).
pub fn draw_rect(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: Color) {
    if w == 0 || h == 0 {
        return;
    }
    let (x0, x1) = if w >= 0 { (x, x + w) } else { (x + w, x) };
    let (y0, y1) = if h >= 0 { (y, y + h) } else { (y + h, y) };
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    // top and bottom (inclusive endpoints)
    draw_line(surface, x0, y0, x1 - 1, y0, color);
    draw_line(surface, x0, y1 - 1, x1 - 1, y1 - 1, color);
    // left and right
    draw_line(surface, x0, y0, x0, y1 - 1, color);
    draw_line(surface, x1 - 1, y0, x1 - 1, y1 - 1, color);
}

/// Fill rectangle area. `(x, y)` is a corner; `w`, `h` may be negative.
/// Fills all pixels within half-open region [x0, x1) x [y0, y1), with clipping.
pub fn fill_rect(surface: &mut Surface, x: i32, y: i32, w: i32, h: i32, color: Color) {
    if w == 0 || h == 0 {
        return;
    }
    let (x0, x1) = if w >= 0 { (x, x + w) } else { (x + w, x) };
    let (y0, y1) = if h >= 0 { (y, y + h) } else { (y + h, y) };
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    for yy in y0..y1 {
        for xx in x0..x1 {
            surface.set_pixel(xx, yy, color);
        }
    }
}

/// Draw a circle outline centered at (cx, cy) with integer radius `r` (r >= 0).
/// Uses the Midpoint Circle Algorithm. Clipping is delegated to `set_pixel`.
pub fn draw_circle(surface: &mut Surface, cx: i32, cy: i32, r: i32, color: Color) {
    if r < 0 {
        return;
    }
    if r == 0 {
        surface.set_pixel(cx, cy, color);
        return;
    }

    let mut x = r;
    let mut y = 0;
    let mut d = 1 - r; // decision parameter

    while y <= x {
        // 8-way symmetry
        surface.set_pixel(cx + x, cy + y, color);
        surface.set_pixel(cx - x, cy + y, color);
        surface.set_pixel(cx + x, cy - y, color);
        surface.set_pixel(cx - x, cy - y, color);
        surface.set_pixel(cx + y, cy + x, color);
        surface.set_pixel(cx - y, cy + x, color);
        surface.set_pixel(cx + y, cy - x, color);
        surface.set_pixel(cx - y, cy - x, color);

        y += 1;
        if d <= 0 {
            d += 2 * y + 1;
        } else {
            x -= 1;
            d += 2 * (y - x) + 1;
        }
    }
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

    #[test]
    fn line_horizontal() {
        let mut s = Surface::new(5, 3);
        let c = Color::rgba(255, 0, 0, 255);
        super::draw_line(&mut s, 0, 1, 4, 1, c);
        // y=1, x=0..4 must be set
        for x in 0..5 {
            assert_eq!(s.get_pixel(x, 1), Some(c));
        }
        // other rows remain default (0)
        for y in [0, 2] {
            for x in 0..5 {
                assert_eq!(s.get_pixel(x, y), Some(Color::from_u32(0)));
            }
        }
    }

    #[test]
    fn line_vertical() {
        let mut s = Surface::new(3, 5);
        let c = Color::rgba(0, 255, 0, 255);
        super::draw_line(&mut s, 1, 0, 1, 4, c);
        for y in 0..5 {
            assert_eq!(s.get_pixel(1, y), Some(c));
        }
    }

    #[test]
    fn line_diagonal_positive_slope() {
        let mut s = Surface::new(5, 5);
        let c = Color::rgba(0, 0, 255, 255);
        super::draw_line(&mut s, 0, 0, 4, 4, c);
        for i in 0..5 {
            assert_eq!(s.get_pixel(i, i), Some(c));
        }
    }

    #[test]
    fn line_diagonal_negative_slope() {
        let mut s = Surface::new(5, 5);
        let c = Color::rgba(200, 200, 0, 255);
        super::draw_line(&mut s, 0, 4, 4, 0, c);
        for i in 0..5 {
            assert_eq!(s.get_pixel(i, 4 - i), Some(c));
        }
    }

    #[test]
    fn line_forward_backward_equivalence() {
        let mut a = Surface::new(8, 8);
        let mut b = Surface::new(8, 8);
        let c = Color::rgba(123, 45, 67, 255);
        super::draw_line(&mut a, 1, 2, 7, 6, c);
        super::draw_line(&mut b, 7, 6, 1, 2, c);

        assert_eq!(a.pixels(), b.pixels());
    }

    #[test]
    fn line_out_of_bounds_clipped_by_set_pixel() {
        let mut s = Surface::new(5, 3);
        let c = Color::rgba(255, 0, 255, 255);
        // span beyond left/right; should draw across visible row y=1
        super::draw_line(&mut s, -2, 1, 6, 1, c);
        for x in 0..5 {
            assert_eq!(s.get_pixel(x, 1), Some(c));
        }
    }

    #[test]
    fn rect_outline_basic() {
        let mut s = Surface::new(5, 5);
        let c = Color::rgba(10, 20, 30, 255);
        super::draw_rect(&mut s, 1, 1, 3, 3, c); // covers x=[1,3], y=[1,3] edges
        // corners
        for (x, y) in [(1, 1), (3, 1), (1, 3), (3, 3)] {
            assert_eq!(s.get_pixel(x, y), Some(c));
        }
        // inside remains default
        assert_eq!(s.get_pixel(2, 2), Some(Color::from_u32(0)));
    }

    #[test]
    fn rect_fill_basic_and_area() {
        let mut s = Surface::new(6, 4);
        let c = Color::rgba(200, 50, 50, 255);
        super::fill_rect(&mut s, 1, 1, 3, 2, c); // x:[1,4), y:[1,3)
        // count filled pixels
        let mut cnt = 0;
        for y in 0..4 {
            for x in 0..6 {
                if s.get_pixel(x, y) == Some(c) {
                    cnt += 1;
                }
            }
        }
        assert_eq!(cnt, 3 * 2);
    }

    #[test]
    fn rect_negative_size_normalizes() {
        let mut a = Surface::new(6, 4);
        let mut b = Surface::new(6, 4);
        let c = Color::rgba(0, 200, 0, 255);
        super::fill_rect(&mut a, 4, 3, -3, -2, c);
        super::fill_rect(&mut b, 1, 1, 3, 2, c);
        assert_eq!(a.pixels(), b.pixels());
    }

    #[test]
    fn rect_clip_out_of_bounds() {
        let mut s = Surface::new(4, 3);
        let c = Color::rgba(50, 50, 200, 255);
        super::fill_rect(&mut s, -2, -1, 6, 4, c); // spans beyond all sides; half-open covers full surface
        // Visible region should be filled entirely
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(s.get_pixel(x, y), Some(c));
            }
        }
    }

    #[test]
    fn circle_radius_zero_sets_center() {
        let mut s = Surface::new(5, 5);
        let c = Color::rgba(100, 100, 100, 255);
        super::draw_circle(&mut s, 2, 2, 0, c);
        assert_eq!(s.get_pixel(2, 2), Some(c));
    }

    #[test]
    fn circle_cardinals_for_small_radius() {
        let mut s = Surface::new(7, 7);
        let c = Color::rgba(10, 200, 10, 255);
        super::draw_circle(&mut s, 3, 3, 2, c);
        // Cardinal points at distance r should be drawn
        assert_eq!(s.get_pixel(3 + 2, 3), Some(c));
        assert_eq!(s.get_pixel(3 - 2, 3), Some(c));
        assert_eq!(s.get_pixel(3, 3 + 2), Some(c));
        assert_eq!(s.get_pixel(3, 3 - 2), Some(c));
    }

    #[test]
    fn circle_symmetry_and_clipping() {
        let mut s = Surface::new(5, 5);
        let c = Color::rgba(200, 10, 10, 255);
        // Partially out-of-bounds: should not panic and visible points should appear
        super::draw_circle(&mut s, 0, 0, 3, c);
        // Expect some visible pixels near the top-left corner
        assert_eq!(s.get_pixel(0, 3), Some(c));
        assert_eq!(s.get_pixel(3, 0), Some(c));
    }
}
