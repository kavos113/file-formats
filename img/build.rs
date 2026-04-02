use std::env;
use std::fs::copy;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to find project root");

    copy("src/shader.hlsl", dest_path.join("shader.hlsl")).expect("Failed to copy shader.hlsl");
}
