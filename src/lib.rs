mod parser;

use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::web::Json;
use actix_web::{error, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use derive_more::{Display, Error};
use parser::hex_to_rgb;
use pigment_mixing::mix_srgb_u8;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Debug, Display, Error, PartialEq)]
#[display(fmt = "ColorMixerError: {}", msg)]
pub struct ColorMixerError {
    msg: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ColorMixerError {}

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/greet/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RgbColor {
    pub color: [u8; 3],
}

#[get("/parse/{color}")]
async fn parse(path: web::Path<String>) -> Result<Json<RgbColor>, ColorMixerError> {
    match hex_to_rgb(&path.into_inner()) {
        Ok(color) => Ok(Json(RgbColor { color })),
        Err(err) => Err(err),
    }
}

#[get("/mix/{color1}/{color2}/{ratio}")]
async fn mix(path: web::Path<(String, String, f32)>) -> Result<Json<RgbColor>, ColorMixerError> {
    let (color1, color2, ratio) = path.into_inner();
    match (hex_to_rgb(&color1), hex_to_rgb(&color2)) {
        (Ok(color1), Ok(color2)) => {
            let mix = mix_srgb_u8(&color1, &color2, ratio);
            Ok(Json(RgbColor { color: mix }))
        }
        (Err(err1), _) => Err(err1),
        (_, Err(err2)) => Err(err2),
    }
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(health_check)
            .service(greet)
            .service(parse)
            .service(mix)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
