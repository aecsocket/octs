use core::convert::Infallible;

use crate::{BufTooShortOr, Decode, Encode, FixedEncodeLen, Read, Write};

use super::InvalidValue;

impl FixedEncodeLen for bool {
    const ENCODE_LEN: usize = 1;
}

impl Decode for bool {
    type Error = InvalidValue;

    #[inline]
    fn decode(src: &mut impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        match src.read_exact::<1>()? {
            [0] => Ok(false),
            [1] => Ok(true),
            [_] => Err(InvalidValue(()).into()),
        }
    }
}

impl Encode for bool {
    type Error = Infallible;

    #[inline]
    fn encode(&self, dst: &mut impl Write) -> Result<(), BufTooShortOr<Self::Error>> {
        dst.write(&u8::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use super::*;

    fn round_trip(value: bool) {
        let mut buf = BytesMut::new();
        buf.write(value).unwrap();
        let mut buf = buf.freeze();
        assert_eq!(value, buf.read::<bool>().unwrap());
    }

    #[test]
    fn round_trip_false() {
        round_trip(false);
    }

    #[test]
    fn round_trip_true() {
        round_trip(true);
    }

    #[test]
    fn decode_invalid() {
        (&[2u8][..]).read::<bool>().unwrap_err();
    }
}
