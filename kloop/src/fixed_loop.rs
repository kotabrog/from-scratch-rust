use std::time::{Duration, Instant};

use ktime::Clock;

use crate::app::App;
use crate::config::LoopConfig;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TickResult {
    pub updates: u32,
    pub alpha: f32,
}

pub struct FixedLoop<C: Clock> {
    pub clock: C,
    pub last: Instant,
    pub accumulator: Duration,
    pub cfg: LoopConfig,
    pub frame_index: u64,
}

impl<C: Clock> FixedLoop<C> {
    pub fn new(clock: C, cfg: LoopConfig) -> Self {
        let now = clock.now();
        Self {
            clock,
            last: now,
            accumulator: Duration::from_secs(0),
            cfg,
            frame_index: 0,
        }
    }

    /// Advance one real-time tick using the internal clock, performing updates and one render.
    pub fn tick<A: App>(&mut self, app: &mut A) -> TickResult {
        let now = self.clock.now();
        let mut frame_dt = now.saturating_duration_since(self.last);
        self.last = now;

        if frame_dt > self.cfg.max_frame_dt {
            frame_dt = self.cfg.max_frame_dt;
        }
        self.accumulator = self.accumulator.saturating_add(frame_dt);

        let mut updates = 0u32;
        while self.accumulator >= self.cfg.fixed_dt && updates < self.cfg.max_updates_per_frame {
            app.update(self.cfg.fixed_dt);
            self.accumulator -= self.cfg.fixed_dt;
            updates += 1;
        }

        if updates >= self.cfg.max_updates_per_frame {
            // Drop any remaining accumulated time to avoid spiral-of-death.
            self.accumulator = Duration::from_secs(0);
        }

        let alpha = if self.cfg.fixed_dt.is_zero() {
            0.0
        } else {
            let num = self.accumulator.as_secs_f64();
            let den = self.cfg.fixed_dt.as_secs_f64();
            let a = (num / den) as f32;
            a.clamp(0.0, 0.999_999)
        };

        app.render(alpha);
        self.frame_index += 1;

        TickResult { updates, alpha }
    }

    /// Run exactly `n` updates with the fixed timestep (headless stepping). No rendering.
    pub fn run_steps<A: App>(&mut self, app: &mut A, n: u32) {
        for _ in 0..n {
            app.update(self.cfg.fixed_dt);
            self.frame_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LoopConfig;
    use ktime::FakeClock;

    struct CounterApp {
        pub updates: u32,
        pub renders: u32,
        pub last_alpha: f32,
    }
    impl CounterApp {
        fn new() -> Self {
            Self {
                updates: 0,
                renders: 0,
                last_alpha: 0.0,
            }
        }
    }
    impl App for CounterApp {
        fn update(&mut self, _dt: Duration) {
            self.updates += 1;
        }
        fn render(&mut self, alpha: f32) {
            self.renders += 1;
            self.last_alpha = alpha;
        }
    }

    #[test]
    fn one_second_produces_sixty_updates() {
        // allow full 1s accumulation by setting a large max_frame_dt
        let mut loopr = FixedLoop::new(
            FakeClock::default(),
            LoopConfig::from_hz(60).with_limits(Duration::from_secs(5), 1000),
        );
        let mut app = CounterApp::new();
        // Advance by 1 second in one go; one tick should perform 60 updates
        loopr.clock.advance(Duration::from_secs(1));
        let res = loopr.tick(&mut app);
        // Due to Duration rounding to integer nanoseconds, 1/60s may round up,
        // allowing only 59 updates to fit in exactly 1 second.
        assert!((59..=60).contains(&app.updates));
        assert!((59..=60).contains(&res.updates));
        assert!(res.alpha >= 0.0 && res.alpha < 1.0);
        assert_eq!(app.renders, 1);
    }

    #[test]
    fn alpha_in_range_and_limits_apply() {
        let mut clock = FakeClock::default();
        let cfg = LoopConfig::from_hz(60).with_limits(Duration::from_millis(50), 3);
        let mut loopr = FixedLoop::new(clock.clone(), cfg);
        let mut app = CounterApp::new();
        // Large frame jump -> clamped to 50ms and max 3 updates
        clock.advance(Duration::from_millis(500));
        loopr.clock = clock;
        let res = loopr.tick(&mut app);
        assert!(res.updates <= 3);
        assert!(res.alpha >= 0.0 && res.alpha < 1.0);
        assert_eq!(app.renders, 1);
    }

    #[test]
    fn run_steps_advances_exact_updates() {
        let clock = FakeClock::default();
        let mut loopr = FixedLoop::new(clock, LoopConfig::from_hz(120));
        let mut app = CounterApp::new();
        loopr.run_steps(&mut app, 5);
        assert_eq!(app.updates, 5);
        assert_eq!(app.renders, 0);
    }
}
