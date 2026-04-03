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

    pub fn read_bytes(&mut self, n: usize) -> &'a [u8] {
        let (buf, rest) = self.current.split_at(n);
        self.current = rest;
        self.read_bytes += n;

        buf
    }

    pub fn seek(&mut self, offset: usize) {
        if offset > self.full_data.len() {
            panic!("Seek offset {} is out of bounds", offset);
        }
        self.current = &self.full_data[offset..];
        self.read_bytes = offset;
    }
}