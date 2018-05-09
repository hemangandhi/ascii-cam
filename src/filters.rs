extern crate image;

use std::f64::consts::PI;

fn color_dist(this: image::Rgb<u8>, other: image::Rgb<u8>) -> u32{
    let d_x = (this.data[0] - other.data[0]) as f64;
    let d_y = (this.data[1] - other.data[1]) as f64;
    let d_z = (this.data[2] - other.data[2]) as f64;
    return (d_x * d_x + d_y * d_y + d_z * d_z).sqrt() as u32;
}

pub type ImageFilter<P: image::Pixel> = Fn(image::ImageBuffer<P, Vec<P::Subpixel>>) -> image::ImageBuffer<P, Vec<P::Subpixel>>;

pub fn gaussian_blur(sigma: f64) -> Box<ImageFilter<image::Rgb<u8>>> {
    fn gaiss_wt(x: i32, y: i32, cx: i32, cy: i32, sigma: f64) -> f64 {
        let dist = - (((cx - x) * (cx - x) + (cy - y) * (cy - y)) as f64 / (2.0 * sigma * sigma));
        return dist.exp() / (sigma * sigma * 2.0 * PI);
    }

    let ker_size = 6.0 * sigma;
    let rv = |ib: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>| {
        let inner = |x, y| {
            *ib.get_pixel(x, y)
        };
        return image::ImageBuffer::from_fn(ib.width(), ib.height(), inner);
    };
    return Box::new(rv);
}
