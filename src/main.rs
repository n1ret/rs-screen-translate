mod capture;
mod routes;
mod structs;

use std::thread::spawn;
use std::sync::mpsc;

use actix_web::{HttpServer, App, web::Data};
use actix_files as fs;
use tera::Tera;

use crate::routes::get_scope;
use crate::structs::AppData;
use crate::capture::start_capture;

#[actix_web::main]
async fn main() {
    let tera = Tera::new("templates/**/*").unwrap();
    let data = Data::new(AppData::new(tera));

    let (tx, rx) = mpsc::channel::<()>();
    let data_clone = data.clone();
    spawn(|| start_capture(data_clone, rx));
    
    let _ = HttpServer::new(move || {
        App::new()
        .app_data(data.clone())
        .service(fs::Files::new("/static", "static/"))
        .service(get_scope())
    })
    .bind(("0.0.0.0", 8080)).unwrap()
    .run().await;

    tx.send(()).unwrap();
}
