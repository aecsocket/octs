use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

impl FixedEncodeLen for () {
    const ENCODE_LEN: usize = 0;
}

impl Decode for () {
    type Error = Infallible;

    fn decode(_: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        Ok(())
    }
}

impl Encode for () {
    type Error = Infallible;

    fn encode(&self, _: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn round_trip() {
        crate::__test::round_trip(());
    }
}
