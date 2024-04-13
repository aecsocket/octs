use core::{convert::Infallible, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufTooShort;

impl Display for BufTooShort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "buffer too short")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BufTooShort {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufTooShortOr<E> {
    TooShort,
    Or(E),
}

impl<E> From<BufTooShort> for BufTooShortOr<E> {
    fn from(_: BufTooShort) -> Self {
        Self::TooShort
    }
}

impl<E> From<Infallible> for BufTooShortOr<E> {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl<E: Display> Display for BufTooShortOr<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooShort => write!(f, "{}", BufTooShort),
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
