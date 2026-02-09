use kmath::Vec2;

/// Edge function E(a,b,p) = cross(b-a, p-a) for 2D vectors.
#[inline]
pub fn edge_function(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    let ab = Vec2::new(b.x - a.x, b.y - a.y);
    let ap = Vec2::new(p.x - a.x, p.y - a.y);
    ab.x * ap.y - ab.y * ap.x
}

/// Triangle signed area (double area) via edge function: E(v0, v1, v2).
#[inline]
pub fn signed_area(v0: Vec2, v1: Vec2, v2: Vec2) -> f32 {
    edge_function(v0, v1, v2)
}

/// Compute integer bounding box [min_x..=max_x], [min_y..=max_y], clamped to the target size.
/// Uses floor(min) and ceil(max)-1 as bounds for center sampling at (x+0.5, y+0.5).
pub fn bbox_clamped(
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    width: usize,
    height: usize,
) -> (i32, i32, i32, i32) {
    let min_x = v0.x.min(v1.x).min(v2.x).floor() as i32;
    let min_y = v0.y.min(v1.y).min(v2.y).floor() as i32;
    let max_x = v0.x.max(v1.x).max(v2.x).ceil() as i32 - 1;
    let max_y = v0.y.max(v1.y).max(v2.y).ceil() as i32 - 1;

    let wmax = (width as i32) - 1;
    let hmax = (height as i32) - 1;
    (
        min_x.clamp(0, wmax),
        min_y.clamp(0, hmax),
        max_x.clamp(0, wmax),
        max_y.clamp(0, hmax),
    )
}

/// Top-Left rule classification for an edge a->b in screen space (y-down).
/// Returns true if the edge is a top or left edge: dy < 0, or dy == 0 and dx > 0.
#[inline]
pub fn is_top_left(a: Vec2, b: Vec2) -> bool {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    dy < 0.0 || (dy == 0.0 && dx > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kmath::num::{approx_eq, approx_eq_with};

    #[test]
    fn edge_function_basic_orientation_y_down() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(1.0, 0.0);
        let p_above = Vec2::new(0.0, -1.0); // y-up is negative in y-down screen
        let p_below = Vec2::new(0.0, 1.0);
        assert!(edge_function(a, b, p_above) < 0.0);
        assert!(edge_function(a, b, p_below) > 0.0);
    }

    #[test]
    fn signed_area_matches_edge_function() {
        let v0 = Vec2::new(10.0, 10.0);
        let v1 = Vec2::new(20.0, 10.0);
        let v2 = Vec2::new(10.0, 20.0);
        assert!(approx_eq(
            signed_area(v0, v1, v2),
            edge_function(v0, v1, v2)
        ));
        // Degenerate
        let v2 = Vec2::new(15.0, 10.0);
        assert!(approx_eq_with(signed_area(v0, v1, v2), 0.0, 1e-6, 1e-6));
    }

    #[test]
    fn bbox_is_clamped_and_ordered() {
        let v0 = Vec2::new(-5.1, -1.0);
        let v1 = Vec2::new(3.2, 4.7);
        let v2 = Vec2::new(1000.0, 2.2);
        let (minx, miny, maxx, maxy) = bbox_clamped(v0, v1, v2, 10, 8);
        assert!(0 <= minx && minx <= maxx && maxx < 10);
        assert!(0 <= miny && miny <= maxy && maxy < 8);
    }

    #[test]
    fn topl_left_rule_y_down() {
        // Horizontal to the right -> top edge under y-down convention
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, 0.0);
        assert!(is_top_left(a, b));
        // Vertical down -> not top-left (dy>0)
        let b2 = Vec2::new(0.0, 10.0);
        assert!(!is_top_left(a, b2));
        // Vertical up -> left edge (dy<0)
        let b3 = Vec2::new(0.0, -10.0);
        assert!(is_top_left(a, b3));
    }
}
