use std::env;
use img_renderer_rs::ImgRenderer;

pub mod fmt;
pub mod reader;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} [image_path]", args[0]);
        return;
    }

    let img = fmt::load_image(&args[1]);
    let mut renderer = ImgRenderer::new(100, 100, img.width as i32, img.height as i32, &img);
    renderer.run();
}
