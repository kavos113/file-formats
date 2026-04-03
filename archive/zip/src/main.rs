use crate::fmt::read_file;
use std::env;
use std::fs::File;

mod fmt;
pub mod reader;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip_file>", args[0]);
        return;
    }

    let mut file = File::open(&args[1]).expect("Failed to open file");
    read_file(&mut file);
}
