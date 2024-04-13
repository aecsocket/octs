# `octs`

[![crates.io](https://img.shields.io/crates/v/octs.svg)](https://crates.io/crates/octs)
[![docs.rs](https://img.shields.io/docsrs/octs)](https://docs.rs/octs)

Finally, a good byte manipulation library.

This crate builds on top of the types defined by [`bytes`] by replacing its panicking `get` and
`put` functions with fallible, non-panicking `read` and `write` functions via [`octs::Read`] and
[`octs::Write`].

## Features

* **Based on [`bytes`]** - which provides useful types for byte manipulation, and allows cheaply
  cloning byte allocations via reference counting. Great for writing zero-copy networking code.

* **Panicking functions were a mistake** - in networking, you can't trust your inputs. So why should
  it ever be possible to panic on malformed input?? All functions in `octs` which can fail return a
  [`Result`].

* **Your types are first-class citizens** - instead of `get_u16`, `put_f32`, etc., just use one
  [`read`] and [`write`] function for all types. This means you can implement [`Decode`] and be able
  to [`read`] it from any buffer, and likewise for [`Encode`] and [`write`].

* **Dedicated varints** - one of the staples of networking primitives is implemented here, without
  needing any extensions. Just `read` or `write` a [`VarInt`] as you would any other value.

* **Zero unsafe** - I'm not smart enough to write unsafe code.

* `#![no_std]` - just like [`bytes`], but it still requires `alloc`.

```rust
use core::num::NonZeroU16;

use octs::{Read, Write, VarInt, Buf};

fn handle_packet(mut buf: octs::Bytes) -> Result<(), octs::BufTooShort> {
    //                    ^^^^^^^^^^^                ^^^^^^^^^^^^^^^^^
    //                    |                      the main error type |
    //                    | `octs` re-exports the core `bytes` types

    let packet_id = buf.read::<u16>()?;
    let timestamp = buf.read::<u64>()?;
    // just use ? for error handling ^
    //                    no panics! |

    let body = match buf.read::<PacketBody>() {
        //               ^^^^^^^^^^^^^^^^^^
        //               | `read` your own types in directly -
        //               | they are just as important as `u8` or `u16`
        // and if you need some custom error handling
        // outside of "buffer too short",
        // we've got you covered
        Ok(body) => body,
        Err(octs::BufTooShortOr::TooShort) => return Err(octs::BufTooShort),
        Err(octs::BufTooShortOr::Or(err)) => {
            // please don't actually panic in real code; this is just so we
            // don't have to return a value from this `match`
            panic!("invalid packet received: {err:?}");
        }
    };

    Ok(())
}

struct PacketBody {
    payload: octs::Bytes,
}

// define your own error type for operations
#[derive(Debug)]
enum InvalidPacketError {
    LengthTooLarge(octs::VarIntTooLarge),
}

// make sure to implement this trait on your error type,
// for better ergonomics
impl octs::BufError for InvalidPacketError {}

// implement `Decode` to be able to `read` it from a buffer
impl octs::Decode for PacketBody {
    // and use your own error type if you want
    type Error = InvalidPacketError;

    fn decode(src: &mut impl Read) -> Result<Self, octs::BufTooShortOr<Self::Error>> {
        let VarInt(len) = src.read::<VarInt<usize>>()
            .map_err(|err| err.map_or(InvalidPacketError::LengthTooLarge))?;
        //  ^^^^^^^^^^^
        //  | decode VarInts directly from a buffer
        let payload: octs::Bytes = src.read_next(len)?;
        //           ^^^^^^^^^^^
        //           | read the next `len` bytes into a `Bytes`
        //           | if `src` is also a `Bytes`, this is a zero-copy operation
        Ok(Self { payload })
    }
}

// same for `Encode` and `write`
impl octs::Encode for PacketBody {
    type Error = core::convert::Infallible;

    fn encode(&self, dst: &mut impl Write) -> Result<(), octs::BufTooShortOr<Self::Error>> {
        dst.write(VarInt(self.payload.len()))?;
        dst.write_from(&mut self.payload.chunk())?;
        Ok(())
    }
}
```

## Inspirations

* [`bytes`] - core byte manipulation primitives, such as the possibly-non-contiguous [`bytes::Buf`]
  trait, and the cheaply-cloneable [`bytes::Bytes`] type.
* [`octets`] - general API style, and having varints be a core part of the API
* [`safer-bytes`] - making a good version of the [`bytes`] API
* [`integer-encoding`] - implementations of varint encode/decode

[`octs::Read`]: Read
[`octs::Write`]: Write
[`read`]: Read::read
[`write`]: Write::write
[`bytes`]: https://docs.rs/bytes
[`octets`]: https://docs.rs/octets
[`integer-encoding`]: https://docs.rs/integer-encoding
[`safer-bytes`]: https://docs.rs/safer-bytes
