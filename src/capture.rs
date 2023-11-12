use std::io::Cursor;
use std::{time::{Instant, Duration}, cmp::Ordering};

use scrap::{Capturer, Display};
use image::{RgbImage, ImageOutputFormat::Png};

use crate::structs::AppData;

pub async fn start_capture(data: &AppData) {
    let mut capturer = Capturer::new(Display::primary().unwrap()).unwrap();
    
    let mut next_second = Instant::now() + Duration::from_secs(1);
    let mut t = 0;
    //let mut interval = time::interval(Duration::from_secs(10));
    loop {
        let frame = capturer.frame().unwrap();
        let image = RgbImage::from_raw(1920, 1080, frame.to_vec()).unwrap();
        let mut cur = Cursor::new(Vec::new());
        
        let _ = image.write_to(&mut cur, Png);
        data.frame.set(cur.into_inner());
        t += 1;
        
        if Instant::now().cmp(&next_second) != Ordering::Less {
            next_second = Instant::now() + Duration::from_secs(1);
            println!("{t}");
            t = 0;
        }
    }
}
