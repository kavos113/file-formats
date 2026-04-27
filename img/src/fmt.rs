use img_renderer::Image;

mod bmp;
mod ppm;

pub fn load_image(path: &str) -> Image {
    if path.ends_with(".bmp") {
        bmp::load_bmp(path)
    } else if path.ends_with(".ppm") {
        ppm::load_ppm(path)
    } else {
        panic!("Unsupported image format: {}", path);
    }
}