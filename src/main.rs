use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpResponse, HttpServer};
use infrastructure::repository::memory_cache::load_products::LoadProducts;
mod controllers;
mod application;
mod infrastructure;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting FiapTechChallenge Fase 2!"); 

    let is_local = env::var("ENVIRONMENT").unwrap_or_else(|_| "local".to_string()) == "local";
    if is_local {
        dotenv().ok();
    }

    println!("Loading products...");
    let loader = LoadProducts::new();
    let result = loader.load_products().await;
    match result {
        Ok(_) => println!("Products loaded!"),
        Err(e) => println!("Error loading products: {}", e),        
    }

    HttpServer::new(|| {
        App::new()
            .configure(controllers::routes::user::init)
            .configure(controllers::routes::product::init)
            .configure(controllers::routes::product_category::init)
            .configure(controllers::routes::order::init)
            .configure(controllers::routes::payment::init)
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello FiapTechChallenge Fase 2!") }))
    })
    .bind(("0.0.0.0", 8080))?
    .bind(("0.0.0.0", 443))?
    .run()
    .await
}
