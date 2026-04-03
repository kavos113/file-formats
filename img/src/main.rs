use std::env;

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

    let _ = fmt::load_image(&args[1]);

    // let img = Image::sample_image(800, 600);
    // let mut window = Window::new(100, 100, 800, 600, &img);
    // window.run();
}
