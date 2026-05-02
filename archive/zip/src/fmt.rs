use crate::fmt::algorithm::decompress_file;
use crate::fmt::central_directory::{CentralDirectory, EndOfCentralDirectoryRecord};
use std::fs::File;
use std::io::{stdout, BufReader, Seek, SeekFrom, Write};

mod algorithm;
mod central_directory;

pub fn read_file(mut f: File) {
    let record = EndOfCentralDirectoryRecord::find_record(&mut f);
    println!("{}", record);

    let central_directory = CentralDirectory::from_record(&mut f, &record);
    println!("{}", central_directory);

    let mut i = 0;
    let mut r = BufReader::new(f);
    for header in &central_directory.headers {
        r.seek(SeekFrom::Start(header.actual_header_offset()))
            .unwrap();
        decompress_file(&mut r, header);

        i += 1;
        eprint!("\rDecompressed: {}/{}", i, central_directory.headers.len());
        stdout().flush().unwrap();
    }
}
