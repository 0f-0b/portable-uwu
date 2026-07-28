#![feature(const_array, const_trait_impl, doc_cfg, portable_simd)]
#![no_std]

mod simd;
mod transform;
mod xorshift;

use core::mem::MaybeUninit;

use transform::{bitap, emoji, nyaify, stutter};
use xorshift::XorShift32;

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::*;

/// uwuifies some bytes, storing output into the provided buffers. non-ascii bytes are unchanged.
///
/// the returned slice is a fully-initialized prefix of `out_buf`. `aux_buf` is left uninitialized.
///
/// # Panics
///
/// this function panics if the buffers are not large enough.
///
/// - `out_buf` must be at least `bytes.len() * 4 + 24` bytes long.
/// - `aux_buf` must be at least `bytes.len() * 2 + 15` bytes long.
///
/// # Example
///
/// ```
/// use portable_uwu::uwuify_into;
///
/// let bytes = "Hey, I think I really love you. Do you want a headpat?".as_bytes();
///
/// let mut out_buf = Box::new_uninit_slice(bytes.len() * 4 + 24);
/// let mut aux_buf = Box::new_uninit_slice(bytes.len() * 2 + 15);
/// let result = uwuify_into(bytes, &mut out_buf, &mut aux_buf);
/// drop(aux_buf);
///
/// assert_eq!(
///     result,
///     "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?".as_bytes(),
/// );
/// ```
pub fn uwuify_into<'a>(
    bytes: &[u8],
    out_buf: &'a mut [MaybeUninit<u8>],
    aux_buf: &mut [MaybeUninit<u8>],
) -> &'a [u8] {
    assert!(bytes.len() <= (usize::MAX - 24) / 4);
    assert!(out_buf.len() >= bytes.len() * 4 + 24);
    assert!(aux_buf.len() >= bytes.len() * 2 + 15);
    let mut rng = XorShift32::new(*b"uwu!");
    unsafe {
        let len = bitap(bytes, aux_buf);
        let len = nyaify(pad_zeros(aux_buf, len), len, out_buf);
        let len = stutter(pad_zeros(out_buf, len), len, aux_buf, &mut rng);
        let len = emoji(pad_zeros(aux_buf, len), len, out_buf, &mut rng);
        out_buf.get_unchecked(..len).assume_init_ref()
    }
}

#[inline(always)]
unsafe fn pad_zeros(bytes: &mut [MaybeUninit<u8>], len: usize) -> &[u8] {
    unsafe {
        bytes
            .get_unchecked_mut(len..len.next_multiple_of(16))
            .fill(MaybeUninit::new(0));
        bytes
            .get_unchecked(..len.next_multiple_of(16))
            .assume_init_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../testdata/input.txt");
    const EXPECTED: &str = include_str!("../testdata/output.txt");

    #[cfg(feature = "alloc")]
    #[test]
    fn uwuify_works() {
        let actual = uwuify(INPUT);
        assert_eq!(actual, EXPECTED);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn uwuify_bytes_works() {
        let actual = uwuify_bytes(INPUT.as_bytes());
        assert_eq!(actual, EXPECTED.as_bytes());
    }

    #[test]
    fn uwuify_into_works() {
        extern crate alloc;
        use alloc::boxed::Box;

        let bytes = INPUT.as_bytes();
        let mut out_buf = Box::new_uninit_slice(bytes.len() * 4 + 24);
        let mut aux_buf = Box::new_uninit_slice(bytes.len() * 2 + 15);
        let actual = uwuify_into(bytes, &mut out_buf, &mut aux_buf);
        assert_eq!(actual, EXPECTED.as_bytes());
    }
}
