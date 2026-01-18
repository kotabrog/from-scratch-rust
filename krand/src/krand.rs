use core::ops::Range;

use crate::pcg32::Pcg32;
use crate::splitmix64::SplitMix64;

/// Minimal, non-cryptographic RNG with reproducible sequences.
#[derive(Clone, Debug)]
pub struct Krand {
    pcg: Pcg32,
}

impl Krand {
    /// Create a new generator from a single u64 seed.
    /// Uses SplitMix64 to derive PCG state + odd stream, then applies PCG seeding protocol.
    pub fn new(seed: u64) -> Self {
        Self::from_seed(seed)
    }

    pub fn from_seed(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        let initstate = sm.next_u64();
        let stream = sm.next_u64();
        let inc = (stream << 1) | 1; // force odd
        let pcg = Pcg32::seed(initstate, inc);
        Self { pcg }
    }

    /// Next u32.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.pcg.next_u32()
    }

    /// Next f32 in [0,1). Uses top 24 bits to avoid 1.0.
    #[inline]
    pub fn next_f32_0_1(&mut self) -> f32 {
        let u24 = self.next_u32() >> 8; // keep top 24 bits
        const SCALE: f32 = 1.0 / 16_777_216.0; // 2^24
        (u24 as f32) * SCALE
    }

    /// Uniformly sample from [range.start, range.end) using Lemire's method.
    /// Panics if start >= end.
    pub fn range_u32(&mut self, range: Range<u32>) -> u32 {
        let start = range.start;
        let end = range.end;
        assert!(start < end, "empty or invalid range");
        let bound = end - start; // > 0

        // Lemire's multiply-high + rejection
        // threshold = (2^32 % bound) to avoid bias
        let threshold = bound.wrapping_neg() % bound;
        loop {
            let x = self.next_u32();
            let m = (x as u64).wrapping_mul(bound as u64);
            let l = m as u32; // low 32 bits
            if l >= threshold {
                let r = (m >> 32) as u32; // high 32 bits ~ unbiased
                return start.wrapping_add(r);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_f32_in_unit_interval() {
        let mut rng = Krand::new(42);
        for _ in 0..10_000 {
            let x = rng.next_f32_0_1();
            assert!((0.0..1.0).contains(&x));
            assert!(!x.is_nan());
        }
    }

    #[test]
    fn range_u32_basic_cases() {
        let mut rng = Krand::new(7);
        for _ in 0..100 {
            assert_eq!(rng.range_u32(0..1), 0);
            let v = rng.range_u32(0..2);
            assert!(v == 0 || v == 1);
            let w = rng.range_u32(10..100);
            assert!((10..100).contains(&w));
        }
    }

    #[test]
    #[should_panic]
    fn range_panics_on_empty() {
        let mut rng = Krand::new(0);
        let _ = rng.range_u32(5..5);
    }

    // Integration: constructor + sequence reproducibility for next_u32
    #[test]
    fn integration_reproducible_sequence_u32() {
        let mut a = Krand::new(123);
        let mut b = Krand::new(123);
        for _ in 0..16 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }
}
