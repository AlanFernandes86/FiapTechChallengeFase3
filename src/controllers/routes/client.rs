use actix_web::web;
use crate::controllers::handlers::client::{ get_client_by_cpf, set_client };

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/client")
            .service(get_client_by_cpf)
            .service(set_client)
    );
}