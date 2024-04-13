//! Implementations of [`Decode`] and [`Encode`] for primitive types.
//!
//! [`Decode`]: crate::Decode
//! [`Encode`]: crate::Encode

mod bool;
mod error;
mod nonzero;
mod num;
mod unit;

pub use error::*;
