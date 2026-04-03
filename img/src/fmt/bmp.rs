use std::fmt::{Display, Formatter};
use std::fs;
use crate::img::{Image, Pixel};
use crate::reader::Reader;

pub fn load_bmp(path: &str) -> Image {
    let buf = fs::read(path).expect("Failed to read BMP file");
    let mut reader = Reader::new(&buf);

    let file_header = BitmapFileHeader::read_from(&mut reader);
    let info_header = BitmapInfoHeader::read_from(&mut reader);

    println!("{}", file_header);
    println!("{}", info_header);

    let pixels = match info_header.bi_compression {
        Compression::BI_RGB => {
            match info_header.bi_bit_count {
                1 | 4 | 8 => {
                    read_palette_pixels(
                        &mut reader,
                        file_header.bf_off_bits,
                        info_header.bi_width,
                        info_header.bi_height,
                        info_header.bi_bit_count,
                        info_header.bi_clr_used,
                    )
                }
                24 => {
                    read_plain_pixels(
                        &mut reader,
                        ((info_header.bi_width as u32 * 3 + 3) & !3), // Row stride (padded to 4 bytes)
                        info_header.bi_width,
                        info_header.bi_height,
                    )
                }
                _ => {
                    println!("Unsupported bit count for BI_RGB: {}", info_header.bi_bit_count);
                    panic!("Cannot load BMP with unsupported bit count");
                }
            }
        }

        Compression::BI_BITFIELDS => {
            match info_header.bi_bit_count {
                16 | 32 => {
                    read_bitfield_pixels(
                        &mut reader,
                        file_header.bf_off_bits,
                        info_header.bi_width,
                        info_header.bi_height,
                        info_header.bi_bit_count,
                    )
                }
                _ => {
                    println!("Unsupported bit count for BI_BITFIELDS: {}", info_header.bi_bit_count);
                    panic!("Cannot load BMP with unsupported bit count for BI_BITFIELDS");
                }
            }
        }

        _ => {
            println!("Unsupported compression type: {}", info_header.bi_compression);
            panic!("Cannot load BMP with unsupported compression");
        }
    };

    Image {
        width: info_header.bi_width as u32,
        height: info_header.bi_height as u32,
        data: pixels,
    }
}

struct BitmapFileHeader {
    bf_type: u16,
    bf_size: u32,
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
            bf_off_bits,
        }
    }
}

impl Display for BitmapFileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-------- Bitmap File Header --------")?;
        writeln!(f, "Type:                 0x{:X}", self.bf_type)?;
        writeln!(f, "Size:                 {}", self.bf_size)?;
        write!(f, "Offset to Pixel Data: {}\n\n", self.bf_off_bits)
    }
}

struct BitmapInfoHeader {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: Compression,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

enum Compression {
    BI_RGB = 0,
    BI_RLE8 = 1,
    BI_RLE4 = 2,
    BI_BITFIELDS = 3,
    BI_JPEG = 4,
    BI_PNG = 5,
}

impl BitmapInfoHeader {
    fn read_from(reader: &mut Reader) -> Self {
        let bi_size = reader.read_u32();
        let bi_width = reader.read_i32();
        let bi_height = reader.read_i32();
        let bi_planes = reader.read_u16();
        let bi_bit_count = reader.read_u16();
        let bi_compression_value = reader.read_u32();
        let bi_compression = match bi_compression_value {
            0 => Compression::BI_RGB,
            1 => Compression::BI_RLE8,
            2 => Compression::BI_RLE4,
            3 => Compression::BI_BITFIELDS,
            4 => Compression::BI_JPEG,
            5 => Compression::BI_PNG,
            _ => {
                println!("Warning: Unknown compression type: {}", bi_compression_value);
                Compression::BI_RGB // Default to no compression
            }
        };
        let bi_size_image = reader.read_u32();
        let bi_x_pels_per_meter = reader.read_i32();
        let bi_y_pels_per_meter = reader.read_i32();
        let bi_clr_used = reader.read_u32();
        let bi_clr_important = reader.read_u32();

        BitmapInfoHeader {
            bi_size,
            bi_width,
            bi_height,
            bi_planes,
            bi_bit_count,
            bi_compression,
            bi_size_image,
            bi_x_pels_per_meter,
            bi_y_pels_per_meter,
            bi_clr_used,
            bi_clr_important,
        }
    }
}

impl Display for BitmapInfoHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-------- Bitmap Info Header --------")?;
        writeln!(f, "Size:                {}", self.bi_size)?;
        writeln!(f, "Width:               {}", self.bi_width)?;
        writeln!(f, "Height:              {}", self.bi_height)?;
        writeln!(f, "Planes:              {}", self.bi_planes)?;
        writeln!(f, "Bit Count:           {}", self.bi_bit_count)?;
        writeln!(f, "Compression:         {}", self.bi_compression)?;
        writeln!(f, "Image Size:          {}", self.bi_size_image)?;
        writeln!(f, "X Pixels per Meter:  {}", self.bi_x_pels_per_meter)?;
        writeln!(f, "Y Pixels per Meter:  {}", self.bi_y_pels_per_meter)?;
        writeln!(f, "Colors Used:         {}", self.bi_clr_used)?;
        write!(f, "Important Colors:    {}\n\n", self.bi_clr_important)
    }
}

