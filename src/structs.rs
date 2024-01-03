use tera::Tera;
use std::sync::RwLock;

pub struct AppData {
    pub tera: Tera,
    pub frames: RwLock<Vec<String>>
}

impl AppData {
    pub fn new(tera: Tera) -> AppData {
        AppData {
            tera,
            frames: RwLock::new(vec![])
        }
    }
}
