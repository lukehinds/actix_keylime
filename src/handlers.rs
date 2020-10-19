use actix_web::{get, post, HttpResponse, Responder};

#[get("/root")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/keys")]
pub async fn get_keys() -> impl Responder {
    HttpResponse::Ok().body("Hello get_keys!")
}

#[get("/quotes")]
pub async fn get_quotes() -> impl Responder {
    HttpResponse::Ok().body("Hello quotes!")
}

#[post("/keys")]
pub async fn post_keys() -> impl Responder {
    HttpResponse::Ok().body("Hello post_keys!")
}
