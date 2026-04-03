use std::fs::File;
use crate::fmt::central_directory::{CentralDirectory, EndOfCentralDirectoryRecord};

mod central_directory;
mod algorithm;

pub fn read_file(f: &mut File) {
    let record = EndOfCentralDirectoryRecord::find_record(f);
    println!("{}", record);

    let central_directory = CentralDirectory::from_record(f, &record);
    println!("{}", central_directory);
}