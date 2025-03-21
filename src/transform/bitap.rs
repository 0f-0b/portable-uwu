use core::simd::prelude::*;
use core::simd::{LaneCount, SupportedLaneCount};

use super::super::array::const_map;
use super::str::str_to_vec;

const fn get_masks<const N: usize>(patterns: [&str; N]) -> [Simd<u16, N>; 256]
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut res = [[0; N]; 256];
    let mut i = 0;
    while i < N {
        let bytes = patterns[i].as_bytes();
        let len = bytes.len();
        assert!(len > 0 && len <= 16);
        let offset = 16 - len;
        let mut j = 0;
        while j < len {
            res[bytes[j].to_ascii_lowercase() as usize][i] |= 1 << (offset + j);
            res[bytes[j].to_ascii_uppercase() as usize][i] |= 1 << (offset + j);
            j += 1;
        }
        i += 1;
    }
    const_map!(Simd::from_array, res)
}

const fn get_start_mask<const N: usize>(patterns: [&str; N]) -> Simd<u16, N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const MAX_LEN: usize = u16::BITS as usize;
    let mut res = [0; N];
    let mut i = 0;
    while i < N {
        let bytes = patterns[i].as_bytes();
        let len = bytes.len();
        assert!(len > 0 && len <= MAX_LEN);
        let offset = MAX_LEN - len;
        res[i] |= 1 << offset;
        i += 1;
    }
    Simd::from_array(res)
}

const PATTERNS: [&str; 8] = [
    "small", "cute", "fluff", "love", "stupid", "what", "meow", "meow",
];
const REPLACE: [&str; 8] = [
    "smol", "kawaii~", "floof", "luv", "baka", "nani", "nya~", "nya~",
];
const START_MASK: u16x8 = get_start_mask(PATTERNS);
const MASKS: [u16x8; 256] = get_masks(PATTERNS);
const PATTERN_LEN: [usize; 8] = const_map!(str::len, PATTERNS);
const REPLACE_LEN: [usize; 8] = const_map!(str::len, REPLACE);
const REPLACE_VEC: [u8x16; 8] = const_map!(str_to_vec, REPLACE);

#[derive(Debug, PartialEq, Eq)]
struct Match {
    match_len: usize,
    replace_len: usize,
    replace_vec: u8x16,
}

struct Bitap8x16(u16x8);

impl Bitap8x16 {
    #[inline]
    fn new() -> Self {
        Self(Simd::splat(0))
    }

    #[inline]
    fn next(&mut self, c: u8) -> Option<Match> {
        self.0 <<= 1;
        self.0 = (self.0 | START_MASK) & MASKS[c as usize];
        let match_mask = self.0.cast::<i16>().is_negative().to_bitmask();
        if match_mask == 0 {
            return None;
        }
        let match_index = match_mask.trailing_zeros() as usize;
        Some(Match {
            match_len: PATTERN_LEN[match_index],
            replace_len: REPLACE_LEN[match_index],
            replace_vec: REPLACE_VEC[match_index],
        })
    }
}

pub unsafe fn bitap(in_bytes: &[u8], out_bytes: &mut [u8]) -> usize {
    let mut len = in_bytes.len();
    let mut out_ptr = out_bytes.as_mut_ptr();
    let mut bitap = Bitap8x16::new();
    unsafe {
        for &c in in_bytes {
            *out_ptr = c;
            out_ptr = out_ptr.add(1);
            if let Some(m) = bitap.next(c) {
                out_ptr = out_ptr.sub(m.match_len);
                out_ptr.cast::<u8x16>().write_unaligned(m.replace_vec);
                out_ptr = out_ptr.add(m.replace_len);
                len = len - m.match_len + m.replace_len;
                bitap = Bitap8x16::new();
            }
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitap8x16_works() {
        let mut bitap = Bitap8x16::new();

        assert_eq!(bitap.next(b'c'), None);
        assert_eq!(bitap.next(b'u'), None);
        assert_eq!(bitap.next(b't'), None);
        assert_eq!(
            bitap.next(b'e'),
            Some(Match {
                match_len: 4,
                replace_len: 7,
                replace_vec: str_to_vec("kawaii~"),
            })
        );

        assert_eq!(bitap.next(b'w'), None);
        assert_eq!(bitap.next(b'h'), None);
        assert_eq!(bitap.next(b'a'), None);
        assert_eq!(
            bitap.next(b't'),
            Some(Match {
                match_len: 4,
                replace_len: 4,
                replace_vec: str_to_vec("nani"),
            })
        );

        assert_eq!(bitap.next(b'w'), None);
        assert_eq!(bitap.next(b'h'), None);
        assert_eq!(bitap.next(b'a'), None);
        assert_eq!(bitap.next(b'a'), None);

        assert_eq!(bitap.next(b'W'), None);
        assert_eq!(bitap.next(b'h'), None);
        assert_eq!(bitap.next(b'A'), None);
        assert_eq!(
            bitap.next(b't'),
            Some(Match {
                match_len: 4,
                replace_len: 4,
                replace_vec: str_to_vec("nani"),
            })
        );
    }
}
