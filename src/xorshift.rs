pub struct XorShift32 {
    state: u32,
    counter: u32,
}

impl XorShift32 {
    #[inline(always)]
    pub fn new(seed: [u8; 4]) -> Self {
        let state = u32::from_le_bytes(seed);
        XorShift32 {
            state: state | 1,
            counter: state,
        }
    }

    #[inline(always)]
    pub fn gen_u32(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.counter = self.counter.wrapping_add(1234567891);
        self.state.wrapping_add(self.counter)
    }

    #[inline(always)]
    pub fn gen_bits(&mut self, bits: usize) -> u32 {
        assert!(bits <= 32);
        self.gen_u32() & (!0u32).checked_shr(32 - bits as u32).unwrap_or(0)
    }
}
