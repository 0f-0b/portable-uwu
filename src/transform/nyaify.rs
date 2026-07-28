use core::mem::MaybeUninit;
use core::simd::prelude::*;

use super::super::simd::ShiftElementsDyn as _;
use super::str::{ASCII_CASE_MASK, NEWLINE, SMALL_N, SPACE, TAB};

pub unsafe fn nyaify(in_bytes: &[u8], mut len: usize, out_bytes: &mut [MaybeUninit<u8>]) -> usize {
    let mut out_ptr = out_bytes.as_mut_ptr();
    unsafe {
        for vec in in_bytes.as_chunks_unchecked::<16>() {
            let vec = u8x16::from_slice(vec);
            out_ptr.cast::<u8x16>().write_unaligned(vec);
            let n_mask = (vec | ASCII_CASE_MASK).simd_eq(SMALL_N);
            let space_mask = vec.simd_eq(SPACE) | vec.simd_eq(TAB) | vec.simd_eq(NEWLINE);
            let mut nya_mask = (space_mask.to_bitmask() << 1) & n_mask.to_bitmask();
            while nya_mask != 0 {
                let nya_index = nya_mask.trailing_zeros() as usize + 1;
                nya_mask &= nya_mask - 1;
                out_ptr.add(nya_index).cast::<u8>().write(b'y');
                out_ptr = out_ptr.add(1);
                len += 1;
                let rest = vec.shift_elements_right_dyn(nya_index);
                out_ptr.add(nya_index).cast::<u8x16>().write_unaligned(rest);
            }
            out_ptr = out_ptr.add(16);
        }
    }
    len
}
