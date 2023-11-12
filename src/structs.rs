use tera::Tera;
use std::cell::Cell;

pub struct AppData {
    pub tera: Tera,
    pub frame: Cell<Vec<u8>>
}

impl AppData {
    pub fn new(tera: Tera) -> AppData {
        AppData {
            tera,
            frame: Cell::new(vec![])
        }
    }
}