impl Display for Compression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Compression::BI_RGB => "BI_RGB (No compression)",
            Compression::BI_RLE8 => "BI_RLE8 (8-bit RLE compression)",
            Compression::BI_RLE4 => "BI_RLE4 (4-bit RLE compression)",
            Compression::BI_BITFIELDS => "BI_BITFIELDS (Bit field masks)",
            Compression::BI_JPEG => "BI_JPEG (JPEG compression)",
            Compression::BI_PNG => "BI_PNG (PNG compression)",
        };
        write!(f, "{}", description)
    }
}

struct RGBQuad {
    rgb_blue: u8,
    rgb_green: u8,
    rgb_red: u8,
    rgb_reserved: u8,
}

// BI_RGB, 24bpp
fn read_plain_pixels(reader: &mut Reader, stride: u32, width: i32, height: i32) -> Vec<Pixel> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for _y in 0..height {
        let row_bytes = reader.read_bytes(stride as usize);
        for x in 0..width {
            let offset = (x * 3) as usize;
            let b = row_bytes[offset];
            let g = row_bytes[offset + 1];
            let r = row_bytes[offset + 2];
            let pixel = Pixel { r, g, b, a: 255 };
            pixels.push(pixel);
        }
    }

    pixels
}

// BI_RGB, 1/4/8bpp with palette
fn read_palette_pixels(reader: &mut Reader, off_bits: u32, width: i32, height: i32, bit_count: u16, color_used: u32) -> Vec<Pixel> {
    let palette_size = if color_used > 0 {
        color_used
    } else {
        1 << bit_count
    };
    let mut palette = Vec::with_capacity(palette_size as usize);
    for _ in 0..palette_size {
        let rgb_quad = RGBQuad {
            rgb_blue: reader.read_u8(),
            rgb_green: reader.read_u8(),
            rgb_red: reader.read_u8(),
            rgb_reserved: reader.read_u8(),
        };
        palette.push(rgb_quad);
    }

    reader.seek(off_bits as usize);

    let mut pixels = Vec::with_capacity((width * height) as usize);
    let stride = ((width * bit_count as i32 + 31) & !31) >> 3;

    for _y in 0..height {
        let row_bytes = reader.read_bytes(stride as usize);
        for x in 0..width {
            let pixel_index = match bit_count {
                1 => (row_bytes[(x / 8) as usize] >> (7 - (x % 8))) & 0x01,
                4 => (row_bytes[(x / 2) as usize] >> (4 - ((x % 2) * 4))) & 0x0F,
                8 => row_bytes[x as usize],
                _ => panic!("Unsupported bit count: {}", bit_count),
            };
            let rgb_quad = &palette[pixel_index as usize];
            let pixel = Pixel {
                r: rgb_quad.rgb_red,
                g: rgb_quad.rgb_green,
                b: rgb_quad.rgb_blue,
                a: 255,
            };
            pixels.push(pixel);
        }
    }

    pixels
}

// BI_BITFIELDS, 16/32bpp with bit masks
fn read_bitfield_pixels(reader: &mut Reader, off_bits: u32, width: i32, height: i32, bit_count: u16) -> Vec<Pixel> {
    let red_mask = reader.read_u32();
    let green_mask = reader.read_u32();
    let blue_mask = reader.read_u32();

    let get_info = |mask: u32| {
        let shift = mask.trailing_zeros();
        let max = mask >> shift;
        (shift, max)
    };
    let (red_shift, red_max) = get_info(red_mask);
    let (green_shift, green_max) = get_info(green_mask);
    let (blue_shift, blue_max) = get_info(blue_mask);

    reader.seek(off_bits as usize);

    let mut pixels = Vec::with_capacity((width * height) as usize);
    let stride = ((width * bit_count as i32 + 31) & !31) >> 3;

    for _y in 0..height {
        let mut read = 0;
        for _x in 0..width {
            let pixel_value = if bit_count == 16 {
                read += 2;
                reader.read_u16() as u32
            } else {
                read += 4;
                reader.read_u32()
            };
            let r = (pixel_value & red_mask) >> red_shift;
            let g = (pixel_value & green_mask) >> green_shift;
            let b = (pixel_value & blue_mask) >> blue_shift;
            let pixel = Pixel {
                r: ((r * 255) / red_max) as u8,
                g: ((g * 255) / green_max) as u8,
                b: ((b * 255) / blue_max) as u8,
                a: 255,
            };
            pixels.push(pixel);
        }

        let padding = (stride as usize).saturating_sub(read);
        if padding > 0 {
            reader.read_bytes(padding);
        }
    }

    pixels
}