use actix_web::{web, get, post, HttpResponse, Responder};
use sqlx::MssqlPool;
use crate::application::client::get_client_by_cpf::GetClientByCpfUseCase;
use crate::infrastructure::repository::client_repository::SqlClientRepository;

#[get("/{cpf}")]
pub async fn get_client_by_cpf(path: web::Path<String>) -> impl Responder {
    let cpf = path.into_inner();
    // Get the database connection pool
    // let pool = MssqlPool::connect("mssql://sa:yourStrong(!)Password@localhost:1433/master")
    //     .await
    //     .expect("Error connecting to the database");
    
    // Create a new instance of the repository
    let repo = SqlClientRepository::new();

    // Create a new instance of the use case
    let use_case = GetClientByCpfUseCase::new(Box::new(repo));

    // Call the use case method to get the client by CPF
    let client = use_case.handle(cpf).await;

    // Handle the result and return the appropriate response
    match client {
        Some(client) => HttpResponse::Ok().json(client),
        None => HttpResponse::NotFound().body("Client not found")
    }
}

#[post("")]
pub async fn set_client() -> impl Responder {
    HttpResponse::Ok().body("Get client by cpf")
}