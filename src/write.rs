use bytes::{Buf, BufMut};

use crate::{BufTooShort, BufTooShortOr};

/// Allows writing bytes into a buffer.
///
/// This is effectively a thin wrapper around a [`BufMut`] to add fallible
/// operations. See [`BufMut`]'s documentation for more details.
pub trait Write: BufMut {
    /// Attempts to write into this buffer from `src`.
    ///
    /// # Errors
    ///
    /// Errors if `src` has more bytes than `self` has room for.
    #[inline]
    fn write_from(&mut self, src: impl Buf) -> Result<(), BufTooShort>
    where
        Self: Sized,
    {
        if self.remaining_mut() >= src.remaining() {
            self.put(src);
            Ok(())
        } else {
            Err(BufTooShort)
        }
    }

    /// Attempts to write a `T` into the next bytes in the buffer.
    ///
    /// # Errors
    ///
    /// Errors if there are not enough bytes in this buffer left for writing
    /// into, or if `value` could not be encoded into bytes.
    #[inline]
    fn write<T: Encode>(&mut self, value: T) -> Result<(), BufTooShortOr<T::Error>>
    where
        Self: Sized,
    {
        value.encode(self)
    }
}

impl<T: BufMut + ?Sized> Write for T {}

/// Allows writing a value of this type into a [`Write`].
pub trait Encode {
    /// Error type of [`Encode::encode`], excluding [`BufTooShort`] errors.
    type Error;

    /// Attempts to encode a value of this type into a [`Write`].
    ///
    /// # Errors
    ///
    /// If there are not enough bytes left for writing into,
    /// [`BufTooShortOr::TooShort`] is returned. Otherwise, it is up to the
    /// implementation on what the returned error represents.
    fn encode(&self, dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>>;
}

impl<T: Encode + ?Sized> Encode for &T {
    type Error = T::Error;

    #[inline]
    fn encode(&self, dst: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        (**self).encode(dst)
    }
}

/// Gets how many bytes it takes to encode a value of this type.
pub trait EncodeLen: Encode {
    /// Gets how many bytes it takes to encode this value into a [`Write`].
    ///
    /// If this function returns `n`, then if you [`Encode::encode`] this value
    /// into a buffer, it is guaranteed that `n` bytes will be consumed.
    fn encode_len(&self) -> usize;
}

/// Provides hints on how many bytes it may take to encode a value of this type.
pub trait FixedEncodeLenHint: EncodeLen {
    /// Inclusive minimum value that [`EncodeLen::encode_len`] may be.
    const MIN_ENCODE_LEN: usize;

    /// Inclusive maximum value that [`EncodeLen::encode_len`] may be.
    const MAX_ENCODE_LEN: usize;
}

/// Defines exactly how many bytes it will take to encode a value of this type.
pub trait FixedEncodeLen {
    /// How many bytes it takes to encode a value of this type.
    ///
    /// This is the value that [`EncodeLen::encode_len`] will always return for
    /// this type.
    const ENCODE_LEN: usize;
}

impl<T: FixedEncodeLen> FixedEncodeLenHint for T {
    const MIN_ENCODE_LEN: usize = Self::ENCODE_LEN;

    const MAX_ENCODE_LEN: usize = Self::ENCODE_LEN;
}

impl<T: FixedEncodeLen> EncodeLen for T {
    #[inline]
    fn encode_len(&self) -> usize {
        Self::ENCODE_LEN
    }
}
