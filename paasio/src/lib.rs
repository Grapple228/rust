use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub type ReadStats<T> = IoStats<T>;
pub type WriteStats<T> = IoStats<T>;

pub struct IoStats<R> {
    pub wrapped: R,
    pub reads: usize,
    pub writes: usize,
    pub bytes_through: usize,
}

impl<T> IoStats<T> {
    pub fn new(wrapped: T) -> IoStats<T> {
        IoStats {
            wrapped,
            reads: 0,
            writes: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }
}

impl<T: Read> IoStats<T> {
    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<T: Write> IoStats<T> {
    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<T: Read> Read for IoStats<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.wrapped.read(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }
}

impl<T: Write> Write for IoStats<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.wrapped.write(buf).map(|bytes| {
            self.bytes_through += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
