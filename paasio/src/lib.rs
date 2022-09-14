use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    bytes: usize,
    reads: usize,
    data: R,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            bytes: 0,
            reads: 0,
            data: wrapped,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        let bytes = self.data.read(buf)?;
        self.bytes += bytes;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    bytes: usize,
    writes: usize,
    data: W,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            bytes: 0,
            writes: 0,
            data: wrapped,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        let bytes = self.data.write(buf)?;
        self.bytes += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
