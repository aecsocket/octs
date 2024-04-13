use core::{
    convert::Infallible,
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

use super::InvalidValue;

macro_rules! impl_nz {
    ($nz:ty, $base:ty) => {
        impl FixedEncodeLen for $nz {
            const ENCODE_LEN: usize = std::mem::size_of::<$base>();
        }

        impl Decode for $nz {
            type Error = InvalidValue;

            #[inline]
            fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let value = <$base>::decode(src)?;
                <$nz>::new(value).ok_or(InvalidValue.into())
            }
        }

        impl Encode for $nz {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                self.get().encode(dst)
            }
        }
    };
}

impl_nz!(NonZeroUsize, usize);
impl_nz!(NonZeroIsize, isize);
impl_nz!(NonZeroU8, u8);
impl_nz!(NonZeroI8, i8);
impl_nz!(NonZeroU16, u16);
impl_nz!(NonZeroI16, i16);
impl_nz!(NonZeroU32, u32);
impl_nz!(NonZeroI32, i32);
impl_nz!(NonZeroU64, u64);
impl_nz!(NonZeroI64, i64);
#[cfg(feature = "i128")]
impl_nz!(core::num::NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz!(core::num::NonZeroI128, i128);

macro_rules! impl_nz_opt {
    ($nz:ty, $base:ty) => {
        impl FixedEncodeLen for Option<$nz> {
            const ENCODE_LEN: usize = std::mem::size_of::<$base>();
        }

        impl Decode for Option<$nz> {
            type Error = InvalidValue;

            #[inline]
            fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let value = <$base>::decode(src)?;
                Ok(<$nz>::new(value))
            }
        }

        impl Encode for Option<$nz> {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                self.map(<$nz>::get).unwrap_or_default().encode(dst)
            }
        }
    };
}

impl_nz_opt!(NonZeroUsize, usize);
impl_nz_opt!(NonZeroIsize, isize);
impl_nz_opt!(NonZeroU8, u8);
impl_nz_opt!(NonZeroI8, i8);
impl_nz_opt!(NonZeroU16, u16);
impl_nz_opt!(NonZeroI16, i16);
impl_nz_opt!(NonZeroU32, u32);
impl_nz_opt!(NonZeroI32, i32);
impl_nz_opt!(NonZeroU64, u64);
impl_nz_opt!(NonZeroI64, i64);
#[cfg(feature = "i128")]
impl_nz_opt!(core::num::NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz_opt!(core::num::NonZeroI128, i128);
