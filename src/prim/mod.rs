mod bool;
mod nonzero;
mod num;

use core::fmt::Display;

use crate::BufferTooShortOr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidValue;

impl Display for InvalidValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid value")
    }
}

impl From<InvalidValue> for BufferTooShortOr<InvalidValue> {
    fn from(value: InvalidValue) -> Self {
        Self::Other(value)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidValue {}
