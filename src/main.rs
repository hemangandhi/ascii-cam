extern crate image;

#[cfg(feature="webcam")]
extern crate rscam;

use std::env;
use std::fs;
use std::io::Write;

mod filters;

#[cfg(feature="webcam")]
fn convert_frame(frame: rscam::Frame, filters: &[filters::ImageFilter<image::Rgb<u8>>]) -> Option<Vec<u8>>{
    let buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = match image::ImageBuffer::from_vec(frame.resolution.0, frame.resolution.1, Vec::from(&frame[..])) {
        Some(ib) => ib,
        None => {
            println!("Wrong dimensions!");
            return None;
        }
    };

    let fin_buf = filters.iter().fold(buf, |b, f| f(&b));
    return Some(fin_buf.into_vec());
}

#[cfg(feature="webcam")]
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

fn run_from_file_to_file(in_path: &str, filts: &[& filters::ImageFilter<image::Rgb<u8>>], out_path: &str) {
    let in_buf = match image::open(in_path) {
        Ok(ib) => ib.to_rgb(),
        Err(e) => {
            println!("Couldn't read from {}: {}", in_path, e);
            return;
        }
    };

    match filts.iter().fold(in_buf, |b, f| f(b)).save(out_path) {
        Err(e) => {
            println!("Couldn't read to {}: {}", out_path, e);
        },
        Ok(()) => ()
    };
}

#[cfg(feature="webcam")]
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let file_name = &args[1];
            run_on_cam(file_name);
        },
        3 => {
            //just testing... run only the filters
            println!("TESTING FILTERS!!");
        },
        _ => {
            println!("USAGE: {} <file name for the webcam device>", args[0]);
            println!("FOR TESTS: {} <something> <something else>", args[0]);
        }
    }
}

#[cfg(not(feature="webcam"))]
fn main() {
    println!("Testing only! Build with --features \"webcam\" to run the other configuration.");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("USAGE: {} <input file> <output file>", &args[0]);
        return;
    }

    let carl = & *(filters::gaussian_blur(2.0));
    let carl2 = & *(filters::gaussian_blur(7.0));
    let marx = &*(filters::color_dist_lines(15));
    let downer = &*(filters::down_sample(5, 5));
    let filts: [& filters::ImageFilter<image::Rgb<u8>>; 3] = [carl, marx, downer];
    run_from_file_to_file(&args[1], &filts, &args[2]);
}
