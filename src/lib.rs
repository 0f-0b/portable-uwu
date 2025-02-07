#![feature(
    decl_macro,
    doc_auto_cfg,
    maybe_uninit_array_assume_init,
    maybe_uninit_uninit_array_transpose,
    portable_simd,
    slice_as_chunks
)]
#![no_std]

mod array;
mod simd;
mod transform;
mod xorshift;

use transform::{bitap, emoji, nyaify, stutter};
use xorshift::XorShift32;

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::*;

/// uwuifies some bytes into the provided buffers. non-ascii bytes are unchanged.
///
/// this function panics if the buffers are not large enough. both buffers must be at least
/// `bytes.len() * 4 + 24` bytes long.
pub fn uwuify_into<'a>(bytes: &[u8], temp1: &'a mut [u8], temp2: &'a mut [u8]) -> &'a [u8] {
    assert!(temp1.len() >= bytes.len() * 4 + 24);
    assert!(temp2.len() >= bytes.len() * 4 + 24);
    let mut rng = XorShift32::new(*b"uwu!");
    unsafe {
        let len = bitap(bytes, temp1);
        pad_zeros(temp1, len);
        let len = nyaify(temp1, len, temp2);
        pad_zeros(temp2, len);
        let len = stutter(temp2, len, temp1, &mut rng);
        pad_zeros(temp1, len);
        let len = emoji(temp1, len, temp2, &mut rng);
        temp2.get_unchecked(..len)
    }
}

#[inline(always)]
unsafe fn pad_zeros(bytes: &mut [u8], len: usize) {
    unsafe { bytes.get_unchecked_mut(len..len.next_multiple_of(16)) }.fill(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&str, &str)] = &[
        (
            "Hey, I think I really love you. Do you want a headpat?",
            "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?",
        ),
        (
            include_str!("../testdata/input.txt"),
            include_str!("../testdata/output.txt"),
        ),
    ];

    #[cfg(feature = "alloc")]
    #[test]
    fn uwuify_works() {
        for &(input, expected) in CASES {
            let actual = uwuify(input);
            assert_eq!(actual, expected);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn uwuify_bytes_works() {
        for &(input, expected) in CASES {
            let actual = uwuify_bytes(input.as_bytes());
            assert_eq!(actual, expected.as_bytes());
        }
    }

    #[test]
    fn uwuify_into_works() {
        extern crate alloc;
        use alloc::vec;

        for &(input, expected) in CASES {
            let bytes = input.as_bytes();
            let mut temp1 = vec![0; bytes.len() * 4 + 24];
            let mut temp2 = vec![0; bytes.len() * 4 + 24];
            let actual = uwuify_into(bytes, &mut temp1, &mut temp2);
            assert_eq!(actual, expected.as_bytes());
        }
    }
}
