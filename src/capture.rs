use std::io::Cursor;
use std::time::{Instant, Duration};

use actix_web::web::Data;
use scrap::{Capturer, Display};
use image::{ImageOutputFormat, ImageBuffer, Rgb};
use base64::{Engine, engine::general_purpose};

use crate::structs::AppData;

pub async fn start_capture(data: Data<AppData>) {
    let display = Display::primary().unwrap();
    let (width, height) = (display.width() as u32, display.height() as u32);
    let mut capturer = Capturer::new(display).unwrap();

    let base64 = general_purpose::STANDARD;
    
    let mut next_second = Instant::now() + Duration::from_secs(1);
    let mut frames_num = 0;
    //let mut interval = time::interval(Duration::from_secs(10));

    loop {
        let frame: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(
            width, height, capturer.frame().unwrap().to_vec()
        ).unwrap();
        frame.save("1.png").unwrap();
        //let mut cur = Cursor::new(Vec::new());
        //rame.write_to(&mut cur, ImageOutputFormat::Png).unwrap();
        //let b64_encoded: String = base64.encode(cur.into_inner());

        //data.frames.lock().unwrap().push(b64_encoded);
        frames_num += 1;

        if Instant::now() >= next_second {
            next_second = Instant::now() + Duration::from_secs(1);
            println!("{frames_num}");
            frames_num = 0;
        }
    }
}
