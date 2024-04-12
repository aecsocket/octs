use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8,
};

use crate::{BufferTooShort, BufferTooShortOr, ConstEncodeLen, Decode, Encode, Read, Write};

use super::InvalidValue;

macro_rules! impl_nz {
    ($nz:ty, $base:ty) => {
        impl ConstEncodeLen for $nz {
            const ENCODE_LEN: usize = std::mem::size_of::<$base>();
        }

        impl Decode for $nz {
            type Error = BufferTooShortOr<InvalidValue>;

            fn decode(buf: impl Read) -> Result<Self, Self::Error> {
                <$nz>::new(<$base>::decode(buf)?).ok_or(InvalidValue.into())
            }
        }

        impl Encode for $nz {
            fn encode(&self, buf: impl Write) -> Result<()> {
                self.get().encode(buf)
            }
        }
    };
}

impl_nz!(NonZeroU8, u8);
impl_nz!(NonZeroI8, i8);
impl_nz!(NonZeroU16, u16);
impl_nz!(NonZeroI16, i16);
impl_nz!(NonZeroU32, u32);
impl_nz!(NonZeroI32, i32);
impl_nz!(NonZeroU64, u64);
impl_nz!(NonZeroI64, i64);
#[cfg(feature = "i128")]
impl_nz!(NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz!(NonZeroI128, i128);

macro_rules! impl_nz_opt {
    ($nz:ty, $base:ty) => {
        impl ConstEncodeLen for Option<$nz> {
            const ENCODE_LEN: usize = std::mem::size_of::<$base>();
        }

        impl Decode for Option<$nz> {
            fn decode(buf: impl Read) -> Result<Self> {
                Ok(<$nz>::new(<$base>::decode(buf)?))
            }
        }

        impl Encode for Option<$nz> {
            fn encode(&self, buf: impl Write) -> Result<()> {
                self.map(<$nz>::get).unwrap_or_default().encode(buf)
            }
        }
    };
}

impl_nz_opt!(NonZeroU8, u8);
impl_nz_opt!(NonZeroI8, i8);
impl_nz_opt!(NonZeroU16, u16);
impl_nz_opt!(NonZeroI16, i16);
impl_nz_opt!(NonZeroU32, u32);
impl_nz_opt!(NonZeroI32, i32);
impl_nz_opt!(NonZeroU64, u64);
impl_nz_opt!(NonZeroI64, i64);
#[cfg(feature = "i128")]
impl_nz_opt!(NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz_opt!(NonZeroI128, i128);
