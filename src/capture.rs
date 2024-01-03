use std::sync::mpsc::{self, TryRecvError};
use std::thread::sleep;
use std::io::{Cursor, prelude::*};
use std::time::{Instant, Duration};

use actix_web::web::Data;
use scrap::{Capturer, Display};
use image::{ImageOutputFormat, ImageBuffer, Rgb};
use base64::{Engine, engine::general_purpose};

use crate::structs::AppData;

const FPS: u64 = 10;

pub fn start_capture(data: Data<AppData>, rx: mpsc::Receiver<()>) {
    let display = Display::primary().unwrap();
    let (width, height) = (display.width() as u32, display.height() as u32);
    let mut capturer = Capturer::new(display).unwrap();

    let mut next_second = Instant::now() + Duration::from_secs(1);
    let mut frames_num = 0;

    let frame_delta = Duration::from_millis(1000 / FPS);
    let mut next_frame = Instant::now()+frame_delta;

    loop {
        let frame = capturer.frame().unwrap();
        let mut bitflipped = Vec::with_capacity(frame.len()/4*3);

        for i in (0..frame.len()).step_by(4) {
            bitflipped.extend_from_slice(&[
                frame[i + 2],
                frame[i + 1],
                frame[i],
            ]);
        }
        let screenshot: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
            width, height, bitflipped
        ).unwrap();

        let mut image_bytes = Vec::new();
        screenshot.write_to(&mut Cursor::new(&mut image_bytes), ImageOutputFormat::Png).unwrap();

        let b64_encoded = general_purpose::STANDARD.encode(&image_bytes);
        let mut frames = data.frames.write().unwrap();
        frames.clear();
        frames.push(b64_encoded);

        frames_num += 1;

        if Instant::now() >= next_second {
            next_second = Instant::now() + Duration::from_secs(1);
            print!("\rCurrent fps: {frames_num}"); std::io::stdout().flush().unwrap();
            frames_num = 0;
        }

        sleep(next_frame - Instant::now());
        next_frame += frame_delta;

        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}
