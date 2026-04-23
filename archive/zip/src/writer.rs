use std::fs::File;
use std::io::{BufWriter, Write};

struct Writer {
    buffer: Vec<u8>,
    out: BufWriter<File>,
}

impl Writer {
    const CHUNK_SIZE: usize = 131072;
    const LEAST_BUFFER_SIZE: usize = 32768; // to copy previous data for back-references
    const BUFFER_SIZE: usize = Self::CHUNK_SIZE + Self::LEAST_BUFFER_SIZE;

    pub fn new(out: File) -> Self {
        Writer {
            buffer: Vec::with_capacity(Self::BUFFER_SIZE),
            out: BufWriter::new(out),
        }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);

        if self.buffer.len() >= Self::BUFFER_SIZE {
            self.flush();
        }
    }

    // flush. leave at least LEAST_BUFFER_SIZE bytes
    pub fn flush(&mut self) {

    }
}