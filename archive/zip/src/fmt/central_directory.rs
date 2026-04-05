use std::cmp::min;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use chrono::{DateTime, Duration, Utc};
use crate::reader::Reader;

pub struct EndOfCentralDirectoryRecord {
    signature: u32,
    disk_number: u16,
    central_directory_start_disk_number: u16,
    total_entries_disk: u16,
    total_entries: u16,
    central_directory_size: u32,
    central_directory_offset: u32,
    comment_length: u16,
    comment: String
}

impl EndOfCentralDirectoryRecord {
    const MAX_LENGTH: i64 = 22 + 65535;

    pub fn find_record(file: &mut File) -> Self {
        let file_size = file
            .metadata()
            .expect("Failed to get file metadata")
            .len();
        if file_size < Self::MAX_LENGTH as u64 {
            file.seek(SeekFrom::Start(0)).expect("Failed to seek to start of file");
        } else {
            file.seek(SeekFrom::End(-Self::MAX_LENGTH)).expect("Failed to seek to end of file");
        }

        let buf_size = min(Self::MAX_LENGTH as usize, file_size as usize);

        let mut buffer = vec![0; buf_size];
        file.read_exact(&mut buffer)
            .expect("Failed to read from file");

        for i in (0..buf_size - 21).rev() {
            if &buffer[i..i + 4] == [0x50, 0x4b, 0x05, 0x06] {
                let record = match Self::read_from(&buffer[i..]) {
                    Some(record) => record,
                    None => continue,
                };
                return record;
            }
        }

        panic!("End of central directory record not found");
    }

    fn read_from(buf: &[u8]) -> Option<Self> {
        let mut reader = Reader::new(buf);

        let signature = reader.read_u32();
        if signature != 0x06054b50 {
            return None;
        }

        let disk_number = reader.read_u16();
        let central_directory_start_disk_number = reader.read_u16();
        let total_entries_disk = reader.read_u16();
        let total_entries = reader.read_u16();
        let central_directory_size = reader.read_u32();
        let central_directory_offset = reader.read_u32();
        let comment_length = reader.read_u16();
        let comment_bytes = reader.read_bytes(comment_length as usize);
        let comment = String::from_utf8_lossy(comment_bytes).to_string();

        if comment_length + 22 != buf.len() as u16 {
            return None;
        }

        Some(Self {
            signature,
            disk_number,
            central_directory_start_disk_number,
            total_entries_disk,
            total_entries,
            central_directory_size,
            central_directory_offset,
            comment_length,
            comment
        })
    }
}

impl Display for EndOfCentralDirectoryRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------- End of Central Directory Record ----------")?;
        writeln!(f, "Signature:                           0x{:08x}", self.signature)?;
        writeln!(f, "Current Disk Number:                 {}", self.disk_number)?;
        writeln!(f, "Central Directory Start Disk Number: {}", self.central_directory_start_disk_number)?;
        writeln!(f, "Total Entries on Current Disk:       {}", self.total_entries_disk)?;
        writeln!(f, "Total Entries:                       {}", self.total_entries)?;
        writeln!(f, "Central Directory Size:              {}", self.central_directory_size)?;
        writeln!(f, "Central Directory Offset:            {}", self.central_directory_offset)?;
        writeln!(f, "Comment Length:                      {}", self.comment_length)?;
        writeln!(f, "Comment: \n  {}\n", self.comment)?;

        Ok(())
    }
}

pub struct CentralDirectory {
    headers: Vec<CentralDirectoryHeader>,
    signature: Option<CentralDirectoryDigitalSignature>
}

impl CentralDirectory {
    pub fn from_record(file: &mut File, record: &EndOfCentralDirectoryRecord) -> Self {
        file.seek(SeekFrom::Start(record.central_directory_offset as u64))
            .expect("Failed to seek to central directory");

        let mut buffer = vec![0; record.central_directory_size as usize];
        file.read_exact(&mut buffer)
            .expect("Failed to read central directory");

        Self::read_from(&mut Reader::new(&buffer), record.total_entries as usize, record.central_directory_size as usize)
    }

    fn read_from(reader: &mut Reader, num_entries: usize, total_bytes: usize) -> Self {
        let mut headers = Vec::with_capacity(num_entries);
        for _ in 0..num_entries {
            headers.push(CentralDirectoryHeader::read_from(reader));
        }

        let signature = if reader.read_bytes < total_bytes {
            Some(CentralDirectoryDigitalSignature::read_from(reader))
        } else {
            None
        };

        Self {
            headers,
            signature
        }
    }
}

