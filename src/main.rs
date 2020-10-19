use actix_web::{App, HttpServer};
mod handlers;
use handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(get_keys)
            .service(get_quotes)
            .service(post_keys)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
