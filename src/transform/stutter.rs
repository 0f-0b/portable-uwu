use core::mem::MaybeUninit;
use core::simd::Select as _;
use core::simd::prelude::*;

use super::super::simd::ShiftElementsDyn as _;
use super::super::xorshift::XorShift32;
use super::str::{
    ASCII_CASE_MASK, NEWLINE, SMALL_A, SMALL_L, SMALL_R, SMALL_W, SMALL_Z, SPACE, TAB,
};

pub unsafe fn stutter(
    in_bytes: &[u8],
    mut len: usize,
    out_bytes: &mut [MaybeUninit<u8>],
    rng: &mut XorShift32,
) -> usize {
    let mut out_ptr = out_bytes.as_mut_ptr();
    unsafe {
        for vec in in_bytes.as_chunks_unchecked::<16>() {
            let vec = u8x16::from_slice(vec);
            let lower = vec | ASCII_CASE_MASK;
            let alpha_mask = lower.simd_ge(SMALL_A) & lower.simd_le(SMALL_Z);
            let space_mask = vec.simd_eq(SPACE) | vec.simd_eq(TAB) | vec.simd_eq(NEWLINE);
            let replace_mask = lower.simd_eq(SMALL_L) | lower.simd_eq(SMALL_R);
            let stutter_mask = (space_mask.to_bitmask() << 1) & alpha_mask.to_bitmask();
            let replaced = alpha_mask.select(replace_mask.select(SMALL_W, lower), vec);
            out_ptr.cast::<u8x16>().write_unaligned(replaced);
            if stutter_mask != 0 {
                let stutter_index = stutter_mask.trailing_zeros() as usize;
                out_ptr.add(stutter_index + 1).cast::<u8>().write(b'-');
                let increment = rng.gen_bits(1) as usize * 2;
                out_ptr = out_ptr.add(increment);
                len += increment;
                let rest = replaced.shift_elements_right_dyn(stutter_index);
                out_ptr
                    .add(stutter_index)
                    .cast::<u8x16>()
                    .write_unaligned(rest);
            }
            out_ptr = out_ptr.add(16);
        }
    }
    len
}
