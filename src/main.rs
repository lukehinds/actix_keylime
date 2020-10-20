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

// GET:
// tentant -> /quotes/identity?nonce={nonce}

// tenant ->  /keys/verify?challenge={challenge}'
// challenge = TPM_Utilities.random_password(20)

// POST
// tenant -> /keys/ukey
// contains data=u_json_message
// data = {
//    'encrypted_key': b64_encrypted_u,
//    'auth_tag': self.auth_tag
//}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()x

//             .service(root)
//             .service(get_keys)
//             .service(get_quotes)
//             .service(post_keys)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
