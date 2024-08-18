use actix_web::{App,  HttpServer};

mod controllers;
mod application;
mod infrastructure;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(controllers::routes::client::init)
            .configure(controllers::routes::product::init)
            .configure(controllers::routes::product_category::init)
            .configure(controllers::routes::order::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
