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

## Examples

### Writing

```rust
use octs::{Read, Write, VarInt};

fn write_packet(
    mut buf: octs::BytesMut,
    //       ^^^^^^^^^^^^^^
    //       | re-exports the core `bytes` types
    packet_id: u16,
    timestamp: u64,
    payload: &[u8],
) -> Result<(), octs::BufTooShort> {
    //          ^^^^^^^^^^^^^^^^^
    //          | the main error type
    buf.write(packet_id)?;
    //  ^^^^^
    //  | one `write` function for all your types

    buf.write(timestamp)?;
    //  +---------------^
    //  | just use ? for errors
    //  | no panics

    buf.write(VarInt(payload.len()))?;
    //       ^^^^^^^
    //       | inbuilt support for varints
    //       | using the Protocol Buffers spec

    buf.write_from(payload)?;
    //  ^^^^^^^^^^
    //  | copy from an existing buffer 

    Ok(())
}
```

### Reading

```rust
use core::num::NonZeroU8;

use octs::{Bytes, BufError, Decode, Read, BufTooShortOr, VarInt};

#[derive(Debug)]
struct Fragment {
    num_frags: NonZeroU8,
    payload: Bytes,
}

#[derive(Debug)]
enum FragmentError {
    InvalidNumFrags,
    PayloadTooLarge,
}

impl Decode for Fragment {
//   ^^^^^^
//   | implement this trait to be able to `read`
//   | this value from a buffer

    type Error = FragmentError;

    fn decode(mut buf: impl Read) -> Result<Self, BufTooShortOr<Self::Error>> {
        let num_frags = buf
            .read::<NonZeroU8>()
            .map_err(|e| e.map_or(|_| FragmentError::InvalidNumFrags))?;
        // +--------------^^^^^^^
        // | map the `InvalidValue` error of reading
        // | a `NonZeroU8` to your own error value

        let VarInt(payload_len) = buf
            .read::<VarInt<usize>>()
            .map_err(|e| e.map_or(|_| FragmentError::PayloadTooLarge))?;

        let payload = buf.read_next(payload_len)?;
        // +-------------^^^^^^^^^^
        // | read the next `payload_len` bytes directly into `Bytes`
        // | if `buf` is also a `Bytes`, this is zero-copy!

        Ok(Self {
            num_frags,
            payload
        })
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
