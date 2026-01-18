use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::num::safe_div;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns Some(normalized) or None if nearly zero length.
    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        let inv = safe_div(1.0, len)?;
        Some(self * inv)
    }

    /// Panics if vector is nearly zero.
    pub fn normalize(self) -> Self {
        self.try_normalize().expect("normalize on near-zero Vec2")
    }

    /// Normalizes or returns zero when near-zero.
    pub fn normalize_or_zero(self) -> Self {
        self.try_normalize().unwrap_or(Self::ZERO)
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }

    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
impl Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Self = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn try_normalize(self) -> Option<Self> {
        let len = self.length();
        let inv = safe_div(1.0, len)?;
        Some(self * inv)
    }

    pub fn normalize(self) -> Self {
        self.try_normalize().expect("normalize on near-zero Vec3")
    }
    pub fn normalize_or_zero(self) -> Self {
        self.try_normalize().unwrap_or(Self::ZERO)
    }

    pub fn distance(self, rhs: Self) -> f32 {
        (self - rhs).length()
    }
    pub fn distance_squared(self, rhs: Self) -> f32 {
        (self - rhs).length_squared()
    }
    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        self * (1.0 - t) + rhs * t
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::{approx_eq, approx_eq_with};

    // Vec2: follow implementation order
    #[test]
    fn vec2_new_components() {
        let v = Vec2::new(1.2, -3.4);
        assert!(approx_eq_with(v.x, 1.2, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, -3.4, 1e-6, 1e-6));
    }

    #[test]
    fn vec2_dot_symmetry() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(-3.0, 4.0);
        assert!(approx_eq(a.dot(b), b.dot(a)));
    }

    #[test]
    fn vec2_length_non_negative() {
        let v = Vec2::new(-3.0, 4.0);
        assert!(v.length() >= 0.0);
    }

    #[test]
    fn vec2_length_sq_equals_dot() {
        let v = Vec2::new(2.0, -5.0);
        assert!(approx_eq(v.length_squared(), v.dot(v)));
    }

    #[test]
    fn vec2_try_normalize_none_on_zero() {
        let v = Vec2::new(0.0, 0.0);
        assert!(v.try_normalize().is_none());
        let tiny = Vec2::new(1e-9, 0.0);
        assert!(tiny.try_normalize().is_none());
    }

    #[test]
    #[should_panic]
    fn vec2_normalize_panics_on_zero() {
        let _ = Vec2::new(0.0, 0.0).normalize();
    }

    #[test]
    fn vec2_normalize_or_zero_returns_zero() {
        let v = Vec2::new(0.0, 0.0).normalize_or_zero();
        assert!(approx_eq_with(v.x, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, 0.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec2_distance_and_sq() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(4.0, 6.0);
        assert!(approx_eq_with(a.distance(b), 5.0, 1e-6, 1e-6));
        assert!(approx_eq_with(a.distance_squared(b), 25.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec2_lerp_works() {
        let a = Vec2::new(0.0, 0.0);
        let b = Vec2::new(10.0, -10.0);
        let t0 = a.lerp(b, 0.0);
        let t1 = a.lerp(b, 1.0);
        let th = a.lerp(b, 0.5);
        assert!(approx_eq_with(t0.x, 0.0, 1e-6, 1e-6) && approx_eq_with(t0.y, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(t1.x, 10.0, 1e-6, 1e-6) && approx_eq_with(t1.y, -10.0, 1e-6, 1e-6));
        assert!(approx_eq_with(th.x, 5.0, 1e-6, 1e-6) && approx_eq_with(th.y, -5.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec2_scalar_ops() {
        let v = Vec2::new(2.0, -4.0);
        let m = v * 0.5;
        let d = v / 2.0;
        assert!(approx_eq_with(m.x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(m.y, -2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(d.x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(d.y, -2.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec2_assign_ops() {
        let mut v = Vec2::new(1.0, 2.0);
        v += Vec2::new(3.0, -1.0);
        v -= Vec2::new(1.0, 1.0);
        v *= 2.0;
        v /= 2.0;
        assert!(approx_eq_with(v.x, 3.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, 0.0, 1e-6, 1e-6));
    }

    // Vec3: follow implementation order
    #[test]
    fn vec3_new_components() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert!(approx_eq_with(v.x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, 2.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.z, 3.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec3_dot_symmetry() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 4.0);
        assert!(approx_eq(a.dot(b), b.dot(a)));
    }

    #[test]
    fn vec3_length_non_negative() {
        let v = Vec3::new(-3.0, 4.0, -5.0);
        assert!(v.length() >= 0.0);
    }

    #[test]
    fn vec3_length_sq_equals_dot() {
        let v = Vec3::new(2.0, -5.0, 1.0);
        assert!(approx_eq(v.length_squared(), v.dot(v)));
    }

    #[test]
    fn vec3_try_normalize_none_on_zero() {
        let v = Vec3::new(0.0, 0.0, 0.0);
        assert!(v.try_normalize().is_none());
        let tiny = Vec3::new(1e-9, 0.0, 0.0);
        assert!(tiny.try_normalize().is_none());
    }

    #[test]
    #[should_panic]
    fn vec3_normalize_panics_on_zero() {
        let _ = Vec3::new(0.0, 0.0, 0.0).normalize();
    }

    #[test]
    fn vec3_normalize_or_zero_returns_zero() {
        let v = Vec3::new(0.0, 0.0, 0.0).normalize_or_zero();
        assert!(approx_eq_with(v.x, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.z, 0.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec3_distance_and_sq() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 6.0, 3.0);
        assert!(approx_eq_with(a.distance(b), 5.0, 1e-6, 1e-6));
        assert!(approx_eq_with(a.distance_squared(b), 25.0, 1e-6, 1e-6));
    }

    #[test]
    fn vec3_lerp_works() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(10.0, -10.0, 5.0);
        let t0 = a.lerp(b, 0.0);
        let t1 = a.lerp(b, 1.0);
        let th = a.lerp(b, 0.5);
        assert!(
            approx_eq_with(t0.x, 0.0, 1e-6, 1e-6)
                && approx_eq_with(t0.y, 0.0, 1e-6, 1e-6)
                && approx_eq_with(t0.z, 0.0, 1e-6, 1e-6)
        );
        assert!(
            approx_eq_with(t1.x, 10.0, 1e-6, 1e-6)
                && approx_eq_with(t1.y, -10.0, 1e-6, 1e-6)
                && approx_eq_with(t1.z, 5.0, 1e-6, 1e-6)
        );
        assert!(
            approx_eq_with(th.x, 5.0, 1e-6, 1e-6)
                && approx_eq_with(th.y, -5.0, 1e-6, 1e-6)
                && approx_eq_with(th.z, 2.5, 1e-6, 1e-6)
        );
    }

    #[test]
    fn vec3_scalar_ops() {
        let v = Vec3::new(2.0, -4.0, 8.0);
        let m = v * 0.5;
        let d = v / 2.0;
        assert!(
            approx_eq_with(m.x, 1.0, 1e-6, 1e-6)
                && approx_eq_with(m.y, -2.0, 1e-6, 1e-6)
                && approx_eq_with(m.z, 4.0, 1e-6, 1e-6)
        );
        assert!(
            approx_eq_with(d.x, 1.0, 1e-6, 1e-6)
                && approx_eq_with(d.y, -2.0, 1e-6, 1e-6)
                && approx_eq_with(d.z, 4.0, 1e-6, 1e-6)
        );
    }

    #[test]
    fn vec3_assign_ops() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v += Vec3::new(1.0, -1.0, 0.0);
        v -= Vec3::new(1.0, 1.0, 1.0);
        v *= 2.0;
        v /= 2.0;
        assert!(approx_eq_with(v.x, 1.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.y, 0.0, 1e-6, 1e-6));
        assert!(approx_eq_with(v.z, 2.0, 1e-6, 1e-6));
    }

    // Integration tests (use multiple APIs), placed last
    #[test]
    fn integration_vec2_algebra_identity() {
        let v = Vec2::new(0.1, -2.3);
        let w = Vec2::new(3.4, -0.7);
        let res = v + w - w;
        assert!(approx_eq_with(res.x, v.x, 1e-6, 1e-6));
        assert!(approx_eq_with(res.y, v.y, 1e-6, 1e-6));
    }

    #[test]
    fn integration_vec3_props() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 4.0);
        assert!(approx_eq(a.dot(b), b.dot(a)));
        let n = a.normalize();
        assert!(approx_eq_with(n.length(), 1.0, 1e-5, 1e-5));
        let res = a + b - b;
        assert!(approx_eq_with(res.x, a.x, 1e-6, 1e-6));
        assert!(approx_eq_with(res.y, a.y, 1e-6, 1e-6));
        assert!(approx_eq_with(res.z, a.z, 1e-6, 1e-6));
        assert!(approx_eq(a.length_squared(), a.dot(a)));
    }
}
