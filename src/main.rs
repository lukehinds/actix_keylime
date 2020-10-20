use actix_web::{middleware, web, App, HttpServer};

mod keys_handler;
mod quotes_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/quotes")
                .service(web::resource("/identity").route(web::get().to(quotes_handler::identity))),
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
