use std::time::Duration;

use ktime::from_secs_f64;

#[derive(Copy, Clone, Debug)]
pub struct LoopConfig {
    pub fixed_hz: u32,
    pub fixed_dt: Duration,
    pub max_frame_dt: Duration,
    pub max_updates_per_frame: u32,
}

impl LoopConfig {
    pub fn from_hz(fixed_hz: u32) -> Self {
        let hz = if fixed_hz == 0 { 60 } else { fixed_hz };
        let fixed_dt = from_secs_f64(1.0 / (hz as f64));
        Self {
            fixed_hz: hz,
            fixed_dt,
            max_frame_dt: Duration::from_millis(250),
            max_updates_per_frame: 10,
        }
    }

    pub fn with_limits(mut self, max_frame_dt: Duration, max_updates_per_frame: u32) -> Self {
        self.max_frame_dt = max_frame_dt;
        self.max_updates_per_frame = max_updates_per_frame;
        self
    }
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self::from_hz(60)
    }
}
