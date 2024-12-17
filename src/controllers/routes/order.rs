use actix_web::web;
use crate::controllers::handlers::order::{create_order, delete_order_product, get_order_by_id, get_orders, put_order_product, update_order_status};

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/order")
            .service(get_order_by_id)
            .service(get_orders)
            .service(create_order)
            .service(update_order_status)
            .service(put_order_product)
            .service(delete_order_product)
    );
}