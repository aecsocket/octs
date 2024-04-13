use bytes::Buf;

use crate::{BufTooShort, BufTooShortOr};

pub trait Write {
    #[must_use]
    fn rem_mut(&self) -> usize;

    fn write_from(&mut self, src: impl Buf) -> Result<(), BufTooShort>;

    #[inline]
    fn write<T: Encode>(&mut self, value: T) -> Result<(), BufTooShortOr<T::Error>>
    where
        Self: Sized,
    {
        value.encode(self)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn writer(self) -> crate::std_io::Writer<Self>
    where
        Self: Sized,
    {
        crate::std_io::Writer::new(self)
    }
}

pub trait Encode {
    type Error;

    fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>>;
}

impl<T: Encode + ?Sized> Encode for &T {
    type Error = T::Error;

    fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        (**self).encode(dst)
    }
}

pub trait EncodeLen {
    fn encode_len(&self) -> usize;
}

pub trait MaxEncodeLen {
    const MAX_ENCODE_LEN: usize;
}

pub trait FixedEncodeLen {
    const ENCODE_LEN: usize;
}

impl<T: FixedEncodeLen> EncodeLen for T {
    fn encode_len(&self) -> usize {
        Self::ENCODE_LEN
    }
}

impl<T: FixedEncodeLen> MaxEncodeLen for T {
    const MAX_ENCODE_LEN: usize = Self::ENCODE_LEN;
}
