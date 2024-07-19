use core::{convert::Infallible, mem::size_of};

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

macro_rules! impl_for {
    ($ty:ty) => {
        impl FixedEncodeLen for $ty {
            const ENCODE_LEN: usize = size_of::<$ty>();
        }

        impl Decode for $ty {
            type Error = Infallible;

            #[inline]
            fn decode(mut src: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                Ok(<$ty>::from_be_bytes(src.read_exact()?))
            }
        }

        impl Encode for $ty {
            type Error = Infallible;

            #[inline]
            fn encode(&self, mut dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
                dst.write_from(&self.to_be_bytes()[..])?;
                Ok(())
            }
        }
    };
}

impl_for!(usize);
impl_for!(isize);
impl_for!(u8);
impl_for!(i8);
impl_for!(u16);
impl_for!(i16);
impl_for!(u32);
impl_for!(i32);
impl_for!(u64);
impl_for!(i64);
#[cfg(feature = "i128")]
impl_for!(u128);
#[cfg(feature = "i128")]
impl_for!(i128);

impl_for!(f32);
impl_for!(f64);

#[cfg(test)]
mod tests {
    use crate::test::*;

    macro_rules! round_trip {
        ($ty:ty) => {
            hint_round_trip(&<$ty>::MIN);
            hint_round_trip(&(0 as $ty));
            hint_round_trip(&(1 as $ty));
            hint_round_trip(&(2 as $ty));
            hint_round_trip(&<$ty>::MAX);
        };
    }

    #[test]
    fn round_trip() {
        round_trip!(usize);
        round_trip!(isize);
        round_trip!(u8);
        round_trip!(i8);
        round_trip!(u16);
        round_trip!(i16);
        round_trip!(u32);
        round_trip!(i32);
        round_trip!(u64);
        round_trip!(i64);
        #[cfg(feature = "i128")]
        {
            round_trip!(u128);
            round_trip!(i128);
        }

        round_trip!(f32);
        round_trip!(f64);
    }
}
