use actix_web::web;
use crate::controllers::handlers::user::{ get_user_by_cpf, set_user };

pub fn init(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1/user")
            .service(get_user_by_cpf)
            .service(set_user)
    );
}