use actix_web::{web, post, HttpResponse, Responder};

use crate::controllers::models::payment::SetPaymentDTO;

#[post("")]
pub async fn set_payment(set_payment_dto: web::Json<SetPaymentDTO>) -> impl Responder {
    HttpResponse::Ok().json("set_payment")
}