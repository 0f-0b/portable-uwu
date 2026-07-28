extern crate alloc;

use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::str;

use super::uwuify_into;

/// uwuifies a string.
///
/// # Example
///
/// ```
/// use portable_uwu::uwuify;
///
/// assert_eq!(
///     uwuify("Hey, I think I really love you. Do you want a headpat?"),
///     "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?",
/// );
/// ```
#[must_use = "this function returns a new String without modifying the original"]
pub fn uwuify(s: &str) -> String {
    let result = uwuify_bytes(s.as_bytes());
    // SAFETY: `s` is valid UTF-8; `uwuify_bytes` outputs non-ASCII bytes as-is, resulting in
    // still valid UTF-8.
    unsafe { String::from_utf8_unchecked(result) }
}

/// uwuifies some bytes. non-ascii bytes are unchanged.
///
/// # Example
///
/// ```
/// use portable_uwu::uwuify_bytes;
///
/// assert_eq!(
///     uwuify_bytes("Hey, I think I really love you. Do you want a headpat?".as_bytes()),
///     "hey, (ꈍᴗꈍ) i think i weawwy wuv you. ^•ﻌ•^ do y-you want a headpat?".as_bytes(),
/// );
/// ```
#[must_use = "this function returns a new Vec without modifying the original"]
pub fn uwuify_bytes(v: &[u8]) -> Vec<u8> {
    assert!(v.len() <= (usize::MAX - 24) / 4);
    let mut out_buf = Box::new_uninit_slice(v.len() * 4 + 24);
    let mut aux_buf = Box::new_uninit_slice(v.len() * 2 + 15);
    let result = uwuify_into(v, &mut out_buf, &mut aux_buf);
    drop(aux_buf);
    result.to_owned()
}
