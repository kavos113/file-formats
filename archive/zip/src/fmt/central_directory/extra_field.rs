use std::fmt::{Display, Formatter};
use std::time::{Duration, UNIX_EPOCH};
use chrono::{Date, DateTime, Utc};
use crate::reader::Reader;

pub enum ExtraField {
    Zip64(ExtraFieldZip64),
    OS2(ExtraFieldOS2),
    NTFS(ExtraFieldNTFS),
    ExtendedTimestamp(ExtraFieldExtendedTimestamp)
}

impl ExtraField {
    pub fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        reader.seek_from_current(-2);
        match header_id {
            0x0001 => ExtraField::Zip64(ExtraFieldZip64::read_from(reader)),
            0x000d => ExtraField::OS2(ExtraFieldOS2::read_from(reader)),
            0x000a => ExtraField::NTFS(ExtraFieldNTFS::read_from(reader)),
            0x5455 => ExtraField::ExtendedTimestamp(ExtraFieldExtendedTimestamp::read_from(reader)),
            _ => panic!("Unknown extra field header ID: 0x{:04x}", header_id),
        }
    }
}

impl Display for ExtraField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtraField::Zip64(zip64) => write!(f, "{}", zip64)?,
            ExtraField::OS2(os2) => write!(f, "{}", os2)?,
            ExtraField::NTFS(ntfs) => write!(f, "{}", ntfs)?,
            ExtraField::ExtendedTimestamp(ext_ts) => write!(f, "{}", ext_ts)?,
        }

        Ok(())
    }
}

pub struct ExtraFieldZip64 {
    pub header_id: u16,
    pub data_size: u16,
    pub original_size: u64,
    pub compressed_size: u64,
    pub local_header_offset: u64,
    pub disk_start_number: u32,
}

impl ExtraFieldZip64 {
    fn read_from(reader: &mut Reader) -> Self {
        let header_id = reader.read_u16();
        if header_id != 0x0001 {
            panic!("Invalid Zip64 extra field header ID: 0x{:04x}", header_id);
        }

        let data_size = reader.read_u16();

        let mut remain_data_size = data_size;

        let original_size = reader.read_u64();
        let compressed_size = reader.read_u64();
        remain_data_size -= 16;

        let local_header_offset = if remain_data_size >= 8 {
            remain_data_size -= 8;
            reader.read_u64()
        } else {
            0
        };
        let disk_start_number = if remain_data_size >= 4 {
            remain_data_size -= 4;
            reader.read_u32()
        } else {
            0
        };

        if remain_data_size != 0 {
            panic!("Invalid Zip64 extra field data size: {}", data_size);
        }

        Self {
            header_id,
            data_size,
            original_size,
            compressed_size,
            local_header_offset,
            disk_start_number,
        }
    }
}

impl Display for ExtraFieldZip64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "---------- Zip64 Extended Information Extra Field ----------"
        )?;
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
    block: Vec<u8>,
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
            block,
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
    attribute_block: Vec<ExtraFieldNTFSAttributeBlock>,
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
            attribute_block,
        }
    }
}

struct ExtraFieldNTFSAttributeBlock {
    tag: u16,
    size: u16,
    mod_time: u64,
    access_time: u64,
    create_time: u64,
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
            create_time,
        }
    }
}

fn format_ntfs_time(ntfs_time: u64) -> String {
    if ntfs_time == 0 {
        return "N/A".to_string();
    }
    let unix_time = (ntfs_time / 10_000_000) - 11644473600;
    let datetime = Duration::from_secs(unix_time);
    let datetime: DateTime<Utc> = DateTime::from(UNIX_EPOCH + datetime);
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
            writeln!(
                f,
                "  Last Modified Time:  {}",
                format_ntfs_time(block.mod_time)
            )?;
            writeln!(
                f,
                "  Last Access Time:    {}",
                format_ntfs_time(block.access_time)
            )?;
            writeln!(
                f,
                "  Creation Time:       {}",
                format_ntfs_time(block.create_time)
            )?;
        }

        Ok(())
    }
}

struct ExtraFieldExtendedTimestamp {
    tag: u16,
    size: u16,
    flags: u8,
    mod_time: u32
}

impl ExtraFieldExtendedTimestamp {
    fn read_from(reader: &mut Reader) -> Self {
        let tag = reader.read_u16();
        if tag != 0x5455 {
            panic!("invalid tag: {:?}", tag)
        }

        let size = reader.read_u16();
        let flags = reader.read_u8();
        let mod_time = reader.read_u32();

        Self {
            tag,
            size,
            flags,
            mod_time
        }
    }
}

impl Display for ExtraFieldExtendedTimestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let datetime: DateTime<Utc> = DateTime::from(UNIX_EPOCH + Duration::from_secs(self.mod_time as u64));

        writeln!(f, "---------- Extended Timestamp Extra Field ----------")?;
        writeln!(f, "Header ID:              0x{:04x}", self.tag)?;
        writeln!(f, "Data Size:              {}", self.size)?;
        writeln!(f, "Modification Time:      {}", datetime.format("%Y-%m-%d %H:%M:%S"))?;

        writeln!(f, "Flags:")?;
        if self.flags & 0x01 != 0 {
            writeln!(f, "  - Modification Time Present")?;
        }
        if self.flags & 0x02 != 0 {
            writeln!(f, "  - Access Time Present")?;
        }
        if self.flags & 0x04 != 0 {
            writeln!(f, "  - Creation Time Present")?;
        }

        Ok(())
    }
}