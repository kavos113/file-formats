use std::fmt::{Display, Formatter};
use std::fs;
use crate::img::Image;
use crate::reader::Reader;

pub fn load_bmp(path: &str) -> Image {
    let buf = fs::read(path).expect("Failed to read BMP file");
    let mut reader = Reader::new(&buf);

    let file_header = BitmapFileHeader::read_from(&mut reader);
    let info_header = BitmapInfoHeader::read_from(&mut reader);

    println!("{}", file_header);
    println!("{}", info_header);

    Image {
        width: info_header.bi_width as u32,
        height: info_header.bi_height as u32,
        data: vec![]
    }
}

struct BitmapFileHeader {
    bf_type: u16,
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    bf_off_bits: u32,
}

impl BitmapFileHeader {
    fn read_from(reader: &mut Reader) -> Self {
        let bf_type = reader.read_u16();
        let bf_size = reader.read_u32();
        let bf_reserved1 = reader.read_u16();
        let bf_reserved2 = reader.read_u16();
        let bf_off_bits = reader.read_u32();

        if bf_type != 0x4D42 {
            println!("Invalid BMP file: expected 'BM' signature, found 0x{:X}", bf_type);
            panic!("Not a valid BMP file (missing 'BM' signature)");
        }
        if bf_reserved1 != 0 || bf_reserved2 != 0 {
            println!("Warning: Reserved fields are not zero ({} and {})", bf_reserved1, bf_reserved2);
        }

        BitmapFileHeader {
            bf_type,
            bf_size,
            bf_reserved1,
            bf_reserved2,
            bf_off_bits,
        }
    }
}

impl Display for BitmapFileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-------- Bitmap File Header --------\n")?;
        write!(f, "Type:                 0x{:X}\n", self.bf_type)?;
        write!(f, "Size:                 {}\n", self.bf_size)?;
        write!(f, "Offset to Pixel Data: {}\n\n", self.bf_off_bits)
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

impl Display for BitmapInfoHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-------- Bitmap Info Header --------\n")?;
        write!(f, "Size:                {}\n", self.bi_size)?;
        write!(f, "Width:               {}\n", self.bi_width)?;
        write!(f, "Height:              {}\n", self.bi_height)?;
        write!(f, "Planes:              {}\n", self.bi_planes)?;
        write!(f, "Bit Count:           {}\n", self.bi_bit_count)?;
        write!(f, "Compression:         {}\n", self.bi_compression)?;
        write!(f, "Image Size:          {}\n", self.bi_size_image)?;
        write!(f, "X Pixels per Meter:  {}\n", self.bi_x_pels_per_meter)?;
        write!(f, "Y Pixels per Meter:  {}\n", self.bi_y_pels_per_meter)?;
        write!(f, "Colors Used:         {}\n", self.bi_clr_used)?;
        write!(f, "Important Colors:    {}\n\n", self.bi_clr_important)
    }
}

struct RGBQuad {
    rgb_blue: u8,
    rgb_green: u8,
    rgb_red: u8,
    rgb_reserved: u8,
}
