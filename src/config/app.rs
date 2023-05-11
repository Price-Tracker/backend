use actix_web::web;
use crate::api::ping_controller;

pub fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(ping_controller::ping)
    );
}