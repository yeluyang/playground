use std::io::{self, Read, Seek, SeekFrom, Write};

#[cfg(test)]
mod tests;

pub struct Buffer<B> {
    inner: B,
    cursor: usize,
}

impl<B: Seek> Buffer<B> {
    pub fn new(mut inner: B) -> io::Result<Buffer<B>> {
        let cursor = inner.seek(SeekFrom::Start(0))? as usize;
        Ok(Buffer { inner, cursor })
    }
}

impl<B: Seek> Seek for Buffer<B> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.cursor = self.inner.seek(pos)? as usize;
        Ok(self.cursor as u64)
    }
}

impl<B: Seek + Read> Read for Buffer<B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.inner.read(buf)?;
        self.cursor += len;
        Ok(len)
    }
}

impl<B: Seek + Write> Write for Buffer<B> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.inner.write(buf)?;
        self.cursor += len;
        Ok(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
