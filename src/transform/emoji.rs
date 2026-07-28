use core::mem::MaybeUninit;
use core::simd::prelude::*;

use super::super::simd::ShiftElementsDyn as _;
use super::super::xorshift::XorShift32;
use super::str::{COMMA, EXCLAMATION, NEWLINE, PERIOD, SPACE, TAB, str_to_vec};

const LUT_BITS: usize = 5;
const LUT_SIZE: usize = 1 << LUT_BITS;
const LUT: [&str; LUT_SIZE] = [
    " rawr x3",
    " OwO",
    " UwU",
    " o.O",
    " -.-",
    " >w<",
    " (⑅˘꒳˘)",
    " (ꈍᴗꈍ)",
    " (˘ω˘)",
    " (U ᵕ U❁)",
    " σωσ",
    " òωó",
    " (///ˬ///✿)",
    " (U ﹏ U)",
    " ( ͡o ω ͡o )",
    " ʘwʘ",
    " :3",
    " :3",
    " XD",
    " nyaa~~",
    " mya",
    " >_<",
    " 😳",
    " 🥺",
    " 😳😳😳",
    " rawr",
    " ^^",
    " ^^;;",
    " (ˆ ﻌ ˆ)♡",
    " ^•ﻌ•^",
    " /(^•ω•^)",
    " (✿oωo)",
];
const INSERT_VEC: [u8x16; LUT_SIZE] = LUT.map(str_to_vec);
const INSERT_LEN: [usize; LUT_SIZE] = LUT.map(str::len);

pub unsafe fn emoji(
    in_bytes: &[u8],
    mut len: usize,
    out_bytes: &mut [MaybeUninit<u8>],
    rng: &mut XorShift32,
) -> usize {
    let mut out_ptr = out_bytes.as_mut_ptr();
    unsafe {
        for vec in in_bytes.as_chunks_unchecked::<16>() {
            let vec = u8x16::from_slice(vec);
            out_ptr.cast::<u8x16>().write_unaligned(vec);
            let punctuation_mask =
                vec.simd_eq(COMMA) | vec.simd_eq(PERIOD) | vec.simd_eq(EXCLAMATION);
            let space_mask = vec.simd_eq(SPACE) | vec.simd_eq(TAB) | vec.simd_eq(NEWLINE);
            let insert_mask = punctuation_mask.to_bitmask()
                & !(punctuation_mask.to_bitmask() << 1)
                & (space_mask.to_bitmask() >> 1);
            if insert_mask != 0 {
                let insert_index = insert_mask.trailing_zeros() as usize + 1;
                let rand_index = rng.gen_bits(LUT_BITS) as usize;
                let insert = INSERT_VEC[rand_index];
                let insert_len = INSERT_LEN[rand_index];
                out_ptr
                    .add(insert_index)
                    .cast::<u8x16>()
                    .write_unaligned(insert);
                out_ptr = out_ptr.add(insert_len);
                len += insert_len;
                let rest = vec.shift_elements_right_dyn(insert_index);
                out_ptr
                    .add(insert_index)
                    .cast::<u8x16>()
                    .write_unaligned(rest);
            }
            out_ptr = out_ptr.add(16);
        }
    }
    len
}
