use std::sync::Arc;

use actix_web::{web, post, HttpResponse, Responder};

use crate::{
    application::{order::update_order_status::UpdateOrderStatusUseCase, payment::{mercado_pago_notification::MercadoPagoNotificationUseCase, start_payment::StartPaymentUseCase}}, 
    controllers::models::payment::{MercadoPagoNotificationDTO, StartPaymentDTO}, 
    domain::enums::payment_method::EnPaymentMethod, 
    infrastructure::{messaging::kafka::kafka_producer::KafkaProducer, 
    repository::dynamo_db::{common::dynamo_db_factory::DynamoDbFactory, order_repository::DynamoDbOrderRepository, payment_repository::DynamoDbPaymentRepository}, 
    service::mercado_pago::mercado_pago_service::MercadoPagoService}
};

#[post("/start")]
pub async fn start_payment(start_payment: web::Json<StartPaymentDTO>) -> impl Responder {
    let start_payment_dto: StartPaymentDTO = start_payment.into_inner();
    let payment_method = match EnPaymentMethod::from_id(start_payment_dto.payment_method_id) {
        Ok(method) => method,
        Err(_) => return HttpResponse::BadRequest().json("Método de pagamento inválido."),
    };
    
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let order_repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let service = match payment_method {
                EnPaymentMethod::MercadoPago => MercadoPagoService::new(reqwest::Client::new()),
                _ => {
                    return HttpResponse::BadRequest().json("Método de pagamento não implementado.");
                }
            };
            let start_payment_use_case = StartPaymentUseCase::new(Box::new(order_repo), Box::new(service));
            
            let order_id = start_payment_dto.order_id;
            let result = start_payment_use_case.handle(start_payment_dto).await;
        
            match result {
                Ok(option) => 
                    match option {
                        Some(start_payment_response) => HttpResponse::Created().json(start_payment_response),
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
pub async fn mercado_pago_notification(mercado_pago_notification: web::Json<MercadoPagoNotificationDTO>) -> impl Responder {
    let mercado_pago_notification_dto = mercado_pago_notification.into_inner();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let order_repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let payment_repo = DynamoDbPaymentRepository::new(dynamo_db_client.clone());
            let message_publisher = KafkaProducer::new().expect("Failed to create Kafka producer");
            let mercado_pago_service = MercadoPagoService::new(reqwest::Client::new());
            let updated_order_status_use_case = UpdateOrderStatusUseCase::new(Arc::new(order_repo), Arc::new(message_publisher));
            let mercado_pago_notification_use_case = MercadoPagoNotificationUseCase::new(Box::new(payment_repo), Box::new(updated_order_status_use_case), Box::new(mercado_pago_service));
 
            let result = mercado_pago_notification_use_case.handle(&mercado_pago_notification_dto).await;
        
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(e) => {
                    println!("{:?}", e);
                    HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
                }
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error.")
    }
}