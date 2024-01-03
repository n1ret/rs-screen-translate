use actix_web::{get, web::Data, Responder, Scope, HttpResponse};
use tera::Context;

use crate::structs::AppData;

#[get("/")]
async fn index(data: Data<AppData>) -> impl Responder {
    let ctx = Context::new();
    let rendered = data.tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/frames")]
async fn get_frames(_data: Data<AppData>) -> impl Responder {
    HttpResponse::BadGateway().finish()
    /*let frames = data.frames.read().unwrap();
    HttpResponse::Ok().body(frames.last().unwrap().clone())*/
}

#[get("/last")]
async fn last(data: Data<AppData>) -> impl Responder {
    let frames = data.frames.read().unwrap();
    HttpResponse::Ok().body(frames.last().unwrap().clone())
}

pub fn get_scope() -> Scope {
    Scope::new("")
        .service(index)
        .service(get_frames)
        .service(last)
}
