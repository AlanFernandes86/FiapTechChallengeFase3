use actix_web::{web, get, put, HttpResponse, Responder};
use crate::{
    application::product::{
        get_products_by_category::GetProductsByCategoryUseCase, 
        put_product::PutProductUseCase
    },
    controllers::models::product::{
            ProductDTO,
            ProductQuery
        },
        infrastructure::repository::{
            common::mssql_pool::SqlServerPool,
            product_repository::MssqlProductRepository
        }
};

#[get("")]
pub async fn get_product_by_category_id(category: web::Query<ProductQuery>) -> impl Responder {
    let category_id = category.into_inner().category_id;
    let arc_pool = SqlServerPool::get_instance().await;    
    match arc_pool {
        Ok(pool) => {
            let repo = MssqlProductRepository::new(pool.clone());
            let use_case = GetProductsByCategoryUseCase::new(Box::new(repo));
            let result = use_case.handle(category_id).await;
            match result {
                Ok(option) => {
                    match option {
                        Some(vec_product) => HttpResponse::Ok().json(vec_product),
                        None => HttpResponse::NotFound().body(format!("No products found with the given category id {category_id}"))
                    }
                },
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    } 
}

#[put("")]
pub async fn put_product(product_dto: web::Json<ProductDTO>) -> impl Responder {
    let product = product_dto.into_inner().into();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool)=> {
            let repo = MssqlProductRepository::new(pool.clone());
            let use_case = PutProductUseCase::new(Box::new(repo));
            let result = use_case.handle(product).await;
        
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}