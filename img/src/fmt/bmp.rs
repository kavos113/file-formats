use crate::reader::Reader;

struct BitmapFileHeader {
    bf_type: u16,
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    bf_off_bits: u32,
}

impl BitmapFileHeader {
    fn read_from(reader: &mut Reader) -> Self {
        BitmapFileHeader {
            bf_type: reader.read_u16(),
            bf_size: reader.read_u32(),
            bf_reserved1: reader.read_u16(),
            bf_reserved2: reader.read_u16(),
            bf_off_bits: reader.read_u32(),
        }
    }
}

struct BitmapInfoHeader {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

impl BitmapInfoHeader {
    fn read_from(reader: &mut Reader) -> Self {
        BitmapInfoHeader {
            bi_size: reader.read_u32(),
            bi_width: reader.read_i32(),
            bi_height: reader.read_i32(),
            bi_planes: reader.read_u16(),
            bi_bit_count: reader.read_u16(),
            bi_compression: reader.read_u32(),
            bi_size_image: reader.read_u32(),
            bi_x_pels_per_meter: reader.read_i32(),
            bi_y_pels_per_meter: reader.read_i32(),
            bi_clr_used: reader.read_u32(),
            bi_clr_important: reader.read_u32(),
        }
    }
}

struct RGBQuad {
    rgb_blue: u8,
    rgb_green: u8,
    rgb_red: u8,
    rgb_reserved: u8,
}