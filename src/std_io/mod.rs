//! Wrappers around [`crate::Read`] and [`crate::Write`] for use as
//! [`std::io::Read`] and [`std::io::Write`] values.

mod read;
mod write;

pub use {read::*, write::*};
