use image::{Rgb, RgbImage};

pub fn rgb_to_image(buf: &[u8], w: u16, h: u16) -> RgbImage {
    let mut img = RgbImage::new(w as u32, h as u32);
    for y in 0..h {
        for x in 0..w {
            let i = ((y * w + x) * 3) as usize;
            img.put_pixel(x as u32, y as u32, Rgb([buf[i], buf[i+1], buf[i+2]]));
        }
    }
    img
}
