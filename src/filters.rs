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
    fn gauss_wt(x: i32, y: i32, cx: i32, cy: i32, sigma: f64) -> f64 {
        let dist = - (((cx - x) * (cx - x) + (cy - y) * (cy - y)) as f64 / (2.0 * sigma * sigma));
        return dist.exp() / (sigma * sigma * 2.0 * PI);
    }

    let rv = move |ib: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>| {
        let inner = |x: u32, y: u32| {
            let ker_size = (6.0 * sigma) as i32;
            let mut pix: [f64; 3] = [0.0, 0.0, 0.0];
            for i in (x as i32 - ker_size)..(x as i32 + ker_size) {
                for j in (y as i32 - ker_size)..(y as i32 + ker_size) {
                    if i < 0 || i >= (ib.width() as i32) || j < 0 || j >= (ib.height() as i32) {
                        continue;
                    }

                    let p = ib.get_pixel(i as u32, j as u32);
                    let gauss = gauss_wt(i, j, x as i32, y as i32, sigma);
                    pix[0] = (p.data[0] as f64) * gauss + pix[0];
                    pix[1] = (p.data[1] as f64) * gauss + pix[1];
                    pix[2] = (p.data[2] as f64) * gauss + pix[2];
                }
            }
            return image::Rgb{
                data: [pix[0] as u8, pix[1] as u8, pix[2] as u8]
            };
        };
        return image::ImageBuffer::from_fn(ib.width(), ib.height(), inner);
    };
    return Box::new(rv);
}
