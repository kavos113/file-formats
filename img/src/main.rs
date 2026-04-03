use std::env;
use crate::render::renderer::Window;

pub mod fmt;
pub mod img;
pub mod reader;
pub mod render;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} [image_path]", args[0]);
        return;
    }

    let img = fmt::load_image(&args[1]);
    let mut window = Window::new(100, 100, img.width as i32, img.height as i32, &img);
    window.run();
}
