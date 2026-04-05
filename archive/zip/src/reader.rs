use std::fs::File;
use std::io::{BufReader, Read};

pub struct Reader<'a> {
    full_data: &'a [u8],
    current: &'a [u8],
    pub read_bytes: usize,
    pub total_bytes: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader {
            full_data: data,
            current: data,
            read_bytes: 0,
            total_bytes: data.len(),
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let (buf, rest) = self.current.split_at(1);
        self.current = rest;
        self.read_bytes += 1;

        buf[0]
    }

    pub fn read_u16(&mut self) -> u16 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;
        self.read_bytes += 2;

        u16::from_le_bytes([buf[0], buf[1]])
    }

    pub fn read_i16(&mut self) -> i16 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;
        self.read_bytes += 2;

        i16::from_le_bytes([buf[0], buf[1]])
    }

    pub fn read_u32(&mut self) -> u32 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;
        self.read_bytes += 4;

        u32::from_le_bytes(buf.try_into().unwrap())
    }

    pub fn read_i32(&mut self) -> i32 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;
        self.read_bytes += 4;

        i32::from_le_bytes(buf.try_into().unwrap())
    }

    pub fn read_u64(&mut self) -> u64 {
        let (buf, rest) = self.current.split_at(8);
        self.current = rest;
        self.read_bytes += 8;

        u64::from_le_bytes(buf.try_into().unwrap())
    }

    pub fn read_i64(&mut self) -> i64 {
        let (buf, rest) = self.current.split_at(8);
        self.current = rest;
        self.read_bytes += 8;

        i64::from_le_bytes(buf.try_into().unwrap())
    }

    pub fn read_bytes(&mut self, n: usize) -> &'a [u8] {
        let (buf, rest) = self.current.split_at(n);
        self.current = rest;
        self.read_bytes += n;

        buf
    }

    pub fn peek_bytes(&self, n: usize) -> &'a [u8] {
        if n > self.current.len() {
            panic!("Peek length {} is out of bounds", n);
        }
        &self.current[..n]
    }

    pub fn seek(&mut self, offset: usize) {
        if offset > self.full_data.len() {
            panic!("Seek offset {} is out of bounds", offset);
        }
        self.current = &self.full_data[offset..];
        self.read_bytes = offset;
    }

    pub fn seek_from_current(&mut self, offset: i32) {
        let new_offset = (self.read_bytes as i32) + offset;
        if new_offset < 0 || (new_offset as usize) > self.full_data.len() {
            panic!("Seek offset {} is out of bounds", new_offset);
        }
        self.current = &self.full_data[new_offset as usize..];
        self.read_bytes = new_offset as usize;
    }
}

pub struct BitReader<'a> {
    reader: &'a mut BufReader<File>,
    buffer: u64,
    remain_bits: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(reader: &'a mut BufReader<File>) -> Self {
        BitReader {
            reader,
            buffer: 0,
            remain_bits: 0,
        }
    }

    fn fill_buffer(&mut self) {
        let mut byte = [0u8; 1];

        if self.reader.read_exact(&mut byte).is_ok() {
            self.buffer |= (byte[0] as u64) << self.remain_bits;
            self.remain_bits += 8;
        }
    }

    pub fn read_bits(&mut self, n: usize) -> u64 {
        if n > 64 {
            panic!("Cannot read more than 64 bits at a time");
        }

        while self.remain_bits < n {
            self.fill_buffer();
        }

        let result = self.buffer & ((1u64 << n) - 1);
        self.buffer >>= n;
        self.remain_bits -= n;

        result
    }

    pub fn align_to_byte(&mut self) {
        let skip_bits = self.remain_bits % 8;
        if skip_bits > 0 {
            self.read_bits(skip_bits);
        }
    }
}