impl Display for CentralDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.headers
            .iter()
            .try_for_each(|header| writeln!(f, "{}", header))?;

        Ok(())
    }
}

pub struct CentralDirectoryHeader {
    signature: u32,
    version_made_by: u16,
    version_needed: u16,
    general_purpose_flag: u16,
    compression_method: CompressionMethod,
    last_mod_time: u16,
    last_mod_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_number_start: u16,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    local_header_offset: u32,
    file_name: String,
    extra_field: Vec<ExtraField>,
    file_comment: String
}

impl CentralDirectoryHeader {
    fn read_from(reader: &mut Reader) -> Self {
        let signature = reader.read_u32();
        if signature != 0x02014b50 {
            panic!("Invalid central directory header signature: 0x{:08x}", signature);
        }

        let version_made_by = reader.read_u16();
        let version_needed = reader.read_u16();
        let general_purpose_flag = reader.read_u16();
        let compression_method = reader.read_u16();
        let last_mod_time = reader.read_u16();
        let last_mod_date = reader.read_u16();
        let crc32 = reader.read_u32();
        let compressed_size = reader.read_u32();
        let uncompressed_size = reader.read_u32();
        let file_name_length = reader.read_u16();
        let extra_field_length = reader.read_u16();
        let file_comment_length = reader.read_u16();
        let disk_number_start = reader.read_u16();
        let internal_file_attributes = reader.read_u16();
        let external_file_attributes = reader.read_u32();
        let local_header_offset = reader.read_u32();

        let file_name_bytes = reader.read_bytes(file_name_length as usize);
        let file_name = String::from_utf8_lossy(file_name_bytes).to_string();

        let extra_field_bytes = reader.read_bytes(extra_field_length as usize);
        let mut extra_field_reader = Reader::new(extra_field_bytes);
        let mut extra_field = Vec::with_capacity((extra_field_length / 4) as usize);
        while extra_field_reader.read_bytes < extra_field_length as usize {
            extra_field.push(ExtraField::read_from(&mut extra_field_reader));
        }

        let file_comment_bytes = reader.read_bytes(file_comment_length as usize);
        let file_comment = String::from_utf8_lossy(file_comment_bytes).to_string();

        Self {
            signature,
            version_made_by,
            version_needed,
            general_purpose_flag,
            compression_method: CompressionMethod::from_u16(compression_method),
            last_mod_time,
            last_mod_date,
            crc32,
            compressed_size,
            uncompressed_size,
            file_name_length,
            extra_field_length,
            file_comment_length,
            disk_number_start,
            internal_file_attributes,
            external_file_attributes,
            local_header_offset,
            file_name,
            extra_field,
            file_comment
        }
    }
}

impl Display for CentralDirectoryHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------- Central Directory Header ----------")?;
        writeln!(f, "Signature:                0x{:08x}", self.signature)?;
        writeln!(f, "Version Made By:          {}", format_version_made_by(self.version_made_by))?;
        writeln!(f, "Version Needed:           {}", format_version_needed(self.version_needed))?;
        writeln!(f, "General Purpose Flag:     0b{:016b}", self.general_purpose_flag)?;
        writeln!(f, "Compression Method:       {}", self.compression_method)?;
        writeln!(f, "Last Modified             {} {}", format_date(self.last_mod_date), format_time(self.last_mod_time))?;
        writeln!(f, "CRC-32:                   0x{:08x}", self.crc32)?;
        writeln!(f, "Compressed Size:          {}", self.compressed_size)?;
        writeln!(f, "Uncompressed Size:        {}", self.uncompressed_size)?;
        writeln!(f, "File Name Length:         {}", self.file_name_length)?;
        writeln!(f, "Extra Field Length:       {}", self.extra_field_length)?;
        writeln!(f, "File Comment Length:      {}", self.file_comment_length)?;
        writeln!(f, "Disk Number Start:        {}", self.disk_number_start)?;
        writeln!(f, "Internal File Attributes: 0b{:016b}", self.internal_file_attributes)?;
        writeln!(f, "External File Attributes: {}", format_external_attributes(self.external_file_attributes, self.version_made_by))?;
        writeln!(f, "Local Header Offset:      {}", self.local_header_offset)?;
        writeln!(f, "File Name: \n  {}", self.file_name)?;
        self.extra_field.iter().try_for_each(|field| writeln!(f, "{}", field))?;
        writeln!(f, "File Comment: \n  {}\n", self.file_comment)?;

        Ok(())
    }
}

