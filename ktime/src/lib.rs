pub mod clock;
pub mod duration;
pub mod stopwatch;

pub use clock::{Clock, FakeClock, SystemClock};
pub use duration::{from_secs_f32, from_secs_f64, secs_f32, secs_f64};
pub use stopwatch::Stopwatch;
