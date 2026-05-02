use crate::fmt::algorithm::deflate::analyze_file;
use crate::fmt::central_directory::{CentralDirectoryHeader, CompressionMethod};
use crate::reader::{BitReader, Reader};
use crate::writer::Writer;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;

mod deflate;

pub fn decompress_file(r: &mut BufReader<File>, header: &CentralDirectoryHeader) {
    let local_header = LocalFileHeader::load_from(r);

    let is_dir = local_header.file_name.ends_with('/');
    if is_dir {
        std::fs::create_dir_all(&local_header.file_name).unwrap();
        return;
    }

    let out_path = Path::new(&local_header.file_name);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut out = File::create(out_path).unwrap();

    match local_header.compression_method {
        CompressionMethod::Stored => {
            let mut buf = vec![0u8; local_header.compressed_size as usize];
            r.read_exact(&mut buf).unwrap();
            out.write_all(&buf).unwrap();
        }
        CompressionMethod::Deflated => {
            let mut bit_reader = BitReader::new(r);
            let mut writer = Writer::new(out);
            analyze_file(&mut bit_reader, &mut writer);

            writer.close();
        }
        _ => panic!(
            "Unsupported compression method: {}",
            local_header.compression_method
        ),
    }
}

struct LocalFileHeader {
    signature: u32,
    version_needed: u16,
    general_purpose_bit_flag: u16,
    compression_method: CompressionMethod,
    last_mod_file_time: u16,
    last_mod_file_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_name: String,
    extra_field: Vec<u8>,
}

impl LocalFileHeader {
    const FIXED_SIZE: usize = 30;

    fn load_from(reader: &mut BufReader<File>) -> Self {
        let mut buf = [0u8; Self::FIXED_SIZE];
        reader.read_exact(&mut buf).unwrap();

        let mut r = Reader::new(&buf);
        let signature = r.read_u32();
        if signature != 0x04034b50 {
            panic!("Invalid local file header signature: {:#x}", signature);
        }

        let version_needed = r.read_u16();
        let general_purpose_bit_flag = r.read_u16();
        let compression_method = r.read_u16();
        let last_mod_file_time = r.read_u16();
        let last_mod_file_date = r.read_u16();
        let crc32 = r.read_u32();
        let compressed_size = r.read_u32();
        let uncompressed_size = r.read_u32();
        let file_name_length = r.read_u16();
        let extra_field_length = r.read_u16();

        let mut file_name_buf = vec![0u8; file_name_length as usize];
        reader.read_exact(&mut file_name_buf).unwrap();
        let file_name = String::from_utf8(file_name_buf).unwrap();

        let mut extra_field = vec![0u8; extra_field_length as usize];
        reader.read_exact(&mut extra_field).unwrap();

        Self {
            signature,
            version_needed,
            general_purpose_bit_flag,
            compression_method: CompressionMethod::from_u16(compression_method),
            last_mod_file_time,
            last_mod_file_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_name,
            extra_field,
        }
    }
}
