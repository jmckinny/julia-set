use image::{ImageBuffer, RgbImage};
fn main() {
    let mut img: RgbImage = ImageBuffer::new(8000, 6000);
    let cx = -0.9;
    let cy = 0.27015;
    let iterations = 500;
    let height = img.height() as f64;
    let width = img.width() as f64;
    for (cur_x, cur_y, pixel) in img.enumerate_pixels_mut() {
        let y = cur_y as f64;
        let x = cur_x as f64;
        let mut zx = 3.0 * (x - 0.5 * width) / (width);
        let mut zy = 2.0 * (y - 0.5 * height) / (height);
        let mut i = iterations;
        while zx * zx + zy * zy < 4.0 && i > 1 {
            let tmp = zx * zx - zy * zy + cx;
            zy = 2.0 * zx * zy + cy;
            zx = tmp;
            i -= 1;
        }
        let r = (i << 3) as u8;
        let g = (i << 5) as u8;
        let b = (i << 4) as u8;
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
    }
    img.save("test.png").unwrap();
}
