/// SplitMix64 PRNG used for seeding other generators.
/// Non-cryptographic; provides good equidistribution and speed.
/// Based on the SplitMix64 algorithm by Sebastiano Vigna.
#[derive(Clone, Debug)]
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Creates a new SplitMix64 with the given seed.
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advances the generator and returns the next u64.
    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        // Constants from the reference SplitMix64 implementation.
        const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;
        const MUL1: u64 = 0xBF58_476D_1CE4_E5B9;
        const MUL2: u64 = 0x94D0_49BB_1331_11EB;

        let mut z = self.state;
        self.state = self.state.wrapping_add(GAMMA);

        z ^= z >> 30;
        z = z.wrapping_mul(MUL1);
        z ^= z >> 27;
        z = z.wrapping_mul(MUL2);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = SplitMix64::new(123456789);
        let mut b = SplitMix64::new(123456789);

        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_different_first_value() {
        let mut a = SplitMix64::new(1);
        let mut b = SplitMix64::new(2);
        assert_ne!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn progresses_and_changes_values() {
        let mut sm = SplitMix64::new(0);
        let v1 = sm.next_u64();
        let v2 = sm.next_u64();
        let v3 = sm.next_u64();
        // Extremely unlikely to be equal if implemented correctly (practical sanity check)
        assert_ne!(v1, v2);
        assert_ne!(v2, v3);
        assert_ne!(v1, v3);
    }
}