fn format_version_made_by(version: u16) -> String {
    let os = version >> 8;
    let ver = version & 0x00ff;

    let os_str = match os {
        0 => "MS-DOS and OS/2 (FAT)",
        1 => "Amiga",
        2 => "OpenVMS",
        3 => "Unix",
        4 => "VM/CMS",
        5 => "Atari ST",
        6 => "OS/2 H.P.F.S.",
        7 => "Macintosh",
        8 => "Z-System",
        9 => "CP/M",
        10 => "Windows NTFS",
        11 => "MVS (OS/390 - Z/OS)",
        12 => "VSE",
        13 => "Acorn Risc",
        14 => "VFAT",
        15 => "Alternate MVS",
        16 => "BeOS",
        17 => "Tandem",
        18 => "OS/400",
        19 => "OS X (Darwin)",
        _ => "Unknown"
    };

    let major = ver / 10;
    let minor = ver % 10;

    format!("{}.{} {}", major, minor, os_str)
}

fn format_version_needed(version: u16) -> String {
    let major = version / 10;
    let minor = version % 10;

    format!("{}.{}", major, minor)
}

fn format_time(time: u16) -> String {
    let hours = time >> 11;
    let minutes = (time >> 5) & 0x3f;
    let seconds = (time & 0x1f) * 2;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn format_date(date: u16) -> String {
    let year = ((date >> 9) & 0x7f) + 1980;
    let month = (date >> 5) & 0x0f;
    let day = date & 0x1f;

    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn format_external_attributes(attrs: u32, made_by: u16) -> String {
    let os = made_by >> 8;
    match os {
        0 => {
            let read_only = (attrs & 0x01) != 0;
            let hidden = (attrs & 0x02) != 0;
            let system = (attrs & 0x04) != 0;
            let volume_label = (attrs & 0x08) != 0;
            let directory = (attrs & 0x10) != 0;
            let archive = (attrs & 0x20) != 0;

            format!(
                "MS-DOS Attributes: {}{}{}{}{}{}",
                if read_only { "Read-Only " } else { "" },
                if hidden { "Hidden " } else { "" },
                if system { "System " } else { "" },
                if volume_label { "Volume Label " } else { "" },
                if directory { "Directory " } else { "" },
                if archive { "Archive" } else { "" }
            )
        }
        3 => format!("Unix Permissions: 0o{:o}", (attrs >> 16) & 0xffff),
        _ => format!("External Attributes: 0x{:08x}", attrs)
    }
}

pub enum CompressionMethod {
    Stored = 0,
    Shrunk = 1,
    Reduced1 = 2,
    Reduced2 = 3,
    Reduced3 = 4,
    Reduced4 = 5,
    Imploded = 6,
    Deflated = 8,
    Deflate64 = 9,
    PKWareImploded = 10,
    BZIP2 = 12,
    LZMA = 14,
    IBMCMPSC = 16,
    IBMTERSE = 18,
    IBMLZ77 = 19,
    Zstd = 93,
    MP3 = 94,
    XZ = 95,
    JPEG = 96,
    WavPack = 97,
    PPMd = 98,
    AEX = 99
}

impl CompressionMethod {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => CompressionMethod::Stored,
            1 => CompressionMethod::Shrunk,
            2 => CompressionMethod::Reduced1,
            3 => CompressionMethod::Reduced2,
            4 => CompressionMethod::Reduced3,
            5 => CompressionMethod::Reduced4,
            6 => CompressionMethod::Imploded,
            8 => CompressionMethod::Deflated,
            9 => CompressionMethod::Deflate64,
            10 => CompressionMethod::PKWareImploded,
            12 => CompressionMethod::BZIP2,
            14 => CompressionMethod::LZMA,
            16 => CompressionMethod::IBMCMPSC,
            18 => CompressionMethod::IBMTERSE,
            19 => CompressionMethod::IBMLZ77,
            93 => CompressionMethod::Zstd,
            94 => CompressionMethod::MP3,
            95 => CompressionMethod::XZ,
            96 => CompressionMethod::JPEG,
            97 => CompressionMethod::WavPack,
            98 => CompressionMethod::PPMd,
            99 => CompressionMethod::AEX,
            _ => panic!("Unknown compression method: {}", value)
        }
    }
}

