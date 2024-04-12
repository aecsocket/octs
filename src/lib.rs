#![cfg_attr(any(nightly, docsrs), feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod prim;

mod error;
// mod impl_bytes;
mod read;
// mod varint;
mod write;

#[cfg(feature = "std")]
mod std_io;

pub use {error::*, read::*, varint::*, write::*};
