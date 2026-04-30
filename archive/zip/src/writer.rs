use std::fs::File;
use std::io::{BufWriter, Write};

pub struct Writer {
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

    pub fn copy(&mut self, distance: u64, length: u64) {
        let copy_range: Vec<_> = self.buffer[self.buffer.len() - distance as usize..]
            .iter()
            .cloned()
            .collect();
        let mut tocopy = copy_range.iter().cycle();

        for _ in 0..length {
            self.write_u8(*tocopy.next().unwrap());
        }
    }

    // flush. leave at least LEAST_BUFFER_SIZE bytes
    pub fn flush(&mut self) {
        if self.buffer.len() > Self::LEAST_BUFFER_SIZE {
            let flush_size = self.buffer.len() - Self::LEAST_BUFFER_SIZE;
            self.out.write_all(&self.buffer[..flush_size]).unwrap();
            self.buffer.drain(..flush_size);
        }
    }

    fn write_all(&mut self) {
        self.out.write_all(&self.buffer).unwrap();
        self.buffer.clear();
    }

    pub fn close(mut self) {
        self.write_all();
        self.out.flush().unwrap();
    }
}