impl Display for CompressionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            CompressionMethod::Stored => "Stored (no compression)",
            CompressionMethod::Shrunk => "Shrunk",
            CompressionMethod::Reduced1 => "Reduced with compression factor 1",
            CompressionMethod::Reduced2 => "Reduced with compression factor 2",
            CompressionMethod::Reduced3 => "Reduced with compression factor 3",
            CompressionMethod::Reduced4 => "Reduced with compression factor 4",
            CompressionMethod::Imploded => "Imploded",
            CompressionMethod::Deflated => "Deflated",
            CompressionMethod::Deflate64 => "Deflate64",
            CompressionMethod::PKWareImploded => "PKWare Data Compression Library Imploding",
            CompressionMethod::BZIP2 => "BZIP2",
            CompressionMethod::LZMA => "LZMA",
            CompressionMethod::IBMCMPSC => "IBM z/OS CMPSC",
            CompressionMethod::IBMTERSE => "IBM TERSE (new)",
            CompressionMethod::IBMLZ77 => "IBM LZ77 z Architecture (PFS)",
            CompressionMethod::Zstd => "Zstandard (zstd)",
            CompressionMethod::MP3 => "MP3",
            CompressionMethod::XZ => "XZ",
            CompressionMethod::JPEG => "JPEG",
            CompressionMethod::WavPack => "WavPack compressed data",
            CompressionMethod::PPMd => "PPMd version I, Rev 1",
            CompressionMethod::AEX => "AE-x encryption marker"
        };

        write!(f, "{}", method_str)
    }
}

struct CentralDirectoryDigitalSignature {
    signature: u32,
    size_of_data: u32,
    data: Vec<u8>
}

impl CentralDirectoryDigitalSignature {
    fn read_from(reader: &mut Reader) -> Self {
        let signature = reader.read_u32();
        if signature != 0x05054b50 {
            panic!("Invalid central directory digital signature: 0x{:08x}", signature);
        }

        let size_of_data = reader.read_u32();
        let data = reader.read_bytes(size_of_data as usize).to_vec();

        Self {
            signature,
            size_of_data,
            data
        }
    }
}

enum ExtraField {
    Zip64(ExtraFieldZip64),
    OS2(ExtraFieldOS2),
    NTFS(ExtraFieldNTFS)
}

impl ExtraField {
    fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        reader.seek_from_current(-2);
        match header_id {
            0x0001 => ExtraField::Zip64(ExtraFieldZip64::read_from(reader)),
            0x000d => ExtraField::OS2(ExtraFieldOS2::read_from(reader)),
            0x000a => ExtraField::NTFS(ExtraFieldNTFS::read_from(reader)),
            _ => panic!("Unknown extra field header ID: 0x{:04x}", header_id)
        }
    }
}

impl Display for ExtraField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtraField::Zip64(zip64) => write!(f, "{}", zip64)?,
            ExtraField::OS2(os2) => write!(f, "{}", os2)?,
            ExtraField::NTFS(ntfs) => write!(f, "{}", ntfs)?
        }

        Ok(())
    }
}

struct ExtraFieldZip64 {
    header_id: u16,
    data_size: u16,
    original_size: u64,
    compressed_size: u64,
    local_header_offset: u64,
    disk_start_number: u32
}

impl ExtraFieldZip64 {
    fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        if header_id != 0x0001 {
            panic!("Invalid Zip64 extra field header ID: 0x{:04x}", header_id);
        }

        let data_size = reader.read_u16();
        if data_size != 28 {
            panic!("Invalid Zip64 extra field data size: {}", data_size);
        }

        let original_size = reader.read_u64();
        let compressed_size = reader.read_u64();
        let local_header_offset = reader.read_u64();
        let disk_start_number = reader.read_u32();

        Self {
            header_id,
            data_size,
            original_size,
            compressed_size,
            local_header_offset,
            disk_start_number
        }
    }
}

impl Display for ExtraFieldZip64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------- Zip64 Extended Information Extra Field ----------")?;
        writeln!(f, "Header ID:              0x{:04x}", self.header_id)?;
        writeln!(f, "Data Size:              {}", self.data_size)?;
        writeln!(f, "Original Size:          {}", self.original_size)?;
        writeln!(f, "Compressed Size:        {}", self.compressed_size)?;
        writeln!(f, "Local Header Offset:    {}", self.local_header_offset)?;
        writeln!(f, "Disk Start Number:      {}", self.disk_start_number)?;

        Ok(())
    }
}

