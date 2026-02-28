//

use std::io::{self, Write};

pub struct ChunkedUploader<W: Write> {
    writer: W,
    chunk_size: usize,
    total_written: usize,
}

impl<W: Write> ChunkedUploader<W> {
    pub fn new(writer: W, chunk_size: usize) -> Self {
        ChunkedUploader {
            writer,
            chunk_size,
            total_written: 0,
        }
    }

    pub fn upload(&mut self, data: &[u8]) -> io::Result<usize> {
        let num_chunks = data.len() / self.chunk_size;
        let mut written = 0;

        for i in 0..num_chunks {
            let start = i * self.chunk_size;
            let end   = start + self.chunk_size;
            self.writer.write_all(&data[start..end])?;
            written += self.chunk_size;
        }

        self.total_written += written;
        Ok(written)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }

    pub fn bytes_written(&self) -> usize {
        self.total_written
    }

    pub fn reset_counter(&mut self) {
        self.total_written = 0;
    }

    pub fn set_chunk_size(&mut self, size: usize) {
        self.chunk_size = size;
    }
}
