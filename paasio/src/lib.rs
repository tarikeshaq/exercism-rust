use std::io::{Read, Result, Write};

pub struct ReadStats<R>{
    wrapped: R,
    bytes_read: usize,
    num_reads: usize
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes_read: 0,
            num_reads: 0
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.num_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(val) => {
                self.bytes_read += val;
                self.num_reads += 1;
                Ok(val)
            },
            Err(err) => Err(err)
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes_writen: usize,
    num_writes: usize
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            bytes_writen: 0,
            num_writes: 0
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_writen
    }

    pub fn writes(&self) -> usize {
        self.num_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(val) => {
                self.bytes_writen += val;
                self.num_writes += 1;
                Ok(val)
            }, 
            Err(err) => Err(err)
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
