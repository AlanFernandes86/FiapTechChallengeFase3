use actix_web::{web, get, put, delete, HttpResponse, Responder};
use crate::application::product_category::delete_product_category::DeleteProductCategoryUseCase;
use crate::application::product_category::get_product_categories::GetProductCategoriesUseCase;
use crate::application::product_category::put_product_category::PutProductCategoryUseCase;
use crate::controllers::models::product_category::ProductCategoryDTO;
use crate::infrastructure::repository::mssql::common::mssql_pool::SqlServerPool;
use crate::infrastructure::repository::mssql::product_category_repository::MssqlProductCategoryRepository;
use serde_json::json;

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
    let product_category = put_product_category_dto.into_inner().into();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool)=> {
            let repo = MssqlProductCategoryRepository::new(pool.clone());
            let use_case = PutProductCategoryUseCase::new(Box::new(repo));
            let result = use_case.handle(product_category).await;
        
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }    
}

#[delete("/{id}")]
pub async fn delete_product_category(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool)=> {
            let repo = MssqlProductCategoryRepository::new(pool.clone());
            let use_case = DeleteProductCategoryUseCase::new(Box::new(repo));
            let result = use_case.handle(id).await;
        
            match result {
                Ok(rows_affected) => HttpResponse::Ok().body(json!({ "rows_affected": rows_affected }).to_string()),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}