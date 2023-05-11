use actix_web::web;
use crate::api::*;

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
            .service(
                web::scope("/user")
                    .service(account_controller::signup)
            )
    );
}