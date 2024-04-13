use core::{convert::Infallible, fmt::Display};

/// Performed an operation on a [`Read`] or [`Write`] which required more bytes
/// available than were actually available.
///
/// For [`Read`] operations, this means that there were not enough bytes left in
/// the buffer to read a value of a certain type.
///
/// For [`Write`] operations, this means that there was not enough space left in
/// the buffer to write a certain number of bytes. Note, however, that some
/// values (notably [`Bytes`]) may allow writing a [`usize::MAX`] number of
/// bytes by reallocating their internal buffer, and therefore will never return
/// this error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufTooShort;

impl Display for BufTooShort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "buffer too short")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BufTooShort {}

/// Error which may represent either a [`BufTooShort`] or some other
/// user-specified error type.
///
/// It is recommended that `E` implements [`BufError`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufTooShortOr<E> {
    /// See [`BufTooShort`].
    TooShort,
    /// Alternative error type.
    Or(E),
}

impl<E> BufTooShortOr<E> {
    /// Maps [`BufTooShortOr::Or`] to a new value using the given mapper
    /// function.
    pub fn map_or<F>(self, op: impl FnOnce(E) -> F) -> BufTooShortOr<F> {
        match self {
            Self::TooShort => BufTooShortOr::TooShort,
            Self::Or(e) => BufTooShortOr::Or(op(e)),
        }
    }
}

impl<E: Display> Display for BufTooShortOr<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => write!(f, "{BufTooShort}"),
            Self::Or(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error + 'static> std::error::Error for BufTooShortOr<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::TooShort => None,
            Self::Or(err) => Some(err),
        }
    }
}

/// Marker trait for an error type which can be used as the `E` parameter in
/// [`BufTooShortOr`].
///
/// This enables the blanket `impl`s:
/// * `impl<E: BufError> From<E> for BufTooShortOr<E>`
/// * `impl<E: BufError> From<BufTooShortOr<Infallible>> for BufTooShortOr<E>`
pub trait BufError {}

impl<E> From<BufTooShort> for BufTooShortOr<E> {
    fn from(_: BufTooShort) -> Self {
        Self::TooShort
    }
}

impl<E: BufError> From<E> for BufTooShortOr<E> {
    fn from(value: E) -> Self {
        Self::Or(value)
    }
}

impl From<BufTooShortOr<Infallible>> for BufTooShort {
    fn from(value: BufTooShortOr<Infallible>) -> Self {
        match value {
            BufTooShortOr::TooShort => Self,
            BufTooShortOr::Or(_) => unreachable!(),
        }
    }
}

impl<E: BufError> From<BufTooShortOr<Infallible>> for BufTooShortOr<E> {
    fn from(value: BufTooShortOr<Infallible>) -> Self {
        value.map_or(|_| unreachable!())
    }
}
