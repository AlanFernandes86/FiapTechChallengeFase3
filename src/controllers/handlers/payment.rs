use actix_web::{web, post, HttpResponse, Responder};
use serde_json::Value;

use crate::{
    application::{order::get_order_by_id::GetOrderByIdUseCase, payment::start_payment::StartPaymentUseCase}, 
    controllers::models::payment::StartPaymentDTO, domain::enums::payment_method::PaymentMethod, 
    infrastructure::{repository::{common::mssql_pool::SqlServerPool, order_product_repository::MssqlOrderProductRepository, order_repository::MssqlOrderRepository, payment_repository::MssqlPaymentRepository}, 
    service::mercado_pago::mercado_pago_service::MercadoPagoService}
};

#[post("/start")]
pub async fn start_payment(start_payment: web::Json<StartPaymentDTO>) -> impl Responder {
    let start_payment_dto: StartPaymentDTO = start_payment.into_inner();
    let payment_method = match PaymentMethod::from_id(start_payment_dto.payment_method_id) {
        Ok(method) => method,
        Err(_) => return HttpResponse::BadRequest().json("Método de pagamento inválido."),
    };
    
    let arc_pool = SqlServerPool::get_instance().await;
    match arc_pool {
        Ok(pool)=> {
            let payment_repo = MssqlPaymentRepository::new(pool.clone());
            let order_repo = MssqlOrderRepository::new(pool.clone());
            let order_product_repo = MssqlOrderProductRepository::new(pool.clone());
            let service = match payment_method {
                PaymentMethod::MercadoPago => MercadoPagoService::new(reqwest::Client::new()),
                _ => {
                    return HttpResponse::BadRequest().json("Método de pagamento não implementado.");
                }
            };
            let get_order_by_id_use_case = GetOrderByIdUseCase::new(Box::new(order_repo), Box::new(order_product_repo));
            let start_payment_use_case = StartPaymentUseCase::new(Box::new(payment_repo), Box::new(get_order_by_id_use_case), Box::new(service));
            
            let order_id = start_payment_dto.order_id;
            let result = start_payment_use_case.handle(start_payment_dto).await;
        
            match result {
                Ok(option) => 
                    match option {
                        Some(_) => HttpResponse::Created().finish(),
                        None => HttpResponse::BadRequest().body(
                            format!(
                            "No order_id found with the given id [{0}].",
                            order_id
                            )
                        )
                    },
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
                }
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error.")
    }
}

#[post("/mercado_pago_notification")]
pub async fn mercado_pago_notification(start_payment_dto: web::Json<Value>) -> impl Responder {
    println!("{:?}", start_payment_dto);
    HttpResponse::Ok().json("mercado_pago_notification")
}