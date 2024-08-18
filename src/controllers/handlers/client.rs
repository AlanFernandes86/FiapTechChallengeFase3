use actix_web::{web, get, post, HttpResponse, Responder};
use crate::application::client::get_client_by_cpf::GetClientByCpfUseCase;
use crate::application::client::set_client::SetClientUseCase;
use crate::infrastructure::repository::client_repository::SqlClientRepository;
use crate::infrastructure::repository::pool::mssql_pool;
use crate::controllers::models::client::ClientDTO;

#[get("/{cpf}")]
pub async fn get_client_by_cpf(path: web::Path<String>) -> impl Responder {
    // Extrai o CPF do path
    let cpf = path.into_inner();
    
    // Cria database pool usando o metodo get_mssql_connection
    let pool = mssql_pool::get_mssql_connection().await;    
    // Lida com o resultado e retorna a resposta
    match pool {
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
        _ => ()
    }    
    // Cria nova instancia do repositorio
    let repo = SqlClientRepository::new(pool.unwrap());
    // Cria nova instancia do use case
    let use_case = GetClientByCpfUseCase::new(Box::new(repo));
    // Chama o use case
    let client = use_case.handle(cpf).await;

    // Lida com o resultado e retorna a resposta    
    match client {
        Ok(client) => {
            match client {
                Some(client) => HttpResponse::Ok().json(client),
                None => HttpResponse::NotFound().body("Client not found")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Internal server error")
    }
}

#[post("")]
pub async fn set_client(client_dto: web::Json<ClientDTO>) -> impl Responder {
    let client = client_dto.into_inner().into();
    let pool = mssql_pool::get_mssql_connection().await;
    match pool {
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
        _ => ()
    }
    let repo = SqlClientRepository::new(pool.unwrap());
    let use_case = SetClientUseCase::new(Box::new(repo));
    let client = use_case.handle(client).await;
  
    match client {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}