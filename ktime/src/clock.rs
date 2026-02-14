use std::time::{Duration, Instant};

/// Time source abstraction.
pub trait Clock {
    fn now(&self) -> Instant;
}

/// System clock using `std::time::Instant::now`.
#[derive(Copy, Clone, Debug, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

/// Deterministic, manually advanceable clock for tests and headless runs.
#[derive(Clone, Debug)]
pub struct FakeClock {
    base: Instant,
    offset: Duration,
}

impl FakeClock {
    /// Create a new FakeClock anchored at current `Instant` with zero offset.
    pub fn new() -> Self {
        Self {
            base: Instant::now(),
            offset: Duration::from_secs(0),
        }
    }

    /// Create with a specified base Instant.
    pub fn with_base(base: Instant) -> Self {
        Self {
            base,
            offset: Duration::from_secs(0),
        }
    }

    /// Advance internal time by `dt`.
    pub fn advance(&mut self, dt: Duration) {
        self.offset = self.offset.saturating_add(dt);
    }
}

impl Default for FakeClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for FakeClock {
    fn now(&self) -> Instant {
        self.base + self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fake_clock_advances() {
        let mut c = FakeClock::default();
        let t0 = c.now();
        c.advance(Duration::from_millis(10));
        let t1 = c.now();
        assert!(t1 > t0);
        assert!(t1.duration_since(t0) >= Duration::from_millis(10));
    }
}
