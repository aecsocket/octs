use {
    crate::{BufTooShort, BufTooShortOr},
    bytes::{Buf, Bytes},
};

/// Allows reading bytes from a buffer.
///
/// This is effectively a thin wrapper around a [`Buf`] to add fallible
/// operations. See [`Buf`]'s documentation for more details.
pub trait Read: Buf {
    /// Attempts to advance the cursor of this buffer by `n` bytes.
    ///
    /// # Errors
    ///
    /// Errors if there are less than `n` bytes remaining in the buffer.
    fn skip(&mut self, n: usize) -> Result<(), BufTooShort> {
        if self.remaining() >= n {
            self.advance(n);
            Ok(())
        } else {
            Err(BufTooShort)
        }
    }

    /// Attempts to read the next `n` bytes into a [`Bytes`].
    ///
    /// This may be specialized by the implementation to avoid copying memory.
    ///
    /// If you know the number of bytes you will read at compile time, prefer
    /// using [`Read::read_exact`].
    ///
    /// # Errors
    ///
    /// Errors if there are less than `n` bytes remaining in the buffer.
    fn read_next(&mut self, n: usize) -> Result<Bytes, BufTooShort> {
        if n <= self.remaining() {
            Ok(self.copy_to_bytes(n))
        } else {
            Err(BufTooShort)
        }
    }

    /// Attempts to read the next `N` bytes, and returns a copied array of those
    /// bytes.
    ///
    /// You should prefer this over [`Read::read_next`] where possible.
    ///
    /// # Errors
    ///
    /// Errors if there are less than `N` bytes remaining in the buffer.
    #[inline]
    fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], BufTooShort> {
        // if we can read N items of the next chunk contiguously, try to do so
        if let Some(array) = self.chunk().first_chunk::<N>() {
            let array = *array;
            self.advance(N);
            return Ok(array);
        }
        // else we have to buffer the bytes up, and return this temp buf
        if self.remaining() < N {
            return Err(BufTooShort);
        }
        let mut buf = [0u8; N];
        let mut i = 0;
        while i < N {
            let chunk = self.chunk();
            let to_copy = chunk.len().min(N - i);
            buf[i..(i + to_copy)].copy_from_slice(&chunk[..to_copy]);
            i += to_copy;
            self.skip(to_copy)?;
        }
        Ok(buf)
    }

    /// Attempts to read the next `T` in the buffer.
    ///
    /// # Errors
    ///
    /// Errors if there are not enough bytes remaining in the buffer, or if a
    /// value of `T` could not be read from the bytes in the buffer.
    #[inline]
    fn read<T: Decode>(&mut self) -> Result<T, BufTooShortOr<T::Error>>
    where
        Self: Sized,
    {
        T::decode(self)
    }
}

impl<T: Buf + ?Sized> Read for T {}

/// Allows reading a value of this type from a [`Read`].
pub trait Decode: Sized {
    /// Error type of [`Decode::decode`], excluding [`BufTooShort`] errors.
    type Error;

    /// Attempts to decode a value of this type from a [`Read`].
    ///
    /// # Errors
    ///
    /// If there are not enough bytes in `src` to read a value of this type,
    /// [`BufTooShortOr::TooShort`] is returned. Otherwise, it is up to the
    /// implementation on what the returned error represents.
    fn decode(src: impl Read) -> Result<Self, BufTooShortOr<Self::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip() {
        fn test_with(mut buf: impl Read) {
            assert_eq!(4, buf.remaining());

            buf.skip(1).unwrap();
            assert_eq!(3, buf.remaining());

            buf.skip(2).unwrap();
            assert_eq!(1, buf.remaining());

            buf.skip(2).unwrap_err();
            assert_eq!(1, buf.remaining());

            buf.skip(1).unwrap();
            assert_eq!(0, buf.remaining());
            assert!(!buf.has_remaining());
        }

        test_with(&[1, 2, 3, 4][..]);
        test_with(Bytes::from_static(&[1, 2, 3, 4]));
    }

    #[test]
    fn read_next() {
        fn test_with(mut buf: impl Read) {
            assert_eq!(4, buf.remaining());

            let read = buf.read_next(1).unwrap();
            assert_eq!(Bytes::from_static(&[1]), read);
            assert_eq!(3, buf.remaining());

            let read = buf.read_next(2).unwrap();
            assert_eq!(Bytes::from_static(&[2, 3]), read);
            assert_eq!(1, buf.remaining());

            buf.read_next(2).unwrap_err();
            assert_eq!(1, buf.remaining());

            let read = buf.read_next(1).unwrap();
            assert_eq!(Bytes::from_static(&[4]), read);
            assert_eq!(0, buf.remaining());
            assert!(!buf.has_remaining());
        }

        test_with(&[1, 2, 3, 4][..]);
        test_with(Bytes::from_static(&[1, 2, 3, 4]));
    }

    #[test]
    fn read_exact() {
        fn test_with(mut buf: impl Read) {
            assert_eq!(4, buf.remaining());

            assert_eq!([1], buf.read_exact::<1>().unwrap());
            assert_eq!(3, buf.remaining());

            assert_eq!([2, 3], buf.read_exact::<2>().unwrap());
            assert_eq!(1, buf.remaining());

            buf.read_exact::<2>().unwrap_err();
            assert_eq!(1, buf.remaining());

            assert_eq!([4], buf.read_exact::<1>().unwrap());
            assert_eq!(0, buf.remaining());
            assert!(!buf.has_remaining());
        }

        // contiguous
        test_with(&[1, 2, 3, 4][..]);
        test_with(Bytes::from_static(&[1, 2, 3, 4]));

        // chained / non-contiguous
        test_with([1, 2].chain(&[3, 4][..]));
        test_with(Bytes::from_static(&[1, 2]).chain(Bytes::from_static(&[3, 4])));
    }
}
