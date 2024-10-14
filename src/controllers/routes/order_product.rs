use actix_web::web;
use crate::controllers::handlers::order_product::{delete_order_product, put_order_product};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/order_product")
            .service(put_order_product)
            .service(delete_order_product)
    );
}