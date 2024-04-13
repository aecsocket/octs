use core::fmt::Display;

use crate::BufError;

/// Attempted to deserialize a primitive value, but the byte pattern read was
/// not valid for the given type.
///
/// This error's meaning is dependent on which primitive type you are reading:
/// * [`bool`]: read a [`u8`] which was not `0x0` or `0x1`, the two valid values
///   which can represent a boolean.
/// * `NonZero*`: read a value of `0`, which, unsurprising, is invalid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidValue(pub(crate) ());

impl Display for InvalidValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid value")
    }
}

impl BufError for InvalidValue {}

#[cfg(feature = "std")]
impl std::error::Error for InvalidValue {}
