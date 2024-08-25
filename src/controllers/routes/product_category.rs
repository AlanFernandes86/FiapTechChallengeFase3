use actix_web::web;
use crate::controllers::handlers::product_category::{ delete_product_category, get_product_categories, put_product_category };

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/product_category")
            .service(get_product_categories)
            .service(put_product_category)
            .service(delete_product_category)
    );
}