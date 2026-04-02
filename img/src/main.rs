use crate::img::Image;
use crate::render::renderer::Window;

pub mod fmt;
pub mod img;
pub mod reader;
pub mod render;

fn main() {
    let img = Image::sample_image(800, 600);
    let mut window = Window::new(100, 100, 800, 600, &img);
    window.run();
}
