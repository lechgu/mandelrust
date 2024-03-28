mod mandelbrot;
use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use crate::mandelbrot::get_image;

#[get("/")]
async fn index() -> impl Responder {
    match get_image() {
        Ok(bytes) => HttpResponse::Ok().content_type("image/png").body(bytes),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = get_port();
    let addr = format!(":::{}", port);
    HttpServer::new(|| App::new().service(index))
        .bind(addr)?
        .run()
        .await
}

fn get_port() -> u16 {
    match env::var("PORT") {
        Ok(value) => match value.parse::<u16>() {
            Ok(port) => port,
            Err(_) => 80,
        },
        Err(_) => 80,
    }
}
