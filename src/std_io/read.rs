use std::io;

use crate::Read;

/// Allows using a [`Read`] as an [`io::Read`].
///
/// Use [`Read::reader`] to create one.
#[derive(Debug, Clone)]
pub struct Reader<T>(T);

impl<T> Reader<T> {
    pub(crate) fn new(t: T) -> Self {
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
