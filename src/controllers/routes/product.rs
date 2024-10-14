use actix_web::web;
use crate::controllers::handlers::product::{ get_product_by_category_id, put_product };

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/product")
            .service(get_product_by_category_id)
            .service(put_product)
    );
}