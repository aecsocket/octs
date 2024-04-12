use core::{
    fmt::Display,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
};

use crate::{BufferTooShort, BufferTooShortOr, ConstEncodeLen, Decode, Encode, Read, Write};

use super::InvalidValue;

macro_rules! impl_base {
    ($ty:ty) => {
        impl ConstEncodeLen for $ty {
            const ENCODE_LEN: usize = std::mem::size_of::<$ty>();
        }

        impl Decode for $ty {
            type Error = BufferTooShort;

            fn decode(mut buf: impl Read) -> Result<Self, Self::Error> {
                Ok(<$ty>::from_be_bytes(*buf.read_exact()?))
            }
        }

        impl Encode for $ty {
            fn encode(&self, mut buf: impl Write) -> Result<()> {
                buf.write_slice(&self.to_be_bytes())
            }
        }
    };
}

impl_base!(u8);
impl_base!(i8);
impl_base!(u16);
impl_base!(i16);
impl_base!(u32);
impl_base!(i32);
impl_base!(u64);
impl_base!(i64);
#[cfg(feature = "i128")]
impl_base!(u128);
#[cfg(feature = "i128")]
impl_base!(i128);

impl_base!(usize);
impl_base!(isize);

impl_base!(f32);
impl_base!(f64);
