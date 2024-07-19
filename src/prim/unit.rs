use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

impl FixedEncodeLen for () {
    const ENCODE_LEN: usize = 0;
}

impl Decode for () {
    type Error = Infallible;

    #[inline]
    fn decode(_: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        Ok(())
    }
}

impl Encode for () {
    type Error = Infallible;

    #[inline]
    fn encode(&self, _: impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::*;

    #[test]
    fn round_trip() {
        hint_round_trip(&());
    }
}
