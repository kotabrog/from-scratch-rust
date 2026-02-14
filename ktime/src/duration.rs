use std::time::Duration;

/// Convert `Duration` to seconds as `f64`.
pub fn secs_f64(d: Duration) -> f64 {
    d.as_secs_f64()
}

/// Convert `Duration` to seconds as `f32` (precision may be reduced).
pub fn secs_f32(d: Duration) -> f32 {
    d.as_secs_f64() as f32
}

/// Construct `Duration` from seconds as `f64`.
pub fn from_secs_f64(s: f64) -> Duration {
    if s <= 0.0 {
        Duration::from_secs(0)
    } else {
        Duration::try_from_secs_f64(s).unwrap_or(Duration::from_secs(u64::MAX))
    }
}

/// Construct `Duration` from seconds as `f32`.
pub fn from_secs_f32(s: f32) -> Duration {
    if s <= 0.0 {
        Duration::from_secs(0)
    } else {
        // as f64 to use try_from_secs_f64 for overflow safety
        from_secs_f64(s as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secs_roundtrip_f64() {
        use kmath::num::approx_eq_with;
        let d = from_secs_f64(0.5);
        let s = secs_f32(d);
        assert!(approx_eq_with(s, 0.5, 1e-6, 1e-6));
    }

    #[test]
    fn secs_roundtrip_f32_approx() {
        use kmath::num::approx_eq_with;
        let d = from_secs_f32(1.0 / 60.0);
        let s = secs_f32(d);
        assert!(approx_eq_with(s, (1.0 / 60.0) as f32, 1e-5, 1e-5));
    }
}
