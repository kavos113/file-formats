use std::env;
use img_renderer_rs::{Image, ImgRenderer};

pub mod fmt;
pub mod reader;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let img = match args.len() {
        1 => {
            Image::sample_image(800, 600)
        }

        _ => {
            fmt::load_image(&args[1])
        }
    };

    let mut renderer = ImgRenderer::new(100, 100, img.width as i32, img.height as i32, &img);
    renderer.run();
}
