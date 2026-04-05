use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use crate::fmt::algorithm::decompress_file;
use crate::fmt::central_directory::{CentralDirectory, EndOfCentralDirectoryRecord};

mod central_directory;
mod algorithm;

pub fn read_file(mut f: File) {
    let record = EndOfCentralDirectoryRecord::find_record(&mut f);
    println!("{}", record);

    let central_directory = CentralDirectory::from_record(&mut f, &record);
    println!("{}", central_directory);

    let mut r = BufReader::new(f);
    for header in &central_directory.headers {
        r.seek(SeekFrom::Start(header.local_header_offset as u64)).unwrap();
        decompress_file(&mut r, header);
    }
}