use std::io;

use crate::{Read, Write};

/// Allows using a [`Read`] as an [`io::Read`].
///
/// Use [`Read::reader`] to create one.
#[derive(Debug, Clone)]
pub struct Reader<T>(T);

impl<T> Reader<T> {
    pub(super) fn new(t: T) -> Self {
        Self(t)
    }

    /// Gets a reference to the inner [`Read`] in this wrapper.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Takes the inner [`Read`] out of this wrapper.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Read> io::Read for Reader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let chunk = self.0.chunk();
        let n = chunk.len().min(buf.len());
        buf.copy_from_slice(&chunk[..n]);
        Ok(n)
    }
}

/// Allows using a [`Write`] as an [`io::Write`].
///
/// Use [`Write::writer`] to create one.
#[derive(Debug, Clone)]
pub struct Writer<T>(T);

impl<T> Writer<T> {
    pub(super) fn new(t: T) -> Self {
        Self(t)
    }

    /// Gets a reference to the inner [`Write`] in this wrapper.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Takes the inner [`Write`] out of this wrapper.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Write> io::Write for Writer<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.0.remaining_mut().min(buf.len());
        self.0
            .write_from(&buf[..n])
            .expect("we only wrote as many bytes as we can fit");
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
