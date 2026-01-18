use core::ops::Mul;

use crate::num::safe_div;
use crate::vec::{Vec2, Vec3};

/// 3x3 matrix for 2D affine transforms (column vectors, column-major storage).
/// Applies to homogeneous column vectors `[x, y, 1]` via `Mat * Vec`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    // Column-major: m[col*3 + row]
    m: [f32; 9],
}

impl Mat3 {
    #[inline]
    fn idx(row: usize, col: usize) -> usize {
        col * 3 + row
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> f32 {
        self.m[Self::idx(row, col)]
    }

    #[inline]
    fn set(&mut self, row: usize, col: usize, v: f32) {
        self.m[Self::idx(row, col)] = v;
    }

    pub fn identity() -> Self {
        Self {
            m: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }

    /// Builds a matrix from columns (column-major).
    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) -> Self {
        Self {
            m: [c0.x, c0.y, c0.z, c1.x, c1.y, c1.z, c2.x, c2.y, c2.z],
        }
    }

    /// Translation by `t`.
    pub fn translation(t: Vec2) -> Self {
        // Columns: [1,0,0], [0,1,0], [tx,ty,1]
        Self::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(t.x, t.y, 1.0),
        )
    }

    /// Rotation around Z by radians (right-handed, +Y up).
    pub fn rotation(rad: f32) -> Self {
        let (s, c) = rad.sin_cos();
        // Rows: [ c -s 0; s c 0; 0 0 1 ]
        // Columns: [c,s,0], [-s,c,0], [0,0,1]
        Self::from_cols(
            Vec3::new(c, s, 0.0),
            Vec3::new(-s, c, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }

    /// Non-uniform scale.
    pub fn scale(s: Vec2) -> Self {
        // diag(sx, sy, 1)
        Self::from_cols(
            Vec3::new(s.x, 0.0, 0.0),
            Vec3::new(0.0, s.y, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        )
    }

    /// Transforms a point `[x, y]` in homogeneous coordinates (w=1).
    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        // x' = m00*x + m01*y + m02
        // y' = m10*x + m11*y + m12
        // w' = m20*x + m21*y + m22
        let x = self.get(0, 0) * p.x + self.get(0, 1) * p.y + self.get(0, 2);
        let y = self.get(1, 0) * p.x + self.get(1, 1) * p.y + self.get(1, 2);
        let w = self.get(2, 0) * p.x + self.get(2, 1) * p.y + self.get(2, 2);
        if let Some(inv) = safe_div(1.0, w) {
            Vec2::new(x * inv, y * inv)
        } else {
            // If w is ~0 (shouldn't happen for pure affine), return as-is.
            Vec2::new(x, y)
        }
    }
}

impl Mul for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self { m: [0.0; 9] };
        for col in 0..3 {
            for row in 0..3 {
                let v = self.get(row, 0) * rhs.get(0, col)
                    + self.get(row, 1) * rhs.get(1, col)
                    + self.get(row, 2) * rhs.get(2, col);
                out.set(row, col, v);
            }
        }
        out
    }
}

impl Mul<Vec2> for Mat3 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        self.transform_point(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{approx_eq, approx_eq_with};

    // Follow API order
    #[test]
    fn identity_properties() {
        let i = Mat3::identity();
        // I * p = p
        let p = Vec2::new(1.0, -2.0);
        let r = i * p;
        assert!(approx_eq(r.x, p.x) && approx_eq(r.y, p.y));
        // I * I = I
        let ii = i * i;
        assert_eq!(i, ii);
    }

    #[test]
    fn translation_works() {
        let t = Mat3::translation(Vec2::new(2.0, 3.0));
        let p = Vec2::new(1.0, 1.0);
        let r = t * p;
        assert!(approx_eq_with(r.x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.y, 4.0, 1e-6, 1e-6));
    }

    #[test]
    fn rotation_pi_over_2() {
        let r90 = Mat3::rotation(core::f32::consts::FRAC_PI_2);
        let p = Vec2::new(1.0, 0.0);
        let r = r90 * p;
        assert!(approx_eq_with(r.x, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.y, 1.0, 1e-6, 1e-6));
    }

    #[test]
    fn scale_works() {
        let s = Mat3::scale(Vec2::new(2.0, 3.0));
        let p = Vec2::new(1.0, -1.0);
        let r = s * p;
        assert!(approx_eq_with(r.x, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(r.y, -3.0, 1e-6, 1e-6));
    }

    #[test]
    fn mat_mul_rotation_composition() {
        let r90 = Mat3::rotation(core::f32::consts::FRAC_PI_2);
        let r180 = Mat3::rotation(core::f32::consts::PI);
        let composed = r90 * r90;
        let p = Vec2::new(1.0, 0.0);
        let c1 = composed * p;
        let c2 = r180 * p;
        assert!(approx_eq_with(c1.x, c2.x, 1e-6, 1e-6));
        assert!(approx_eq_with(c1.y, c2.y, 1e-6, 1e-6));
    }

    #[test]
    fn transform_point_equals_mul_vec2() {
        let m = Mat3::translation(Vec2::new(2.0, 3.0)) * Mat3::rotation(0.25);
        let p = Vec2::new(1.0, -2.0);
        let a = m.transform_point(p);
        let b = m * p;
        assert!(approx_eq_with(a.x, b.x, 1e-6, 1e-6));
        assert!(approx_eq_with(a.y, b.y, 1e-6, 1e-6));
    }

    // Integration: composition order T*R*S equals applying S->R->T
    #[test]
    fn integration_trs_order() {
        let s = Mat3::scale(Vec2::new(2.0, 3.0));
        let r = Mat3::rotation(core::f32::consts::FRAC_PI_2);
        let t = Mat3::translation(Vec2::new(5.0, -1.0));
        let trs = t * r * s; // apply S then R then T

        let p = Vec2::new(1.0, 2.0);
        let via_mat = trs * p;

        let via_steps = t * (r * (s * p));

        assert!(approx_eq_with(via_mat.x, via_steps.x, 1e-6, 1e-6));
        assert!(approx_eq_with(via_mat.y, via_steps.y, 1e-6, 1e-6));
    }
}
