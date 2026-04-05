use std::fs::File;
use crate::reader::BitReader;

pub fn analyze_file(r: &mut BitReader, out: &mut File) {
    let is_final = r.read_bits(1);
    let block_type = r.read_bits(2);
}