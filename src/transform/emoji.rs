use core_simd::simd::prelude::*;

use super::super::array::const_map;
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
const INSERT_VEC: [u8x16; LUT_SIZE] = const_map!(str_to_vec, LUT);
const INSERT_LEN: [usize; LUT_SIZE] = const_map!(str::len, LUT);

pub unsafe fn emoji(
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
            vec.copy_to_slice(out_bytes.get_unchecked_mut(..16));
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
                insert.copy_to_slice(out_bytes.get_unchecked_mut(insert_index..insert_index + 16));
                out_bytes = out_bytes.get_unchecked_mut(insert_len..);
                len += insert_len;
                let rest = vec.shift_elements_right_dyn(insert_index);
                rest.copy_to_slice(out_bytes.get_unchecked_mut(insert_index..insert_index + 16));
            }
            out_bytes = out_bytes.get_unchecked_mut(16..);
        }
    }
    len
}
