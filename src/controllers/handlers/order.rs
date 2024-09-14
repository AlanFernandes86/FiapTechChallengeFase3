use actix_web::{web, get, post, patch, HttpResponse, Responder};
use crate::{
    application::order::{get_order_by_id::GetOrderByIdUseCase, get_orders_by_status::GetOrdersByStatusUseCase},
    controllers::models::order::{ CreateOrderDTO, GetOrdersQuery, UpdateOrderStatusDTO },
    infrastructure::repository::{common::mssql_pool::SqlServerPool, order_repository::MssqlOrderRepository}};

#[get("/{orderId}")]
pub async fn get_order_by_id(path: web::Path<i32>) -> impl Responder {
    let order_id = path.into_inner();
    let arc_pool = SqlServerPool::get_instance().await;    
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlOrderRepository::new(pool.clone());
            let use_case = GetOrderByIdUseCase::new(Box::new(repo));
            let result = use_case.handle(order_id).await;
            match result {
                Ok(option) => {
                    match option {
                        Some(order) => HttpResponse::Ok().json(order),
                        None => HttpResponse::BadRequest().body(format!("No order found with the given id {order_id}"))
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[get("")]
pub async fn get_orders(get_orders_query: web::Query<GetOrdersQuery>) -> impl Responder {
    let order_status_id = get_orders_query.into_inner().order_status_id;
    let arc_pool = SqlServerPool::get_instance().await;    
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlOrderRepository::new(pool.clone());
            let use_case = GetOrdersByStatusUseCase::new(Box::new(repo));
            let result = use_case.handle(order_status_id).await;
            match result {
                Ok(option) => {
                    match option {
                        Some(order_list) => HttpResponse::Ok().json(order_list),
                        None => HttpResponse::BadRequest().body(format!("No order found with the given status_id {order_status_id}"))
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[post("")]
pub async fn create_order(create_order_dto: web::Json<CreateOrderDTO>) -> impl Responder {
    HttpResponse::Ok().json("create_order")
}

#[patch("")]
pub async fn update_order_status(update_status_dto: web::Json<UpdateOrderStatusDTO>) -> impl Responder {
    HttpResponse::Ok().json("update_order_status")
}