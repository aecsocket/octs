use core::fmt::Display;

use crate::BufError;

/// Attempted to read a [`VarInt`] from a buffer, but the resulting integer
/// would have been too large to fit into this [`VarInt`].
///
/// When reading a byte of a [`VarInt`], the MSB being set indicates that there
/// is more data left in the integer. If we are decoding e.g. a `VarInt<u32>`,
/// and we read 5 bytes all with their MSB set, then we've read `7 * 5 = 35`
/// bits of actual integer data, and still haven't reached the end of this
/// (supposed) [`u32`], so we abort with this error.
///
/// [`VarInt`]: crate::VarInt
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarIntTooLarge;

impl Display for VarIntTooLarge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "varint too large")
    }
}

impl BufError for VarIntTooLarge {}

#[cfg(feature = "std")]
impl std::error::Error for VarIntTooLarge {}
