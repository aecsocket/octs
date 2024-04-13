use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

macro_rules! impl_for {
    ($ty:ty) => {
        impl FixedEncodeLen for $ty {
            const ENCODE_LEN: usize = std::mem::size_of::<$ty>();
        }

        impl Decode for $ty {
            type Error = Infallible;

            #[inline]
            fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
                Ok(<$ty>::from_be_bytes(src.read_exact()?))
            }
        }

        impl Encode for $ty {
            type Error = Infallible;

            #[inline]
            fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
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
