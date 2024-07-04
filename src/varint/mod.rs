use core::{convert::Infallible, mem::size_of};

use crate::{BufTooShortOr, Decode, Encode, EncodeLen, FixedEncodeLenHint, Read, Write};

mod error;

pub use error::*;

/// Integer which is encoded in a variable amount of bytes.
///
/// See the [*Protocol Buffers Documentation*] on an explanation of what
/// varints are, and how they are encoded.
///
/// [*Protocol Buffers Documentation*]: https://protobuf.dev/programming-guides/encoding/#varints
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt<T>(pub T);

macro_rules! impl_base {
    ($ty:ty) => {
        impl From<$ty> for VarInt<$ty> {
            fn from(value: $ty) -> Self {
                Self(value)
            }
        }

        impl From<VarInt<$ty>> for $ty {
            fn from(value: VarInt<$ty>) -> Self {
                value.0
            }
        }

        impl FixedEncodeLenHint for VarInt<$ty> {
            const MIN_ENCODE_LEN: usize = 1;

            const MAX_ENCODE_LEN: usize = (<$ty>::BITS as usize + 7) / 7;
        }
    };
}

macro_rules! impl_unsigned {
    ($ty:ty) => {
        impl_base!($ty);

        impl EncodeLen for VarInt<$ty> {
            #[inline]
            fn encode_len(&self) -> usize {
                let mut v = self.0;
                let mut len = 0;
                while v > 0 {
                    len += 1;
                    v >>= 7;
                }
                // encoded len is always at least 1
                len.max(1)
            }
        }

        impl Decode for VarInt<$ty> {
            type Error = VarIntTooLarge;

            #[inline]
            fn decode(mut buf: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let mut value: $ty = 0;
                for shift in 0..Self::MAX_ENCODE_LEN {
                    let byte = buf.read::<u8>()?;
                    let without_msb = byte & 0b0111_1111;
                    value |= <$ty>::from(without_msb) << shift * 7;

                    if byte & 0b1000_0000 == 0 {
                        return Ok(VarInt(value));
                    }
                }
                Err(VarIntTooLarge.into())
            }
        }

        impl Encode for VarInt<$ty> {
            type Error = Infallible;

            #[inline]
            fn encode(&self, mut dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                let mut n = self.0;
                while n >= 0x80 {
                    let b: u8 = 0b1000_0000 | (n as u8);
                    dst.write(b)?;
                    n >>= 7;
                }
                dst.write(n as u8)?;
                Ok(())
            }
        }
    };
}

const _: () = assert!(size_of::<usize>() <= size_of::<u64>());

impl_unsigned!(usize);
impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);

// signed

macro_rules! impl_signed {
    ($ty:ty, $un:ty) => {
        impl_base!($ty);

        impl VarInt<$ty> {
            #[inline]
            fn zigzag_encode(v: $ty) -> $un {
                const BITS: u32 = <$ty>::BITS;
                ((v << 1) ^ (v >> (BITS - 1))) as $un
            }

            #[inline]
            fn zigzag_decode(v: $un) -> $ty {
                ((v >> 1) ^ (-((v & 1) as $ty)) as $un) as $ty
            }
        }

        impl EncodeLen for VarInt<$ty> {
            #[inline]
            fn encode_len(&self) -> usize {
                VarInt(Self::zigzag_encode(self.0)).encode_len()
            }
        }

        impl Decode for VarInt<$ty> {
            type Error = VarIntTooLarge;

            #[inline]
            fn decode(mut buf: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let VarInt(value) = buf.read::<VarInt<$un>>()?;
                Ok(VarInt(Self::zigzag_decode(value)))
            }
        }

        impl Encode for VarInt<$ty> {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                VarInt(Self::zigzag_encode(self.0)).encode(dst)
            }
        }
    };
}

impl_signed!(isize, usize);
impl_signed!(i8, u8);
impl_signed!(i16, u16);
impl_signed!(i32, u32);
impl_signed!(i64, u64);

#[cfg(test)]
mod tests {
    use bytes::Buf;

    use super::*;

    #[test]
    fn round_trip_all_u8s() {
        for v in 0..u8::MAX {
            crate::__test::round_trip(VarInt(v));
        }
    }

    #[test]
    fn round_trip_all_i8s() {
        for v in 0..i8::MAX {
            crate::__test::round_trip(VarInt(v));
        }
    }

    #[test]
    fn round_trip_all_u16s() {
        for v in 0..u16::MAX {
            crate::__test::round_trip(VarInt(v));
        }
    }

    #[test]
    fn round_trip_all_i16s() {
        for v in 0..i16::MAX {
            crate::__test::round_trip(VarInt(v));
        }
    }

    #[test]
    fn decode_all_msbs() {
        const LEN: usize = 64;
        let mut buf = &[0x80; LEN][..];
        buf.read::<VarInt<u8>>().unwrap_err();
        // make sure it doesn't try to read the entire buffer
        assert_eq!(LEN - VarInt::<u8>::MAX_ENCODE_LEN, buf.remaining());
    }
}
