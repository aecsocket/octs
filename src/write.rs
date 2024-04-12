pub trait Write {
    #[must_use]
    fn rem_mut(&self) -> usize;

    fn write_slice(&mut self, src: &[u8]) -> Result<()>;

    #[inline]
    fn write<T: Encode>(&mut self, value: &T) -> Result<()>
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

impl<T: Write + ?Sized> Write for &mut T {
    #[inline]
    fn rem_mut(&self) -> usize {
        (**self).rem_mut()
    }

    #[inline]
    fn write_slice(&mut self, src: &[u8]) -> Result<()> {
        (**self).write_slice(src)
    }
}

pub trait Encode {
    fn encode(&self, dst: impl Write) -> Result<()>;
}

pub trait EncodeLen {
    fn encode_len(&self) -> usize;
}

pub trait ConstEncodeLen {
    const ENCODE_LEN: usize;
}

impl<T: ConstEncodeLen> EncodeLen for T {
    fn encode_len(&self) -> usize {
        Self::ENCODE_LEN
    }
}
