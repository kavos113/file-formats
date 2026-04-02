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
