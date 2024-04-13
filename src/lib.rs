#![cfg_attr(any(nightly, docsrs), feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub use bytes::{self, Buf, BufMut, Bytes, BytesMut};

mod error;
mod read;
mod varint;
mod write;

pub mod prim;
#[cfg(feature = "std")]
pub mod std_io;

pub use {error::*, read::*, varint::*, write::*};
