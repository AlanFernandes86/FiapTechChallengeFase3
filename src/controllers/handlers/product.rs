use actix_web::{web, get, put, HttpResponse, Responder};
use crate::controllers::models::product::{ ProductDTO, ProductQuery };

#[get("")]
pub async fn get_product_by_category_id(category: web::Query<ProductQuery>) -> impl Responder {
    HttpResponse::Ok().json("get_product_by_category_id")
}

#[put("")]
pub async fn put_product(product_dto: web::Json<ProductDTO>) -> impl Responder {
    HttpResponse::Ok().json("put_product")
}