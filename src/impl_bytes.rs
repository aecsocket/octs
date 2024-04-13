use bytes::{Buf, BufMut, Bytes};

use crate::{BufTooShort, Read, Write};

impl<T: Buf> Read for T {
    fn rem(&self) -> usize {
        Buf::remaining(self)
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        Buf::chunk(self)
    }

    fn advance(&mut self, n: usize) -> Result<(), BufTooShort> {
        if Buf::remaining(self) >= n {
            Buf::advance(self, n);
            Ok(())
        } else {
            Err(BufTooShort)
        }
    }

    fn read_next(&mut self, n: usize) -> Result<Bytes, BufTooShort> {
        if n <= Buf::remaining(self) {
            Ok(Buf::copy_to_bytes(self, n))
        } else {
            Err(BufTooShort)
        }
    }
}

impl<T: BufMut> Write for T {
    fn rem_mut(&self) -> usize {
        BufMut::remaining_mut(self)
    }

    fn write_from(&mut self, src: impl Buf) -> Result<(), BufTooShort> {
        if BufMut::remaining_mut(self) >= Buf::remaining(&src) {
            BufMut::put(self, src);
            Ok(())
        } else {
            Err(BufTooShort)
        }
    }
}
