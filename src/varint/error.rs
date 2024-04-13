use core::{convert::Infallible, fmt::Display};

use crate::BufTooShortOr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarIntTooLarge;

impl Display for VarIntTooLarge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "varint too large")
    }
}

impl From<VarIntTooLarge> for BufTooShortOr<VarIntTooLarge> {
    fn from(value: VarIntTooLarge) -> Self {
        Self::Or(value)
    }
}

impl From<BufTooShortOr<Infallible>> for BufTooShortOr<VarIntTooLarge> {
    fn from(value: BufTooShortOr<Infallible>) -> Self {
        match value {
            BufTooShortOr::TooShort => Self::TooShort,
            BufTooShortOr::Or(_) => unreachable!(),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VarIntTooLarge {}
