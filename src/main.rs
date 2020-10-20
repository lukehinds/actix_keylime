use actix_web::{web, App, HttpServer};

mod keys_handler;
mod quotes_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::resource("/keys/verify").route(web::get().to(keys_handler::verify)))
            .service(web::resource("/keys/ukey").route(web::post().to(keys_handler::ukey)))
            .service(
                web::resource("/quotes/identity").route(web::get().to(quotes_handler::identity)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
