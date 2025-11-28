use core::simd::prelude::*;
use core::simd::{LaneCount, SupportedLaneCount};

pub const ASCII_CASE_MASK: u8x16 = Simd::splat(1 << 5);
pub const SMALL_A: u8x16 = Simd::splat(b'a');
pub const SMALL_L: u8x16 = Simd::splat(b'l');
pub const SMALL_N: u8x16 = Simd::splat(b'n');
pub const SMALL_R: u8x16 = Simd::splat(b'r');
pub const SMALL_W: u8x16 = Simd::splat(b'w');
pub const SMALL_Z: u8x16 = Simd::splat(b'z');
pub const COMMA: u8x16 = Simd::splat(b',');
pub const PERIOD: u8x16 = Simd::splat(b'.');
pub const EXCLAMATION: u8x16 = Simd::splat(b'!');
pub const SPACE: u8x16 = Simd::splat(b' ');
pub const TAB: u8x16 = Simd::splat(b'\t');
pub const NEWLINE: u8x16 = Simd::splat(b'\n');

pub const fn str_to_vec<const MAX_LEN: usize>(s: &str) -> Simd<u8, MAX_LEN>
where
    LaneCount<MAX_LEN>: SupportedLaneCount,
{
    let bytes = s.as_bytes();
    let len = bytes.len();
    assert!(len <= MAX_LEN);
    let mut res = [0; MAX_LEN];
    let mut i = 0;
    while i < len {
        res[i] = bytes[i];
        i += 1;
    }
    Simd::from_array(res)
}
