use std::env;

use ffmpeg_sidecar::{command::FfmpegCommand, event::FfmpegEvent};
const SCALE: u32 = 1;
use std::fs;

fn main() {
    ffmpeg_sidecar::download::auto_download().unwrap();
    // let input_path = ".mp4";
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];

    // ffmpeg -i input.mp4 -vf fps=1 out%d.png
    let factor = 1;
    let frame_width: u32 = 272 / factor; // 480
    let frame_height: u32 = 72 / factor; // 360
    let mut input = FfmpegCommand::new()
        .args(["-i", input_path])
        .args(["-vf", "fps=60"])
        .args(["output/out%d.png"])
        .spawn()
        .expect("could not start thread");
    fs::create_dir_all("output").expect("couldnt create folder");
    input
        .iter()
        .expect("couldnt get iterator")
        .for_each(move |event| match event {
            FfmpegEvent::OutputFrame(frame) => {
                println!("frame: {}x{}", frame.width, frame.height);
                // let _pixels: Vec<u8> = frame.data; // <- raw RGB pixels! 🎨
            }
            FfmpegEvent::Progress(progress) => {
                eprintln!("Current speed: {}x", progress.speed); // <- parsed progress updates
            }
            FfmpegEvent::Log(_level, msg) => {
                eprintln!("[ffmpeg] {}", msg); // <- granular log message from stderr
            }
            FfmpegEvent::Done => {
                println!("done")
            }
            _ => (),
        });
    let paths = fs::read_dir("output").unwrap();
    let mut count: u32 = 0;
    for path in paths {
        if path.is_ok() {
            count += 1;
        }
    }
    for idx in 1..=count {
        let img = image::open(format!("output/out{idx}.png")).expect("couldnt load pic");
        let conf = viuer::Config {
            // set offset
            x: 0,
            y: 0,
            // set dimensions
            restore_cursor: true,
            width: Some(frame_width * SCALE),
            height: Some(frame_height * SCALE),
            ..Default::default()
        };
        viuer::print(&img, &conf).expect("Image printing failed.");
    }
}
