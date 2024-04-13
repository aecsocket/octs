use bytes::Bytes;

use crate::{BufTooShort, BufTooShortOr};

pub trait Read {
    #[must_use]
    fn rem(&self) -> usize;

    #[must_use]
    fn chunk(&self) -> &[u8];

    fn advance(&mut self, n: usize) -> Result<(), BufTooShort>;

    fn read_next(&mut self, n: usize) -> Result<Bytes, BufTooShort>;

    #[inline]
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], BufTooShort> {
        // if we can read N items of the next chunk contiguously, try to do so
        if let Some(array) = self.chunk().first_chunk::<N>() {
            let array = *array;
            self.advance(N).expect("we just read N bytes");
            return Ok(array);
        }
        // else we have to buffer the bytes up, and return this temp buf
        if self.rem() < N {
            return Err(BufTooShort);
        }
        let mut buf = [0u8; N];
        let mut i = 0;
        while i < N {
            let chunk = self.chunk();
            let to_copy = chunk.len().min(N - i);
            buf[i..(i + to_copy)].copy_from_slice(&chunk[..to_copy]);
            i += to_copy;
            self.advance(to_copy)?;
        }
        Ok(buf)
    }

    #[inline]
    fn read<T: Decode>(&mut self) -> Result<T, BufTooShortOr<T::Error>>
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

pub trait Decode: Sized {
    type Error;

    fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>>;
}
