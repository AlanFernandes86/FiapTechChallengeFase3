use actix_web::{App,  HttpServer};

mod presentation;
mod application;
mod infrastructure;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(presentation::routes::client::init)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
