use actix_web::{get, web::Data, Responder, Scope, HttpResponse};
use tera::Context;

use crate::structs::AppData;

#[get("/")]
async fn index(data: Data<AppData>) -> impl Responder {
    let ctx = Context::new();
    let rendered = data.tera.render("index", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/frames")]
async fn frames(data: Data<AppData>) -> impl Responder {
    HttpResponse::Ok().content_type("image/png")
        .body(data.frame.take())
}

pub fn get_scope() -> Scope {
    Scope::new("")
        .service(index)
        .service(frames)
}
