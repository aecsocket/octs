use crate::{ConstEncodeLen, Decode, Error, Read, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarInt<T>(pub T);

macro_rules! impl_u {
    ($ty:ty) => {
        impl ConstEncodeLen for VarInt<$ty> {
            const ENCODE_LEN: usize =
                ((std::mem::size_of::<$ty>() * 8 + 7 - 1) as f64 / 7f64) as usize;
        }

        impl Decode for VarInt<$ty> {
            fn decode(mut buf: impl Read) -> Result<Self> {
                let mut value: $ty = 0;
                for shift in 0..Self::ENCODE_LEN {
                    let byte = buf.read::<u8>()?;
                    let without_msb = byte & 0b0111_1111;
                    value |= <$ty>::from(without_msb) << shift * 7;

                    if byte & 0b1000_0000 == 0 {
                        return Ok(VarInt(value));
                    }
                }
                Err(Error::BadValue)
            }
        }
    };
}

impl_u!(u8);
impl_u!(u16);
impl_u!(u32);
impl_u!(u64);
#[cfg(feature = "i128")]
impl_u!(u128);

impl_u!(usize);
