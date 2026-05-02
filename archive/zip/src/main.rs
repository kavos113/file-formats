use crate::fmt::read_file;
use std::env;
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

mod fmt;
pub mod reader;
pub mod writer;

pub static ENABLE_DEBUG: AtomicBool = AtomicBool::new(false);

#[macro_export]
macro_rules! dbg_println {
    ($($arg:tt)*) => {
        if crate::ENABLE_DEBUG.load(Ordering::Relaxed) {
            println!($($arg)*);
        }
    };
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <zip_file>", args[0]);
        return;
    } else if args.len() > 2 && args[2] == "--debug" {
        ENABLE_DEBUG.store(true, Ordering::Relaxed);
    }

    let start = Instant::now();

    let file = File::open(&args[1]).expect("Failed to open file");
    read_file(file);

    let duration = start.elapsed();
    println!("\nDecompression completed in {:.2?}", duration);
}
