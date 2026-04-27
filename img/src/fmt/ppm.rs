use std::fs;
use std::str::Lines;
use img_renderer::{Image, Pixel};

pub fn load_ppm(path: &str) -> Image {
    let data = fs::read_to_string(path).expect("Failed to read file");
    let mut lines = data.lines();

    let (width, height) = parse_header(&mut lines);

    let mut pixels = Vec::with_capacity((width * height) as usize);
    for line in lines {
        let mut values = line.split_whitespace();
        if let (Some(r), Some(g), Some(b)) = (values.next(), values.next(), values.next()) {
            let r: u8 = r.parse().expect("Invalid red value");
            let g: u8 = g.parse().expect("Invalid green value");
            let b: u8 = b.parse().expect("Invalid blue value");
            pixels.push(Pixel {
                r,
                g,
                b,
                a: 255, // Default alpha value
            });
        }
    }

    Image {
        width,
        height,
        data: pixels,
    }
}

fn parse_header(lines: &mut Lines) -> (u32, u32) {
    let magic_number = lines.next().expect("Missing magic number");
    if magic_number != "P3" {
        panic!("Unsupported format: {}", magic_number);
    }

    let dimensions = lines.next().expect("Missing dimensions");
    let mut dims = dimensions.split_whitespace();
    let width: u32 = dims
        .next()
        .expect("Missing width")
        .parse()
        .expect("Invalid width");
    let height: u32 = dims
        .next()
        .expect("Missing height")
        .parse()
        .expect("Invalid height");

    (width, height)
}

pub fn save_ppm(image: &Image, path: &str) {
    let mut data = format!("P3\n{} {}\n255\n", image.width, image.height);
    for pixel in &image.data {
        data.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
    }
    fs::write(path, data).expect("Failed to write file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_and_save_ppm() {
        let image = Image {
            width: 2,
            height: 2,
            data: vec![
                Pixel {
                    r: 255,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                Pixel {
                    r: 0,
                    g: 255,
                    b: 0,
                    a: 255,
                },
                Pixel {
                    r: 0,
                    g: 0,
                    b: 255,
                    a: 255,
                },
                Pixel {
                    r: 255,
                    g: 255,
                    b: 0,
                    a: 255,
                },
            ],
        };
        save_ppm(&image, "test_output.ppm");

        let loaded_image = load_ppm("test_output.ppm");

        assert_eq!(image.width, loaded_image.width);
        assert_eq!(image.height, loaded_image.height);
        assert_eq!(image.data, loaded_image.data);
    }
}