struct ExtraFieldOS2 {
    header_id: u16,
    data_size: u16,
    block_size: u16,
    compression_type: u16,
    ea_crc: u32,
    block: Vec<u8>
}

impl ExtraFieldOS2 {
    fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        if header_id != 0x000d {
            panic!("Invalid OS/2 extra field header ID: 0x{:04x}", header_id);
        }

        let data_size = reader.read_u16();
        if data_size < 14 {
            panic!("Invalid OS/2 extra field data size: {}", data_size);
        }

        let block_size = reader.read_u16();
        let compression_type = reader.read_u16();
        let ea_crc = reader.read_u32();
        let block = reader.read_bytes((data_size - 14) as usize).to_vec();

        Self {
            header_id,
            data_size,
            block_size,
            compression_type,
            ea_crc,
            block
        }
    }
}

impl Display for ExtraFieldOS2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------- OS/2 Extra Field ----------")?;
        writeln!(f, "Header ID:              0x{:04x}", self.header_id)?;
        writeln!(f, "Data Size:              {}", self.data_size)?;
        writeln!(f, "Block Size:             {}", self.block_size)?;
        writeln!(f, "Compression Type:       0x{:04x}", self.compression_type)?;
        writeln!(f, "EA CRC-32:             0x{:08x}", self.ea_crc)?;
        writeln!(f, "Block Data (hex): \n  {:02x?}", self.block)?;

        Ok(())
    }
}

struct ExtraFieldNTFS {
    header_id: u16,
    data_size: u16,
    attribute_block: Vec<ExtraFieldNTFSAttributeBlock>
}

impl ExtraFieldNTFS {
    fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        if header_id != 0x000a {
            panic!("Invalid NTFS extra field header ID: 0x{:04x}", header_id);
        }

        let data_size = reader.read_u16() - 4;
        if data_size % 24 != 4 {
            panic!("Invalid NTFS extra field data size: {}", data_size);
        }

        reader.read_u32(); // reserved

        let mut attribute_block = Vec::with_capacity((data_size / 24) as usize);
        for _ in 0..(data_size / 24) {
            attribute_block.push(ExtraFieldNTFSAttributeBlock::read_from(reader));
        }

        Self {
            header_id,
            data_size,
            attribute_block
        }
    }
}

struct ExtraFieldNTFSAttributeBlock {
    tag: u16,
    size: u16,
    mod_time: u64,
    access_time: u64,
    create_time: u64
}

impl ExtraFieldNTFSAttributeBlock {
    fn read_from(reader: &mut Reader) -> Self {
        let tag = reader.read_u16();
        let size = reader.read_u16();
        if size != 24 {
            panic!("Invalid NTFS attribute block size: {}", size);
        }

        let mod_time = reader.read_u64();
        let access_time = reader.read_u64();
        let create_time = reader.read_u64();

        Self {
            tag,
            size,
            mod_time,
            access_time,
            create_time
        }
    }
}

fn format_ntfs_time(ntfs_time: u64) -> String {
    if ntfs_time == 0 {
        return "N/A".to_string();
    }
    let unix_time = (ntfs_time / 10_000_000) - 11644473600;
    let datetime = std::time::Duration::from_secs(unix_time);
    let datetime: DateTime<Utc> = DateTime::from(std::time::UNIX_EPOCH + datetime);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

impl Display for ExtraFieldNTFS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "---------- NTFS Extra Field ----------")?;
        writeln!(f, "Header ID:              0x{:04x}", self.header_id)?;
        writeln!(f, "Data Size:              {}", self.data_size)?;
        for (i, block) in self.attribute_block.iter().enumerate() {
            writeln!(f, "Attribute Block {}:", i + 1)?;
            writeln!(f, "  Tag:                 0x{:04x}", block.tag)?;
            writeln!(f, "  Size:                {}", block.size)?;
            writeln!(f, "  Last Modified Time:  {}", format_ntfs_time(block.mod_time))?;
            writeln!(f, "  Last Access Time:    {}", format_ntfs_time(block.access_time))?;
            writeln!(f, "  Creation Time:       {}", format_ntfs_time(block.create_time))?;
        }

        Ok(())
    }
}
