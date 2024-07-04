use core::{
    convert::Infallible,
    mem::size_of,
    num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

#[cfg(feature = "i128")]
use core::num::{NonZeroI128, NonZeroU128};

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

use super::InvalidValue;

macro_rules! impl_nz {
    ($nz:ty, $base:ty) => {
        impl FixedEncodeLen for $nz {
            const ENCODE_LEN: usize = size_of::<$base>();
        }

        impl Decode for $nz {
            type Error = InvalidValue;

            #[inline]
            fn decode(src: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let value = <$base>::decode(src)?;
                <$nz>::new(value).ok_or(InvalidValue(()).into())
            }
        }

        impl Encode for $nz {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
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
impl_nz!(NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz!(NonZeroI128, i128);

macro_rules! impl_nz_opt {
    ($nz:ty, $base:ty) => {
        impl FixedEncodeLen for Option<$nz> {
            const ENCODE_LEN: usize = size_of::<$base>();
        }

        impl Decode for Option<$nz> {
            type Error = Infallible;

            #[inline]
            fn decode(src: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                let value = <$base>::decode(src)?;
                Ok(<$nz>::new(value))
            }
        }

        impl Encode for Option<$nz> {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
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
impl_nz_opt!(NonZeroU128, u128);
#[cfg(feature = "i128")]
impl_nz_opt!(NonZeroI128, i128);

#[cfg(test)]
mod tests {
    use core::num::{
        NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize,
    };

    #[cfg(feature = "i128")]
    use core::num::{NonZeroI128, NonZeroU128};

    use super::*;

    macro_rules! round_trip {
        ($nz:ty, $base:ty) => {
            crate::__test::round_trip(<$nz>::MIN);
            crate::__test::round_trip(<$nz>::new(1 as $base).unwrap());
            crate::__test::round_trip(<$nz>::new(2 as $base).unwrap());
            crate::__test::round_trip(<$nz>::MAX);
        };
    }

    #[test]
    fn round_trip() {
        round_trip!(NonZeroUsize, usize);
        round_trip!(NonZeroIsize, isize);
        round_trip!(NonZeroU8, u8);
        round_trip!(NonZeroI8, i8);
        round_trip!(NonZeroU16, u16);
        round_trip!(NonZeroI16, i16);
        round_trip!(NonZeroU32, u32);
        round_trip!(NonZeroI32, i32);
        round_trip!(NonZeroU64, u64);
        round_trip!(NonZeroI64, i64);
        #[cfg(feature = "i128")]
        {
            round_trip!(core::num::NonZeroU128, u128);
            round_trip!(core::num::NonZeroI128, i128);
        }
    }

    macro_rules! decode_zero {
        ($nz:ty) => {
            let mut buf = &[0u8; 16][..];
            buf.read::<$nz>().unwrap_err();
        };
    }

    #[test]
    fn decode_zero() {
        decode_zero!(NonZeroUsize);
        decode_zero!(NonZeroIsize);
        decode_zero!(NonZeroU8);
        decode_zero!(NonZeroI8);
        decode_zero!(NonZeroU16);
        decode_zero!(NonZeroI16);
        decode_zero!(NonZeroU32);
        decode_zero!(NonZeroI32);
        decode_zero!(NonZeroU64);
        decode_zero!(NonZeroI64);
        #[cfg(feature = "i128")]
        {
            decode_zero!(NonZeroU128);
            decode_zero!(NonZeroI128);
        }
    }

    macro_rules! round_trip_opt {
        ($nz:ty, $base:ty) => {
            crate::__test::round_trip(Some(<$nz>::MIN));
            crate::__test::round_trip(Option::<$nz>::None);
            crate::__test::round_trip(Some(<$nz>::new(1 as $base).unwrap()));
            crate::__test::round_trip(Some(<$nz>::new(2 as $base).unwrap()));
            crate::__test::round_trip(Some(<$nz>::MAX));
        };
    }

    #[test]
    fn round_trip_opt() {
        round_trip_opt!(NonZeroUsize, usize);
        round_trip_opt!(NonZeroIsize, isize);
        round_trip_opt!(NonZeroU8, u8);
        round_trip_opt!(NonZeroI8, i8);
        round_trip_opt!(NonZeroU16, u16);
        round_trip_opt!(NonZeroI16, i16);
        round_trip_opt!(NonZeroU32, u32);
        round_trip_opt!(NonZeroI32, i32);
        round_trip_opt!(NonZeroU64, u64);
        round_trip_opt!(NonZeroI64, i64);
        #[cfg(feature = "i128")]
        {
            round_trip_opt!(core::num::NonZeroU128, u128);
            round_trip_opt!(core::num::NonZeroI128, i128);
        }
    }

    macro_rules! decode_zero_opt {
        ($nz:ty) => {
            let mut buf = &[0u8; 16][..];
            assert_eq!(None, buf.read::<Option<$nz>>().unwrap());
        };
    }

    #[test]
    fn decode_zero_opt() {
        decode_zero_opt!(NonZeroUsize);
        decode_zero_opt!(NonZeroIsize);
        decode_zero_opt!(NonZeroU8);
        decode_zero_opt!(NonZeroI8);
        decode_zero_opt!(NonZeroU16);
        decode_zero_opt!(NonZeroI16);
        decode_zero_opt!(NonZeroU32);
        decode_zero_opt!(NonZeroI32);
        decode_zero_opt!(NonZeroU64);
        decode_zero_opt!(NonZeroI64);
        #[cfg(feature = "i128")]
        {
            decode_zero_opt!(NonZeroU128);
            decode_zero_opt!(NonZeroI128);
        }
    }
}
