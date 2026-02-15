use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Stopwatch {
    start: Option<Instant>,
    elapsed: Duration,
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self {
            start: None,
            elapsed: Duration::from_secs(0),
        }
    }
}

impl Stopwatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self) {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.start.take() {
            self.elapsed = self.elapsed.saturating_add(s.elapsed());
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = Duration::from_secs(0);
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }

    pub fn elapsed(&self) -> Duration {
        if let Some(s) = self.start {
            self.elapsed.saturating_add(s.elapsed())
        } else {
            self.elapsed
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::duration::secs_f32;
    use kmath::num::approx_eq_with;

    #[test]
    fn start_stop_accumulates() {
        let mut sw = Stopwatch::new();
        sw.start();
        std::thread::sleep(Duration::from_millis(5));
        sw.stop();
        let a = sw.elapsed();
        std::thread::sleep(Duration::from_millis(5));
        // not running -> elapsed should remain the same within small tolerance
        let a_same = sw.elapsed();
        assert!(approx_eq_with(secs_f32(a_same), secs_f32(a), 1e-3, 1e-3));
        sw.start();
        std::thread::sleep(Duration::from_millis(5));
        sw.stop();
        let b = sw.elapsed();
        // after running again -> should increase beyond tolerance
        assert!(!approx_eq_with(secs_f32(b), secs_f32(a_same), 1e-3, 1e-3));
    }
}
