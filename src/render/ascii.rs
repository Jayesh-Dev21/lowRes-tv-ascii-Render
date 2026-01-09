use image::RgbImage;
use pixel2ascii::{image_to_ascii, AsciiOptions};

pub fn render(img: &RgbImage, cfg: &AsciiOptions) -> String {
    image_to_ascii(img, cfg)
}
