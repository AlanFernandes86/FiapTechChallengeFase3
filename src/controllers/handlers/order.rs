use actix_web::{web, get, put, post, patch, HttpResponse, Responder};
use crate::controllers::models::order::{ GetOrdersQuery, UpdateOrderStatusDTO, CreateOrderDTO };

#[get("/{orderId}")]
pub async fn get_order_by_id(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json("get_order_by_id")
}

#[get("")]
pub async fn get_orders(get_orders_query: web::Query<GetOrdersQuery>) -> impl Responder {
    HttpResponse::Ok().json("get_orders")
}

#[post("")]
pub async fn create_order(create_order_dto: web::Json<CreateOrderDTO>) -> impl Responder {
    HttpResponse::Ok().json("create_order")
}

#[patch("")]
pub async fn update_order_status(update_status_dto: web::Json<UpdateOrderStatusDTO>) -> impl Responder {
    HttpResponse::Ok().json("update_order_status")
}