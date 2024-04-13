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
    /// let mut chunks = Bytes::from(vec![1, 2, 3, 4]).byte_chunks(2);
    /// assert_eq!(&[1, 2], &*chunks.next().unwrap());
    /// assert_eq!(&[3, 4], &*chunks.next().unwrap());
    /// assert_eq!(None, chunks.next());
    /// ```
    ///
    /// With a remainder:
    ///
    /// ```
    /// # use octs::{Bytes, chunks::ByteChunksExt};
    /// let mut chunks = Bytes::from(vec![1, 2, 3, 4, 5]).byte_chunks(2);
    /// assert_eq!(&[1, 2], &*chunks.next().unwrap());
    /// assert_eq!(&[3, 4], &*chunks.next().unwrap());
    /// assert_eq!(&[5], &*chunks.next().unwrap());
    /// assert_eq!(None, chunks.next());
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

impl Iterator for ByteChunks {
    type Item = Bytes;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            return None;
        }

        let mid = self.buf.len().min(self.chunk_len);
        let next = self.buf.split_to(mid);
        Some(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.buf.len() / self.chunk_len;
        let rem = self.buf.len() % self.chunk_len;
        let n = if rem > 0 { n + 1 } else { n };
        (n, Some(n))
    }

    #[inline]
    fn count(self) -> usize {
        self.len()
    }
}

impl ExactSizeIterator for ByteChunks {}

impl FusedIterator for ByteChunks {}
