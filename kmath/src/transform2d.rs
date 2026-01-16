use core::ops::Mul;

use crate::{Mat3, Vec2};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform2D {
    pub translation: Vec2,
    pub rotation: f32, // radians
    pub scale: Vec2,
}

impl Transform2D {
    pub fn identity() -> Self {
        Self {
            translation: Vec2::ZERO,
            rotation: 0.0,
            scale: Vec2::ONE,
        }
    }

    pub fn new(translation: Vec2, rotation: f32, scale: Vec2) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(t: Vec2) -> Self {
        Self {
            translation: t,
            ..Self::identity()
        }
    }

    pub fn from_rotation(r: f32) -> Self {
        Self {
            rotation: r,
            ..Self::identity()
        }
    }

    pub fn from_scale(s: Vec2) -> Self {
        Self {
            scale: s,
            ..Self::identity()
        }
    }

    pub fn to_mat3(&self) -> Mat3 {
        Mat3::translation(self.translation)
            * Mat3::rotation(self.rotation)
            * Mat3::scale(self.scale)
    }

    pub fn transform_point(&self, p: Vec2) -> Vec2 {
        self.to_mat3() * p
    }

    /// Compose two transforms and return the combined matrix (exact).
    /// Applies `other` first, then `self` (self ∘ other).
    pub fn compose_mat3(&self, other: &Transform2D) -> Mat3 {
        self.to_mat3() * other.to_mat3()
    }
}

/// Operator form of composition producing a Mat3: `a * b` equals `a.compose_mat3(&b)`.
impl Mul<Transform2D> for Transform2D {
    type Output = Mat3;
    fn mul(self, rhs: Transform2D) -> Self::Output {
        self.to_mat3() * rhs.to_mat3()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mat3::Mat3;
    use crate::num::approx_eq_with;

    // Follow API order
    #[test]
    fn identity_behaves_like_noop() {
        let id = Transform2D::identity();
        let p = Vec2::new(1.0, -2.0);
        let r = id.transform_point(p);
        assert!(approx_eq_with(r.x, p.x, 1e-6, 1e-6));
        assert!(approx_eq_with(r.y, p.y, 1e-6, 1e-6));
        assert_eq!(id.to_mat3(), Mat3::identity());
    }

    #[test]
    fn constructors_set_fields() {
        let t = Transform2D::from_translation(Vec2::new(2.0, 3.0));
        assert!(approx_eq_with(t.translation.x, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(t.translation.y, 3.0, 1e-6, 1e-6));
        let r = Transform2D::from_rotation(core::f32::consts::FRAC_PI_2);
        assert!(approx_eq_with(
            r.rotation,
            core::f32::consts::FRAC_PI_2,
            1e-6,
            1e-6
        ));
        let s = Transform2D::from_scale(Vec2::new(2.0, 3.0));
        assert!(approx_eq_with(s.scale.x, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(s.scale.y, 3.0, 1e-6, 1e-6));
    }

    #[test]
    fn to_mat3_matches_trs_product() {
        let tr = Transform2D::new(Vec2::new(5.0, -1.0), 0.25, Vec2::new(2.0, 3.0));
        let m = tr.to_mat3();
        let expected =
            Mat3::translation(tr.translation) * Mat3::rotation(tr.rotation) * Mat3::scale(tr.scale);
        assert_eq!(m, expected);
    }

    #[test]
    fn transform_point_equals_mat_mul() {
        let tr = Transform2D::new(Vec2::new(1.0, 2.0), 0.3, Vec2::new(2.0, 2.0));
        let p = Vec2::new(3.0, -4.0);
        let a = tr.transform_point(p);
        let b = tr.to_mat3() * p;
        assert!(approx_eq_with(a.x, b.x, 1e-6, 1e-6));
        assert!(approx_eq_with(a.y, b.y, 1e-6, 1e-6));
    }

    #[test]
    fn compose_mat3_equivalence() {
        let a = Transform2D::new(Vec2::new(5.0, -1.0), 0.2, Vec2::new(2.0, 3.0));
        let b = Transform2D::new(Vec2::new(-3.0, 4.0), -0.5, Vec2::new(0.5, 1.5));
        let c = a.compose_mat3(&b);
        let expected = a.to_mat3() * b.to_mat3();
        assert_eq!(c, expected);
    }

    #[test]
    fn mul_operator_equivalence_to_compose() {
        let a = Transform2D::new(Vec2::new(1.0, 2.0), 0.3, Vec2::new(2.0, 1.5));
        let b = Transform2D::new(Vec2::new(-2.0, 0.5), -0.2, Vec2::new(0.5, 3.0));
        let m_via_op = a * b; // Mat3
        let m_via_compose = a.compose_mat3(&b);
        assert_eq!(m_via_op, m_via_compose);
    }

    // Integration tests (placed last)
    #[test]
    fn integration_trs_point_application() {
        let s = Transform2D::from_scale(Vec2::new(2.0, 3.0));
        let r = Transform2D::from_rotation(core::f32::consts::FRAC_PI_2);
        let t = Transform2D::from_translation(Vec2::new(5.0, -1.0));

        let trs_mat = t.to_mat3() * r.to_mat3() * s.to_mat3();
        let p = Vec2::new(1.0, 2.0);
        let via_mat = trs_mat * p;

        let via_steps = t.transform_point(r.transform_point(s.transform_point(p)));
        assert!(approx_eq_with(via_mat.x, via_steps.x, 1e-6, 1e-6));
        assert!(approx_eq_with(via_mat.y, via_steps.y, 1e-6, 1e-6));
    }

    #[test]
    fn integration_mul_operator_with_chain() {
        let s = Transform2D::from_scale(Vec2::new(0.5, 2.0));
        let r = Transform2D::from_rotation(0.4);
        let t = Transform2D::from_translation(Vec2::new(-3.0, 1.0));

        // Using operator * (returns Mat3): (t * r) is Mat3, multiply with s.to_mat3()
        let chained = (t * r) * s.to_mat3();
        let p = Vec2::new(2.0, -1.0);
        let via_op = chained * p;

        // Equivalent via stepwise application
        let via_steps = t.transform_point(r.transform_point(s.transform_point(p)));
        assert!(approx_eq_with(via_op.x, via_steps.x, 1e-6, 1e-6));
        assert!(approx_eq_with(via_op.y, via_steps.y, 1e-6, 1e-6));
    }
}
