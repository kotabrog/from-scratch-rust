pub mod splitmix64;

// Re-export for convenience during development
pub use splitmix64::SplitMix64;

mod krand;
mod pcg32;

pub use krand::Krand;
