use std::io;

use crate::{Read, Write};

#[derive(Debug, Clone)]
pub struct Reader<T>(T);

impl<T> Reader<T> {
    pub(super) fn new(t: T) -> Self {
        Self(t)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

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

#[derive(Debug, Clone)]
pub struct Writer<T>(T);

impl<T> Writer<T> {
    pub(super) fn new(t: T) -> Self {
        Self(t)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Write> io::Write for Writer<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.0.rem_mut().min(buf.len());
        self.0
            .write_from(&buf[..n])
            .expect("we only wrote as many bytes as we can fit");
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
