use actix_web::{web, put, delete, HttpResponse, Responder};

use crate::{
    application::order_product::{delete_order_product::DeleteOrderProductUseCase, put_order_product::PutOrderProductUseCase},
    controllers::models::order_product::PutOrderProductDTO,
    infrastructure::repository::{
        common::mssql_pool::SqlServerPool,
        order_product_repository::MssqlOrderProductRepository
    }
};

#[put("")]
pub async fn put_order_product(order_product_dto: web::Json<PutOrderProductDTO>) -> impl Responder {
    let order_product = order_product_dto.into_inner().into();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlOrderProductRepository::new(pool.clone());
            let use_case = PutOrderProductUseCase::new(Box::new(repo));
            let result = use_case.handle(order_product).await;
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[delete("/{order_product_id}")]
pub async fn delete_order_product(path: web::Path<i32>) -> impl Responder {
    let order_product_id = path.into_inner();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlOrderProductRepository::new(pool.clone());
            let use_case = DeleteOrderProductUseCase::new(Box::new(repo));
            let result = use_case.handle(order_product_id).await;
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Database connection error")
    }
}