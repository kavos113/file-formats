use std::fs;
use std::str::Lines;
use crate::img::Image;

pub fn load_ppm(path: &str) -> Image {
    let data = fs::read_to_string(path).expect("Failed to read file");
    let mut lines = data.lines();

    let (width, height) = parse_header(&mut lines);

    let mut pixels = Vec::with_capacity((width * height * 3) as usize);
    for line in lines {
        for value in line.split_whitespace() {
            let val: u8 = value.parse().expect("Invalid pixel value");
            pixels.push(val);
        }
    }

    Image { width, height, data: pixels }
}

fn parse_header(lines: &mut Lines) -> (u32, u32) {
    let magic_number = lines.next().expect("Missing magic number");
    if magic_number != "P3" {
        panic!("Unsupported format: {}", magic_number);
    }

    let dimensions = lines.next().expect("Missing dimensions");
    let mut dims = dimensions.split_whitespace();
    let width: u32 = dims.next().expect("Missing width").parse().expect("Invalid width");
    let height: u32 = dims.next().expect("Missing height").parse().expect("Invalid height");

    (width, height)
}

pub fn save_ppm(image: &Image, path: &str) {
    let mut data = format!("P3\n{} {}\n255\n", image.width, image.height);
    for chunk in image.data.chunks(3) {
        data.push_str(&format!("{} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    fs::write(path, data).expect("Failed to write file");
}