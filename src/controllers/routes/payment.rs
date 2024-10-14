use actix_web::web;
use crate::controllers::handlers::payment::{mercado_pago_notification, start_payment};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/payment")
            .service(start_payment)
            .service(mercado_pago_notification)
    );
}