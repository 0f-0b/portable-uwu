use core::simd::prelude::*;

use super::super::simd::ShiftElementsDyn as _;
use super::super::xorshift::XorShift32;
use super::str::{
    ASCII_CASE_MASK, NEWLINE, SMALL_A, SMALL_L, SMALL_R, SMALL_W, SMALL_Z, SPACE, TAB,
};

pub unsafe fn stutter(
    in_bytes: &[u8],
    mut len: usize,
    mut out_bytes: &mut [u8],
    rng: &mut XorShift32,
) -> usize {
    unsafe {
        for vec in in_bytes
            .get_unchecked(..len.next_multiple_of(16))
            .as_chunks_unchecked::<16>()
        {
            let vec = u8x16::from_slice(vec);
            let lower = vec | ASCII_CASE_MASK;
            let alpha_mask = lower.simd_ge(SMALL_A) & lower.simd_le(SMALL_Z);
            let space_mask = vec.simd_eq(SPACE) | vec.simd_eq(TAB) | vec.simd_eq(NEWLINE);
            let replace_mask = lower.simd_eq(SMALL_L) | lower.simd_eq(SMALL_R);
            let stutter_mask = (space_mask.to_bitmask() << 1) & alpha_mask.to_bitmask();
            let replaced = alpha_mask.select(replace_mask.select(SMALL_W, lower), vec);
            replaced.copy_to_slice(out_bytes.get_unchecked_mut(..16));
            if stutter_mask != 0 {
                let stutter_index = stutter_mask.trailing_zeros() as usize;
                *out_bytes.get_unchecked_mut(stutter_index + 1) = b'-';
                let increment = rng.gen_bits(1) as usize * 2;
                out_bytes = out_bytes.get_unchecked_mut(increment..);
                len += increment;
                let rest = replaced.shift_elements_right_dyn(stutter_index);
                rest.copy_to_slice(out_bytes.get_unchecked_mut(stutter_index..stutter_index + 16));
            }
            out_bytes = out_bytes.get_unchecked_mut(16..);
        }
    }
    len
}
