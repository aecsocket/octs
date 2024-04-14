#![cfg_attr(any(nightly, docsrs), feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use bytes::{self, Buf, BufMut, Bytes, BytesMut};

mod error;
mod read;
mod varint;
mod write;

pub mod chunks;
pub mod prim;

pub use {error::*, read::*, varint::*, write::*};

#[cfg(test)]
pub(crate) mod __test {
    use core::fmt::Debug;

    use bytes::{Buf, BytesMut};

    use crate::{Decode, Encode, EncodeLen, FixedEncodeLenHint, Read, Write};

    pub fn round_trip<T>(value: T)
    where
        T: Debug + Encode + Decode + EncodeLen + FixedEncodeLenHint + PartialEq,
        <T as Encode>::Error: Debug,
        <T as Decode>::Error: Debug,
    {
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

        let mut buf = BytesMut::with_capacity(encode_len);
        buf.write(&value).unwrap();
        assert_eq!(encode_len, buf.len());

        let mut buf = buf.freeze();
        assert_eq!(encode_len, buf.remaining());

        let buf_clone = buf.clone();
        let actual = buf.read::<T>().unwrap();
        assert!(
            value == actual,
            "expected = {value:?}, actual = {actual:?}, buf: {:?}",
            buf_clone.chunk()
        );
        assert_eq!(0, buf.remaining());
    }
}
