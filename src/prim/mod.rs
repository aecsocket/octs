//! Implementations of [`Decode`] and [`Encode`] for primitive types.
//!
//! [`Decode`]: crate::Decode
//! [`Encode`]: crate::Encode

mod bool;
mod error;
mod nonzero;
mod num;
mod zero_sized;

pub use error::*;
