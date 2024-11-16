use actix_web::{web, get, post, HttpResponse, Responder};
use crate::application::user::get_user_by_cpf::GetUserByCpfUseCase;
use crate::application::user::set_user::SetUserUseCase;
use crate::domain::errors::user_already_exists_error::UserAlreadyExistsError;
use crate::infrastructure::repository::dynamo_db::user_repository::DynamoDbUserRepository;
use crate::infrastructure::repository::dynamo_db::common::dynamo_db_factory::DynamoDbFactory;
use crate::controllers::models::user::UserDTO;

#[get("/{cpf}")]
pub async fn get_user_by_cpf(path: web::Path<String>) -> impl Responder {
    // Extrai o CPF do path
    let cpf = path.into_inner();    
    let get_instance_result = DynamoDbFactory::get_instance().await;    
    // Lida com o resultado e retorna a resposta
    match get_instance_result {
        Ok(dynamodb_client) => {
            // Cria nova instancia do repositorio
            let repo = DynamoDbUserRepository::new(dynamodb_client.clone());
            // Cria nova instancia do use case
            let use_case = GetUserByCpfUseCase::new(Box::new(repo));
            // Chama o use case
            let result = use_case.handle(cpf).await;
            // Lida com o resultado e retorna a resposta    
            match result {
                Ok(option) => {
                    match option {
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
pub async fn set_user(user_dto: web::Json<UserDTO>) -> impl Responder {
    let client = user_dto.into_inner().into();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamodb_client)=> {
            let repo = DynamoDbUserRepository::new(dynamodb_client.clone());
            let use_case = SetUserUseCase::new(Box::new(repo));
            let result = use_case.handle(client).await;
        
            match result {
                Ok(_) => HttpResponse::Created().finish(),
                Err(e) => {
                    if let Some(_invalid_status) = e.downcast_ref::<UserAlreadyExistsError>() {
                        HttpResponse::BadRequest().body(e.to_string())
                    } else {
                        HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
                    }
                }
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}