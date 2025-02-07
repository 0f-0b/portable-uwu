extern crate alloc;

use alloc::borrow::ToOwned as _;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::str;

use super::uwuify_into;

/// uwuifies a string.
#[must_use = "this function returns a new String without modifying the original"]
pub fn uwuify(s: &str) -> String {
    let result = uwuify_bytes(s.as_bytes());
    // SAFETY: `s` is valid UTF-8; `uwuify_bytes` outputs non-ASCII bytes as-is, resulting in
    // still valid UTF-8.
    unsafe { String::from_utf8_unchecked(result) }
}

/// uwuifies some bytes. non-ascii bytes are unchanged.
#[must_use = "this function returns a new Vec without modifying the original"]
pub fn uwuify_bytes(v: &[u8]) -> Vec<u8> {
    let mut temp1 = vec![0; v.len() * 4 + 24];
    let mut temp2 = vec![0; v.len() * 4 + 24];
    uwuify_into(v, &mut temp1, &mut temp2).to_owned()
}
