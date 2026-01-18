//! Numeric utilities: EPS, approximate comparisons, safe ops.

/// Default epsilon for `f32` comparisons.
pub const EPS: f32 = 1e-6;

/// Returns true if `a` and `b` are approximately equal within absolute or relative tolerance.
pub fn approx_eq(a: f32, b: f32) -> bool {
    approx_eq_with(a, b, EPS, EPS)
}

/// Approximate equality with custom absolute and relative tolerances.
pub fn approx_eq_with(a: f32, b: f32, abs_tol: f32, rel_tol: f32) -> bool {
    let diff = (a - b).abs();
    if diff <= abs_tol {
        return true;
    }
    // Use max magnitude to scale relative tolerance.
    let max_ab = a.abs().max(b.abs());
    diff <= rel_tol * max_ab
}

/// Returns true if `x` is approximately zero.
pub fn is_zero(x: f32) -> bool {
    x.abs() <= EPS
}

/// Safe divide: returns `None` if denominator is approximately zero.
pub fn safe_div(num: f32, den: f32) -> Option<f32> {
    if is_zero(den) { None } else { Some(num / den) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approx_eq_basic() {
        assert!(approx_eq(1.0, 1.0 + 1e-7));
        assert!(!approx_eq(1.0, 1.0 + 1e-3));
    }

    #[test]
    fn approx_eq_with_relative() {
        assert!(approx_eq_with(1000.0, 1000.001, 1e-6, 1e-6));
        assert!(!approx_eq_with(1000.0, 1001.0, 1e-6, 1e-6));
    }

    #[test]
    fn is_zero_approx() {
        assert!(is_zero(1e-7));
        assert!(!is_zero(1e-4));
    }

    #[test]
    fn safe_div_behavior() {
        assert_eq!(safe_div(1.0, 0.0), None);
        assert_eq!(safe_div(1.0, 2.0), Some(0.5));
    }
}
