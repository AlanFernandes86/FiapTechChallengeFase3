use actix_web::{web, put, delete, HttpResponse, Responder};

use crate::controllers::models::order_product::PutOrderProductDTO;

#[put("")]
pub async fn put_order_product(order_product_dto: web::Json<PutOrderProductDTO>) -> impl Responder {
    HttpResponse::Ok().json("put_order_product")
}

#[delete("/{order_product_id}")]
pub async fn delete_order_product(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json("delete_order_product")
}