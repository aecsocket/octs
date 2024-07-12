//! Allows splitting a byte buffer into non-overlapping chunks of bytes.

use core::iter::FusedIterator;

use bytes::Bytes;

/// Extension trait on [`Read`] providing [`byte_chunks`].
///
/// [`Read`]: crate::Read
/// [`byte_chunks`]: ByteChunksExt::byte_chunks
pub trait ByteChunksExt: Sized {
    /// Type of chunk iterator returned by [`ByteChunksExt::byte_chunks`].
    type ByteChunks;

    /// Converts this into an iterator over non-overlapping chunks of the
    /// original bytes.
    ///
    /// # Examples
    ///
    /// With `len` being a multiple of `chunk_len`:
    ///
    /// ```
    /// # use octs::{Bytes, chunks::ByteChunksExt};
    /// let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
    /// assert_eq!(&[1, 2], &*chunks.next().unwrap());
    /// assert_eq!(&[3, 4], &*chunks.next().unwrap());
    /// assert!(chunks.next().is_none());
    /// ```
    ///
    /// With a remainder:
    ///
    /// ```
    /// # use octs::{Bytes, chunks::ByteChunksExt};
    /// let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
    /// assert_eq!(&[1, 2], &*chunks.next().unwrap());
    /// assert_eq!(&[3, 4], &*chunks.next().unwrap());
    /// assert_eq!(&[5], &*chunks.next().unwrap());
    /// assert!(chunks.next().is_none());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `chunk_len` is 0.
    fn byte_chunks(self, chunk_len: usize) -> Self::ByteChunks;
}

impl<'a> ByteChunksExt for &'a [u8] {
    type ByteChunks = core::slice::Chunks<'a, u8>;

    fn byte_chunks(self, chunk_len: usize) -> Self::ByteChunks {
        self.chunks(chunk_len)
    }
}

impl ByteChunksExt for Bytes {
    type ByteChunks = ByteChunks;

    fn byte_chunks(self, chunk_len: usize) -> Self::ByteChunks {
        ByteChunks {
            buf: self,
            chunk_len,
        }
    }
}

/// Iterator over [`Bytes`] of non-overlapping chunks, with each chunk being of
/// the same length.
///
/// The last item returned may not be of the same length as other items, as it
/// may return the remaining items.
///
/// Each [`Bytes`] returned is owned by the consumer, which is done by creating
/// a cheap clone of the underlying [`Bytes`], which just increases a reference
/// count and changes some indices.
///
/// Use [`byte_chunks`] to create.
///
/// See [`Chunks`].
///
/// [`byte_chunks`]: ByteChunksExt::byte_chunks
/// [`Chunks`]: core::slice::Chunks
#[derive(Debug)]
pub struct ByteChunks {
    buf: Bytes,
    chunk_len: usize,
}

// impls copied from std::slice::Chunks
impl Iterator for ByteChunks {
    type Item = Bytes;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let chunksz = self.buf.len().min(self.chunk_len);
            let next = self.buf.split_to(chunksz);
            Some(next)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.buf.is_empty() {
            (0, Some(0))
        } else {
            let n = self.buf.len() / self.chunk_len;
            let rem = self.buf.len() % self.chunk_len;
            let n = if rem > 0 { n + 1 } else { n };
            (n, Some(n))
        }
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, overflow) = n.overflowing_mul(self.chunk_len);
        if start >= self.buf.len() || overflow {
            self.buf = Bytes::new();
            None
        } else {
            let end = match start.checked_add(self.chunk_len) {
                Some(sum) => core::cmp::min(self.buf.len(), sum),
                None => self.buf.len(),
            };
            let nth = self.buf.slice(start..end);
            self.buf = self.buf.slice(end..);
            Some(nth)
        }
    }

    #[inline]
    fn last(self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let start = (self.buf.len() - 1) / self.chunk_len * self.chunk_len;
            Some(self.buf.slice(start..))
        }
    }
}

impl DoubleEndedIterator for ByteChunks {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let remainder = self.buf.len() % self.chunk_len;
            let chunksz = if remainder != 0 {
                remainder
            } else {
                self.chunk_len
            };

            let last = self.buf.split_off(self.buf.len() - chunksz);
            Some(last)
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let len = self.len();
        if n >= len {
            self.buf = Bytes::new();
            None
        } else {
            let start = (len - 1 - n) * self.chunk_len;
            let end = match start.checked_add(self.chunk_len) {
                Some(res) => core::cmp::min(self.buf.len(), res),
                None => self.buf.len(),
            };
            let nth_back = self.buf.slice(start..end);
            self.buf = self.buf.slice(..start);
            Some(nth_back)
        }
    }
}

impl ExactSizeIterator for ByteChunks {}

impl FusedIterator for ByteChunks {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::iter_nth_zero)] // this is what we're testing
    fn nth() {
        let mut chunks = Bytes::from_static(&[]).byte_chunks(2);
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());
        assert!(chunks.nth(2).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.nth(0).unwrap());
        assert_eq!(&[3, 4], &*chunks.nth(0).unwrap());
        assert_eq!(&[5], &*chunks.nth(0).unwrap());
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[3, 4], &*chunks.nth(1).unwrap());
        assert_eq!(&[5], &*chunks.nth(0).unwrap());
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[5], &*chunks.nth(2).unwrap());
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.nth(0).unwrap());
        assert_eq!(&[3, 4], &*chunks.nth(0).unwrap());
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
        assert_eq!(&[3, 4], &*chunks.nth(1).unwrap());
        assert!(chunks.nth(0).is_none());
        assert!(chunks.nth(1).is_none());
    }

    #[test]
    fn next_back() {
        let mut chunks = Bytes::from_static(&[]).byte_chunks(2);
        assert!(chunks.next().is_none());

        let mut chunks = Bytes::from_static(&[1]).byte_chunks(2);
        assert_eq!(&[1], &*chunks.next_back().unwrap());
        assert!(chunks.next().is_none());

        let mut chunks = Bytes::from_static(&[1, 2]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.next_back().unwrap());
        assert!(chunks.next().is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
        assert_eq!(&[3, 4], &*chunks.next_back().unwrap());
        assert_eq!(&[1, 2], &*chunks.next_back().unwrap());
        assert!(chunks.next().is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[5], &*chunks.next_back().unwrap());
        assert_eq!(&[3, 4], &*chunks.next_back().unwrap());
        assert_eq!(&[1, 2], &*chunks.next_back().unwrap());
        assert!(chunks.next().is_none());
    }

    #[test]
    fn nth_back() {
        let mut chunks = Bytes::from_static(&[]).byte_chunks(2);
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1]).byte_chunks(2);
        assert_eq!(&[1], &*chunks.nth_back(0).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.nth_back(0).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
        assert_eq!(&[3, 4], &*chunks.nth_back(0).unwrap());
        assert_eq!(&[1, 2], &*chunks.nth_back(0).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.nth_back(1).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[5], &*chunks.nth_back(0).unwrap());
        assert_eq!(&[3, 4], &*chunks.nth_back(0).unwrap());
        assert_eq!(&[1, 2], &*chunks.nth_back(0).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[3, 4], &*chunks.nth_back(1).unwrap());
        assert_eq!(&[1, 2], &*chunks.nth_back(0).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());

        let mut chunks = Bytes::from_static(&[1, 2, 3, 4, 5]).byte_chunks(2);
        assert_eq!(&[1, 2], &*chunks.nth_back(2).unwrap());
        assert!(chunks.nth_back(0).is_none());
        assert!(chunks.nth_back(1).is_none());
    }
}
