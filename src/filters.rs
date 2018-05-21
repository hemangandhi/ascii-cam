extern crate image;

use std::f64::consts::PI;
use std::ops::Sub;

fn super_safe_sub<T: Sub<Output=T> + Ord>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    } else {
        return y - x;
    }
}

fn color_dist(this: &image::Rgb<u8>, other: &image::Rgb<u8>) -> u32{
    let d_x = super_safe_sub(this.data[0], other.data[0]) as f64;
    let d_y = super_safe_sub(this.data[1], other.data[1]) as f64;
    let d_z = super_safe_sub(this.data[2], other.data[2]) as f64;
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

pub fn color_dist_lines(dist: u32 ) -> Box<ImageFilter<image::Rgb<u8>>> {
    return Box::new(move |ib| {
        return image::ImageBuffer::from_fn(ib.width(), ib.height(), |x, y| {
            for i in (x as i32) - 1 .. (x as i32) + 1 {
                for j in (y as i32) - 1 .. (y as i32) + 1 {
                    if i < 0 || i >= (ib.width() as i32) || j < 0 || j >= (ib.height() as i32) {
                        continue;
                    }

                    if color_dist(ib.get_pixel(x, y), ib.get_pixel(i as u32, j as u32)) > dist {
                        return image::Rgb{
                            data: [0, 0, 0]
                        };
                    }
                }
            }

            return image::Rgb{
                data: [255, 255, 255]
            };
        });
    });
}

pub fn down_sample(hscale: u32, wscale: u32) -> Box<ImageFilter<image::Rgb<u8>>> {
    return Box::new(move |ib| {
        return image::ImageBuffer::from_fn(ib.width() / wscale, ib.height() / hscale, |x, y| {
            let top_left_x = wscale * x;
            let top_left_y = hscale * y;
            let bottom_right_x = wscale * (x + 1) - 1;
            let bottom_right_y = hscale * (y + 1) - 1;
            let area = ((bottom_right_x - top_left_x) * (bottom_right_y - top_left_y)) as f64;

            let mut pix: [f64; 3] = [0.0, 0.0, 0.0];
            for i in top_left_x .. bottom_right_x {
                for j in top_left_y .. bottom_right_y {
                    let p = ib.get_pixel(i, j);
                    pix[0] += (p[0] as f64)/area;
                    pix[1] += (p[1] as f64)/area;
                    pix[2] += (p[2] as f64)/area;
                }
            }

            return image::Rgb{
                data: [pix[0] as u8, pix[1] as u8, pix[2] as u8]
            };
        });
    });
}
