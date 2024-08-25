use actix_web::{web, get, post, HttpResponse, Responder};
use crate::application::client::get_client_by_cpf::GetClientByCpfUseCase;
use crate::application::client::set_client::SetClientUseCase;
use crate::infrastructure::repository::client_repository::MssqlClientRepository;
use crate::infrastructure::repository::common::mssql_pool::SqlServerPool;
use crate::controllers::models::client::ClientDTO;

#[get("/{cpf}")]
pub async fn get_client_by_cpf(path: web::Path<String>) -> impl Responder {
    // Extrai o CPF do path
    let cpf = path.into_inner();    
    // Cria database pool usando o metodo get_mssql_connection
    let arc_pool = SqlServerPool::get_instance().await;    
    // Lida com o resultado e retorna a resposta
    match arc_pool {
        Ok(pool) => {
            // Cria nova instancia do repositorio
            let repo = MssqlClientRepository::new(pool.clone());
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
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }    
}

#[post("")]
pub async fn set_client(client_dto: web::Json<ClientDTO>) -> impl Responder {
    let client = client_dto.into_inner().into();
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool)=> {
            let repo = MssqlClientRepository::new(pool.clone());
            let use_case = SetClientUseCase::new(Box::new(repo));
            let client = use_case.handle(client).await;
        
            match client {
                Ok(_) => HttpResponse::Created().finish(),
                Err(e) => HttpResponse::InternalServerError().body(e.to_string())
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}