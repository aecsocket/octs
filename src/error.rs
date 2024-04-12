use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferTooShort;

impl Display for BufferTooShort {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "buffer too short")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BufferTooShort {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferTooShortOr<E> {
    Short,
    Other(E),
}

impl<E> From<BufferTooShort> for BufferTooShortOr<E> {
    fn from(_: BufferTooShort) -> Self {
        Self::Short
    }
}

impl<E: Display> Display for BufferTooShortOr<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Short => write!(f, "{}", BufferTooShort),
            Self::Other(err) => write!(f, "{err}"),
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> std::error::Error for BufferTooShortOr<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Short => None,
            Self::Other(err) => Some(err),
        }
    }
}
