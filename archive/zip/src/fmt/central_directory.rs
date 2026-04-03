use std::cmp::min;
use std::fmt::Display;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use crate::reader::Reader;

pub struct EndOfCentralDirectoryRecord {
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