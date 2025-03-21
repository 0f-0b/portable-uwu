use core::simd::prelude::*;

use super::super::simd::ShiftElementsDyn as _;
use super::str::{ASCII_CASE_MASK, NEWLINE, SMALL_N, SPACE, TAB};

pub unsafe fn nyaify(in_bytes: &[u8], mut len: usize, mut out_bytes: &mut [u8]) -> usize {
    unsafe {
        for vec in in_bytes
            .get_unchecked(..len.next_multiple_of(16))
            .as_chunks_unchecked::<16>()
        {
            let vec = u8x16::from_slice(vec);
            vec.copy_to_slice(out_bytes.get_unchecked_mut(..16));
            let n_mask = (vec | ASCII_CASE_MASK).simd_eq(SMALL_N);
            let space_mask = vec.simd_eq(SPACE) | vec.simd_eq(TAB) | vec.simd_eq(NEWLINE);
            let mut nya_mask = (space_mask.to_bitmask() << 1) & n_mask.to_bitmask();
            while nya_mask != 0 {
                let nya_index = nya_mask.trailing_zeros() as usize + 1;
                nya_mask &= nya_mask - 1;
                *out_bytes.get_unchecked_mut(nya_index) = b'y';
                out_bytes = out_bytes.get_unchecked_mut(1..);
                len += 1;
                let rest = vec.shift_elements_right_dyn(nya_index);
                rest.copy_to_slice(out_bytes.get_unchecked_mut(nya_index..nya_index + 16));
            }
            out_bytes = out_bytes.get_unchecked_mut(16..);
        }
    }
    len
}
