use ntex::web::{get, HttpResponse, ServiceConfig, Responder};
use crate::HANDLEBARS;

#[derive(Debug, serde::Serialize)]
struct SomeData {
    message: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    let data = SomeData {
        message: "Hello World".to_string(),
    };

    let body = HANDLEBARS.render("hello", &data).unwrap();

    HttpResponse::Ok().body(body)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(hello);
}