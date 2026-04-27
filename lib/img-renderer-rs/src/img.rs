pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Pixel>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Image {
    pub fn sample_image(width: u32, height: u32) -> Self {
        let mut data = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                let r = (x * 255 / width) as u8;
                let g = (y * 255 / height) as u8;
                let b = 128;
                let a = 255;
                data.push(Pixel { r, g, b, a });
            }
        }
        Self {
            width,
            height,
            data,
        }
    }
}
