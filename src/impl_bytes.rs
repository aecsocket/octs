use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::{Read, Write};

impl Read for Bytes {
    #[inline]
    fn chunk(&self) -> &[u8] {
        Buf::chunk(self)
    }
}

impl Read for BytesMut {
    #[inline]
    fn chunk(&self) -> &[u8] {
        Buf::chunk(self)
    }
}

impl Write for BytesMut {
    fn rem_mut(&self) -> usize {
        BufMut::remaining_mut(self)
    }

    fn write_slice(&mut self, src: &[u8]) -> crate::Result<()> {
        BufMut::put(self, src)
    }
}
