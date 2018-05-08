extern crate image;
extern crate rscam;
use std::env;
use std::fs;
use std::io::Write;

type ImageFilter<P: image::Pixel> = fn(u32, u32, image::ImageBuffer<P, Vec<P::Subpixel>>, P) -> P;

fn color_dist(this: image::Rgb<u8>, other: image::Rgb<u8>) -> u32{
    let d_x = (this.data[0] - other.data[0]) as f64;
    let d_y = (this.data[1] - other.data[1]) as f64;
    let d_z = (this.data[2] - other.data[2]) as f64;
    return (d_x * d_x + d_y * d_y + d_z * d_z).sqrt() as u32;
}

fn convert_frame(frame: rscam::Frame, filters: &[ImageFilter<image::Rgb<u8>>]) -> Option<Vec<u8>>{
    let mut buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = match image::ImageBuffer::from_vec(frame.resolution.0, frame.resolution.1, Vec::from(&frame[..])) {
        Some(ib) => ib,
        None => {
            println!("Wrong dimensions!");
            return None;
        }
    };

    let inner_fn = |x, y| filters.iter().fold(*buf.get_pixel(x, y), |p, f| f(x, y, buf, p));
    let new_buf = image::ImageBuffer::from_fn(frame.resolution.0, frame.resolution.1, inner_fn);

    return Some(new_buf.into_vec());
}

fn run_on_cam(cam: &str){
    let mut camera = match rscam::Camera::new(cam) {
        Ok(c) => c,
        Err(e) => {
            println!("Error in making camera at {}: {}", cam, e);
            return;
        }
    };

    match camera.start(&rscam::Config{
        interval: (1, 30),
        resolution: (1280, 720),
        format: b"MJPG",
        ..Default::default()
    }){
        Ok(()) => println!("Streaming!"),
        Err(e) => {
            println!("Error in starting camera: {}", e);
            return;
        }
    };

    for i in 1..10 {
        let frame = match camera.capture() {
            Ok(f) => f,
            Err(e) => {
                println!("Error in capture {}: {}", i, e);
                return;
            }
        };
        let mut file = fs::File::create(format!("frame-{}.jpg", i)).unwrap();
        file.write_all(&frame[..]).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: {} <file name for the webcam device>", args[0]);
        return;
    }

    let file_name = &args[1];
    run_on_cam(file_name);
}
