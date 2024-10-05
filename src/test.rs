#![allow(clippy::missing_panics_doc)]

//! Utilities for testing trait implementations.
//!
//! Use these functions when writing unit tests for your [`Encode`] and
//! [`Decode`] implementations.

use {
    crate::{Buf, BytesMut, Decode, Encode, EncodeLen, FixedEncodeLenHint, Read, Write},
    core::fmt::Debug,
};

/// Asserts that `T`'s [`EncodeLen::encode_len`] is within the bounds of
/// [`FixedEncodeLenHint`].
pub fn encode_len_hint<T: FixedEncodeLenHint>(value: &T) {
    let encode_len = value.encode_len();
    assert!(
        encode_len >= T::MIN_ENCODE_LEN,
        "encode_len = {encode_len}, T::MIN_ENCODE_LEN: {}",
        T::MIN_ENCODE_LEN
    );
    assert!(
        encode_len <= T::MAX_ENCODE_LEN,
        "encode_len = {encode_len}, T::MAX_ENCODE_LEN: {}",
        T::MAX_ENCODE_LEN
    );
}

/// Asserts that `decode(encode(value)) == value`.
pub fn round_trip<T>(value: &T)
where
    T: Debug + Encode + Decode + EncodeLen + PartialEq,
    <T as Encode>::Error: Debug,
    <T as Decode>::Error: Debug,
{
    let encode_len = value.encode_len();

    let mut buf = BytesMut::with_capacity(encode_len);
    buf.write(&value).unwrap();
    assert_eq!(encode_len, buf.len());

    let mut buf = buf.freeze();
    assert_eq!(encode_len, buf.remaining());

    let buf_clone = buf.clone();
    let actual = buf.read::<T>().unwrap();
    assert!(
        *value == actual,
        "expected = {value:?}, actual = {actual:?}, buf: {:?}",
        buf_clone.chunk()
    );
    assert_eq!(0, buf.remaining());
}

/// Asserts [`encode_len_hint`] and [`round_trip`] for `T`.
pub fn hint_round_trip<T>(value: &T)
where
    T: Debug + Encode + Decode + EncodeLen + FixedEncodeLenHint + PartialEq,
    <T as Encode>::Error: Debug,
    <T as Decode>::Error: Debug,
{
    encode_len_hint(value);
    round_trip(value);
}
