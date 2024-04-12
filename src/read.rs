use crate::BufferTooShort;

pub trait Read {
    #[must_use]
    fn chunk(&self) -> &[u8];

    #[must_use]
    fn rem(&self) -> usize {
        self.chunk().len()
    }

    #[inline]
    fn read_next(&mut self, n: usize) -> Result<&[u8], BufferTooShort> {
        let chunk = (*self).chunk();
        if chunk.len() < n {
            return Err(BufferTooShort);
        }
        Ok(&chunk[..n])
    }

    #[inline]
    fn read_exact<const N: usize>(&mut self) -> Result<&[u8; N], BufferTooShort> {
        let next = self.read_next(N)?;
        Ok(<&[u8; N]>::try_from(next).expect("slice should be N bytes long"))
    }

    #[inline]
    fn read<T: Decode>(&mut self) -> Result<T, T::Error>
    where
        Self: Sized,
    {
        T::decode(self)
    }

    #[cfg(feature = "std")]
    #[inline]
    fn reader(self) -> crate::std_io::Reader<Self>
    where
        Self: Sized,
    {
        crate::std_io::Reader::new(self)
    }
}

impl<T: Read + ?Sized> Read for &mut T {
    #[inline]
    fn chunk(&self) -> &[u8] {
        (**self).chunk()
    }
}

pub trait Decode: Sized {
    type Error;

    fn decode(src: impl Read) -> Result<Self, Self::Error>;
}
