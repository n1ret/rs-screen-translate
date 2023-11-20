use tera::Tera;
use std::sync::Mutex;

pub struct AppData {
    pub tera: Tera,
    pub frames: Mutex<Vec<String>>
}

impl AppData {
    pub fn new(tera: Tera) -> AppData {
        AppData {
            tera,
            frames: Mutex::new(vec![])
        }
    }
}
