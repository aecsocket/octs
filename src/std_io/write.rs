use std::io;

use crate::Write;

/// Allows using a [`Write`] as an [`io::Write`].
///
/// Use [`Write::writer`] to create one.
#[derive(Debug, Clone)]
pub struct Writer<T>(T);

impl<T> Writer<T> {
    pub(crate) fn new(t: T) -> Self {
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
