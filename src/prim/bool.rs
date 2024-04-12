use crate::{BufferTooShortOr, ConstEncodeLen, Decode, Encode, Read, Write};

use super::InvalidValue;

impl ConstEncodeLen for bool {
    const ENCODE_LEN: usize = 1;
}

impl Decode for bool {
    type Error = BufferTooShortOr<InvalidValue>;

    fn decode(mut buf: impl Read) -> Result<Self, Self::Error> {
        match buf.read_exact::<1>()? {
            [0] => Ok(false),
            [1] => Ok(true),
            [_] => Err(InvalidValue.into()),
        }
    }
}

impl Encode for bool {
    fn encode(&self, mut buf: impl Write) -> Result<()> {
        buf.write(if *self { &1u8 } else { &0u8 })
    }
}
