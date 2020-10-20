use actix_web::{get, post, HttpResponse, Responder};

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
