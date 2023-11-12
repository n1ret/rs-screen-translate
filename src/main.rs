mod capture;
mod routes;
mod structs;

use actix_web::{HttpServer, App, web::Data, rt::spawn};
use actix_files as fs;
use tera::Tera;

use crate::routes::get_scope;
use crate::structs::AppData;
use crate::capture::start_capture;

#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| {
        let templates = Tera::new("templates/**/*").unwrap();
        let data = AppData::new(templates);

        //let _ = spawn(start_capture(&data));

        App::new()
            .app_data(data)
            .service(fs::Files::new("/static", "static/"))
            .service(get_scope())
    })
    .bind(("0.0.0.0", 8080)).unwrap()
    .run().await;
}
