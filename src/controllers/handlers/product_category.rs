use actix_web::{web, get, put, HttpResponse, Responder};
use crate::controllers::models::product_category::{ ProductCategoryDTO };

#[get("")]
pub async fn get_product_categories() -> impl Responder {
    HttpResponse::Ok().json("get_product_categories")
}

#[put("")]
pub async fn put_product_category(client_dto: web::Json<ProductCategoryDTO>) -> impl Responder {
    HttpResponse::Ok().json("put_product_category")
}