use actix_web::{web, get, put, delete, HttpResponse, Responder};
use crate::application::product_category::get_product_categories::GetProductCategoriesUseCase;
use crate::controllers::models::product_category::ProductCategoryDTO;
use crate::infrastructure::repository::common::mssql_pool::SqlServerPool;
use crate::infrastructure::repository::product_category_repository::MssqlProductCategoryRepository;

#[get("")]
pub async fn get_product_categories() -> impl Responder {
    let arc_pool = SqlServerPool::get_instance().await;    
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlProductCategoryRepository::new(pool.clone());
            let use_case = GetProductCategoriesUseCase::new(Box::new(repo));
            let result = use_case.handle().await;
            match result {
                Ok(option) => {
                    match option {
                        Some(vec_product_category) => HttpResponse::Ok().json(vec_product_category),
                        None => HttpResponse::NotFound().body("Product categories not found")
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }    
}

#[put("")]
pub async fn put_product_category(put_product_category_dto: web::Json<ProductCategoryDTO>) -> impl Responder {
    HttpResponse::Ok().json("put_product_category")
}

#[delete("/{id}")]
pub async fn delete_product_category(path: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json("put_product_category")
}