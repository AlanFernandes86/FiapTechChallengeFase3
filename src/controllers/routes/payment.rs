use actix_web::web;
use crate::controllers::handlers::payment::set_payment;

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/order")
            .service(set_payment)
    );
}