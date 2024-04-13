use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, MaxEncodeLen, Read, Write};

mod error;

pub use error::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt<T>(pub T);

macro_rules! impl_u {
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

        impl MaxEncodeLen for VarInt<$ty> {
            const MAX_ENCODE_LEN: usize =
                ((std::mem::size_of::<$ty>() * 8 + 7 - 1) as f64 / 7f64) as usize;
        }

        impl Decode for VarInt<$ty> {
            type Error = VarIntTooLarge;

            fn decode(buf: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
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

            fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                let mut n = self.0;
                while n >= 0x80 {
                    dst.write(0b1000_000 | (n as u8))?;
                    n >>= 7;
                }
                dst.write(n as u8)?;
                Ok(())
            }
        }
    };
}

impl_u!(usize);
impl_u!(u8);
impl_u!(u16);
impl_u!(u32);
impl_u!(u64);
#[cfg(feature = "i128")]
impl_u!(u128);
