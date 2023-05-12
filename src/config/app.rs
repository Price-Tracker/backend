use actix_cors::Cors;
use actix_jwt_auth_middleware::{Authority, TokenSigner};
use actix_jwt_auth_middleware::use_jwt::UseJWTOnScope;
use actix_web::{http, web};
use exonum_crypto::KeyPair;
use jwt_compact::alg::Ed25519;
use crate::api::*;
use crate::models::user::UserClaims;

pub fn get_cors() -> Cors {
    Cors::default()
        .allowed_origin_fn(|origin, _req_head| {
            origin.as_bytes().starts_with(b"https://infinity.tail1f457.ts.net")
        })
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    // TODO: Maybe we should switch to static keys?
    let key_pair = KeyPair::random();
    let authority = Authority::<UserClaims, Ed25519, _, _>::new()
        .refresh_authorizer(|| async move { Ok(()) })
        .token_signer(Some(
            TokenSigner::new()
                .signing_key(key_pair.secret_key().clone())
                .algorithm(Ed25519)
                .build()
                .expect(""),
        ))
        .verifying_key(key_pair.public_key())
        .build()
        .expect("");

    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/user")
                    .service(account_controller::signup)
                    .service(account_controller::login)
            )
            .use_jwt(
                authority, web::scope("")
                    .service(ping_controller::ping),
            )
    );
}