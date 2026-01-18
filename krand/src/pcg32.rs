/// PCG32 (XSH-RR 64/32) non-cryptographic PRNG core.
/// Internal use: seeded via SplitMix64 and exposed through `Krand`.
/// Based on the PCG32 (XSH-RR 64/32) algorithm by Melissa O'Neill.
#[derive(Clone, Debug)]
pub(crate) struct Pcg32 {
    pub(crate) state: u64,
    pub(crate) inc: u64, // Must be odd
}

impl Pcg32 {
    /// Create a PCG32 with given initial state and odd increment (stream selector).
    /// The passed `inc` must be odd; callers are responsible to ensure `(inc & 1) == 1`.
    #[inline]
    pub(crate) fn seed(initstate: u64, inc_odd: u64) -> Self {
        debug_assert!(inc_odd & 1 == 1);
        // PCG recommended seeding: start from 0, set inc, advance, add initstate, advance
        let mut s = Self {
            state: 0,
            inc: inc_odd,
        };
        // advance once to mix inc into state
        let _ = s.next_u32();
        // incorporate initstate
        s.state = s.state.wrapping_add(initstate);
        // advance again to finalize seeding
        let _ = s.next_u32();
        s
    }

    /// Next u32 output.
    #[inline]
    pub(crate) fn next_u32(&mut self) -> u32 {
        const MULT: u64 = 6364136223846793005;
        let oldstate = self.state;
        self.state = oldstate.wrapping_mul(MULT).wrapping_add(self.inc);

        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_stream_same_sequence() {
        // inc must be odd
        let inc = 0xDA3E_39CB_94B9_5BDBu64 | 1;
        let mut a = Pcg32::seed(0x4d595df4d0f33173, inc);
        let mut b = Pcg32::seed(0x4d595df4d0f33173, inc);
        for _ in 0..8 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn different_streams_different_sequences() {
        let mut a = Pcg32::seed(123456789, (1u64 << 1) | 1);
        let mut b = Pcg32::seed(123456789, (3u64 << 1) | 1);
        // Very likely to differ on the first value with different streams
        assert_ne!(a.next_u32(), b.next_u32());
    }
}
