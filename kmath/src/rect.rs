use crate::vec::Vec2;

/// 2D axis-aligned bounding box with half-open bounds [min, max).
/// - min: inclusive
/// - max: exclusive
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
    min: Vec2,
    max: Vec2,
}

/// Alias for 2D AABB.
pub type Aabb2 = Rect;

impl Rect {
    /// Constructs a rect from arbitrary min/max; order is normalized internally.
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        let (minx, maxx) = if min.x <= max.x { (min.x, max.x) } else { (max.x, min.x) };
        let (miny, maxy) = if min.y <= max.y { (min.y, max.y) } else { (max.y, min.y) };
        Self { min: Vec2::new(minx, miny), max: Vec2::new(maxx, maxy) }
    }

    /// Constructs a rect from origin (min) and size; negative sizes are normalized.
    pub fn from_origin_size(origin: Vec2, size: Vec2) -> Self {
        let max = Vec2::new(origin.x + size.x, origin.y + size.y);
        Self::from_min_max(origin, max)
    }

    /// Inclusive minimum corner.
    pub fn min(&self) -> Vec2 { self.min }
    /// Exclusive maximum corner.
    pub fn max(&self) -> Vec2 { self.max }

    /// Origin equals min.
    pub fn origin(&self) -> Vec2 { self.min }

    /// Size vector (width, height). Zero for empty along an axis.
    pub fn size(&self) -> Vec2 { self.max - self.min }

    pub fn width(&self) -> f32 { self.max.x - self.min.x }
    pub fn height(&self) -> f32 { self.max.y - self.min.y }

    /// Geometric center ((min+max)/2). For empty edges, lies at the midpoint.
    pub fn center(&self) -> Vec2 { (self.min + self.max) * 0.5 }

    /// Returns true if the rect is empty in any dimension (non-positive extent).
    pub fn is_empty(&self) -> bool { self.width() <= 0.0 || self.height() <= 0.0 }

    /// Half-open contains: includes min edge, excludes max edge.
    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x < self.max.x && p.y >= self.min.y && p.y < self.max.y
    }

    /// Returns true if `other` is fully contained within this rect (half-open policy).
    pub fn contains_rect(&self, other: Rect) -> bool {
        other.min.x >= self.min.x && other.max.x <= self.max.x &&
        other.min.y >= self.min.y && other.max.y <= self.max.y
    }

    /// Half-open intersection: touching only at edges/corners is not considered intersection.
    pub fn intersects(&self, other: Rect) -> bool {
        let x_overlaps = self.min.x < other.max.x && other.min.x < self.max.x;
        let y_overlaps = self.min.y < other.max.y && other.min.y < self.max.y;
        x_overlaps && y_overlaps
    }

    /// Smallest rect containing both.
    pub fn union(&self, other: Rect) -> Rect {
        let min = Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y));
        let max = Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y));
        Rect { min, max }
    }

    /// Expands this rect to include a point.
    pub fn expand_to_include_point(&self, p: Vec2) -> Rect {
        let min = Vec2::new(self.min.x.min(p.x), self.min.y.min(p.y));
        let max = Vec2::new(self.max.x.max(p.x), self.max.y.max(p.y));
        Rect::from_min_max(min, max)
    }

    /// Expands this rect to include another rect.
    pub fn expand_to_include_rect(&self, r: Rect) -> Rect {
        let min = Vec2::new(self.min.x.min(r.min.x), self.min.y.min(r.min.y));
        let max = Vec2::new(self.max.x.max(r.max.x), self.max.y.max(r.max.y));
        Rect { min, max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::approx_eq_with;

    // Follow implementation order
    #[test]
    fn from_min_max_normalizes_order() {
        let r = Rect::from_min_max(Vec2::new(5.0, 2.0), Vec2::new(1.0, 10.0));
        assert!(approx_eq_with(r.min().x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.min().y, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.max().x, 5.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.max().y, 10.0, 1e-6, 1e-6));
    }

    #[test]
    fn from_origin_size_normalizes_negative_size() {
        let r = Rect::from_origin_size(Vec2::new(3.0, 4.0), Vec2::new(-2.0, 5.0));
        assert!(approx_eq_with(r.min().x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.min().y, 4.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.max().x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.max().y, 9.0, 1e-6, 1e-6));
    }

    #[test]
    fn getters_and_size() {
        let r = Rect::from_min_max(Vec2::new(1.0, 2.0), Vec2::new(4.0, 6.0));
        assert!(approx_eq_with(r.origin().x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.origin().y, 2.0, 1e-6, 1e-6));
        let s = r.size();
        assert!(approx_eq_with(s.x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(s.y, 4.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.width(), 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.height(), 4.0, 1e-6, 1e-6));
        let c = r.center();
        assert!(approx_eq_with(c.x, 2.5, 1e-6, 1e-6));
        assert!(approx_eq_with(c.y, 4.0, 1e-6, 1e-6));
    }

    #[test]
    fn empty_detection() {
        let r1 = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0));
        let r2 = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let r3 = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        assert!(r1.is_empty());
        assert!(r2.is_empty());
        assert!(!r3.is_empty());
    }

    #[test]
    fn contains_point_half_open() {
        let r = Rect::from_min_max(Vec2::new(1.0, 2.0), Vec2::new(4.0, 6.0));
        // inside
        assert!(r.contains_point(Vec2::new(1.0, 2.0))); // includes min
        assert!(r.contains_point(Vec2::new(3.999999, 5.999999)));
        // edges
        assert!(r.contains_point(Vec2::new(1.0, 3.0))); // min x edge
        assert!(!r.contains_point(Vec2::new(4.0, 3.0))); // excludes max x
        assert!(!r.contains_point(Vec2::new(3.0, 6.0))); // excludes max y
        // outside
        assert!(!r.contains_point(Vec2::new(0.0, 0.0)));
    }

    #[test]
    fn contains_rect_half_open() {
        let outer = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let inner = Rect::from_min_max(Vec2::new(2.0, 2.0), Vec2::new(5.0, 5.0));
        let same = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let touching = Rect::from_min_max(Vec2::new(9.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(outer.contains_rect(inner));
        assert!(outer.contains_rect(same));
        assert!(outer.contains_rect(touching)); // contained even if touching max edge internally
    }

    #[test]
    fn contains_rect_half_open_false_cases() {
        let outer = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        // Exceeds outer on max side
        let exceed_max = Rect::from_min_max(Vec2::new(9.0, 0.0), Vec2::new(11.0, 10.0));
        assert!(!outer.contains_rect(exceed_max));
        // Starts before outer on min side
        let before_min = Rect::from_min_max(Vec2::new(-1.0, 2.0), Vec2::new(5.0, 5.0));
        assert!(!outer.contains_rect(before_min));
        // Exceeds on y only
        let exceed_y = Rect::from_min_max(Vec2::new(2.0, 2.0), Vec2::new(5.0, 12.0));
        assert!(!outer.contains_rect(exceed_y));
    }

    #[test]
    fn intersects_half_open() {
        let a = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let b = Rect::from_min_max(Vec2::new(1.0, 1.0), Vec2::new(3.0, 3.0));
        let c = Rect::from_min_max(Vec2::new(2.0, 0.0), Vec2::new(4.0, 2.0)); // touches at edge
        let d = Rect::from_min_max(Vec2::new(2.0, 2.0), Vec2::new(4.0, 4.0)); // touches at corner
        assert!(a.intersects(b));
        assert!(!a.intersects(c));
        assert!(!a.intersects(d));
    }

    #[test]
    fn union_and_expand() {
        let a = Rect::from_min_max(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
        let b = Rect::from_min_max(Vec2::new(1.0, -1.0), Vec2::new(3.0, 1.0));
        let u = a.union(b);
        assert!(approx_eq_with(u.min().x, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(u.min().y, -1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(u.max().x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(u.max().y, 2.0, 1e-6, 1e-6));

        let p = Vec2::new(-1.0, 3.0);
        let e1 = a.expand_to_include_point(p);
        assert!(approx_eq_with(e1.min().x, -1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e1.min().y, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e1.max().x, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e1.max().y, 3.0, 1e-6, 1e-6));

        let e2 = a.expand_to_include_rect(b);
        assert!(approx_eq_with(e2.min().x, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e2.min().y, -1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e2.max().x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(e2.max().y, 2.0, 1e-6, 1e-6));
    }

    // Integration tests
    #[test]
    fn integration_expand_points_matches_union() {
        let points = [
            Vec2::new(1.0, 1.0),
            Vec2::new(2.0, -1.0),
            Vec2::new(-3.0, 4.0),
            Vec2::new(0.5, 0.5),
        ];
        // Start from a degenerate empty rect at the first point
        let mut acc = Rect::from_min_max(points[0], points[0]);
        for &p in &points[1..] {
            acc = acc.expand_to_include_point(p);
        }
        let mut u = Rect::from_min_max(points[0], points[0]);
        for &p in &points[1..] {
            u = u.union(Rect::from_min_max(p, p));
        }
        assert!(approx_eq_with(acc.min().x, u.min().x, 1e-6, 1e-6));
        assert!(approx_eq_with(acc.min().y, u.min().y, 1e-6, 1e-6));
        assert!(approx_eq_with(acc.max().x, u.max().x, 1e-6, 1e-6));
        assert!(approx_eq_with(acc.max().y, u.max().y, 1e-6, 1e-6));
    }
}
