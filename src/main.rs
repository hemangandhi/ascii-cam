extern crate image;
extern crate rscam;
use std::env;
use std::fs;
use std::io::Write;

fn run_on_cam(cam: &str){
    let mut camera = match rscam::Camera::new(cam) {
        Ok(c) => c,
        Err(e) => {
            println!("Error in making camera: {}", e);
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

    let file_name = &args[0];
    run_on_cam(file_name);
}
