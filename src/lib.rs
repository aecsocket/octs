#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use bytes::{self, Buf, BufMut, Bytes, BytesMut};

mod error;
mod read;
mod varint;
mod write;

pub mod chunks;
pub mod prim;
pub mod test;

pub use {error::*, read::*, varint::*, write::*};
