use std::num::NonZeroU64;

use bytes::BytesMut;
use octs::{Read, VarInt, Write};

fn h(buf: &mut impl Read) -> Result<(), octs::BufTooShort> {
    let x = buf.read::<u16>();
    buf.read::<u16>()?;
    Ok(())
}
