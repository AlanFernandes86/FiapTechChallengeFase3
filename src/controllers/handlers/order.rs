use std::sync::Arc;
use actix_web::{delete, get, patch, post, put, web, HttpResponse, Responder};
use crate::{
    application::{order::{
        create_order::CreateOrderUseCase,
        get_order_by_id::GetOrderByIdUseCase,
        get_orders_by_status::GetOrdersByStatusUseCase,
        update_order_status::UpdateOrderStatusUseCase
    }, order_product::{delete_order_product::DeleteOrderProductUseCase, put_order_product::PutOrderProductUseCase}}, 
    controllers::models::{order::{ 
        CreateOrderDTO,
        GetOrdersQuery,
        UpdateOrderStatusDTO 
    }, order_product::PutOrderProductDTO}, 
    domain::errors::invalid_order_status_update_error::InvalidOrderStatusUpdateError,
    infrastructure::{messaging::kafka::kafka_producer::KafkaProducer, repository::dynamo_db::{
        user_repository::DynamoDbUserRepository, common::dynamo_db_factory::DynamoDbFactory, order_product_repository::DynamoDbOrderProductRepository, order_repository::DynamoDbOrderRepository, product_repository::DynamoDbProductRepository},
    }};

#[get("/{orderId}")]
pub async fn get_order_by_id(path: web::Path<i32>) -> impl Responder {
    let order_id = path.into_inner();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let use_case = GetOrderByIdUseCase::new(Box::new(repo));
            let result = use_case.handle(order_id).await;
            match result {
                Ok(option) => {
                    match option {
                        Some(order) => HttpResponse::Ok().json(order),
                        None => HttpResponse::NotFound().body(format!("No order found with the given id {order_id}"))
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[get("")]
pub async fn get_orders(get_orders_query: web::Query<GetOrdersQuery>) -> impl Responder {
    let order_status_id = get_orders_query.into_inner().order_status_id;
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let use_case = GetOrdersByStatusUseCase::new(Box::new(repo));
            let result = use_case.handle(order_status_id).await;
            match result {
                Ok(option) => {
                    match option {
                        Some(order_list) => HttpResponse::Ok().json(order_list),
                        None => HttpResponse::NotFound().body(format!("No order found with the given status_id {order_status_id}"))
                    }
                },
                Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[post("")]
pub async fn create_order(create_order_dto: web::Json<CreateOrderDTO>) -> impl Responder {
    let order = create_order_dto.into_inner().into();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let order_repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let user_repo = DynamoDbUserRepository::new(dynamo_db_client.clone());
            let product_repo = DynamoDbProductRepository::new(dynamo_db_client.clone());
            let use_case = CreateOrderUseCase::new(Box::new(order_repo), Box::new(user_repo), Box::new(product_repo));
            let result = use_case.handle(order).await;
        
            match result {
                Ok(created_order_id) => HttpResponse::Created().json(serde_json::json!({ "created_order_id": created_order_id })),
                Err(e) => HttpResponse::InternalServerError().body(format!("Internal server error: {e}"))
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[patch("")]
pub async fn update_order_status(update_status_dto: web::Json<UpdateOrderStatusDTO>) -> impl Responder {
    let update_status_dto = update_status_dto.into_inner();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let repo = DynamoDbOrderRepository::new(dynamo_db_client.clone());
            let message_publisher = KafkaProducer::new().expect("Failed to create Kafka producer");
            let use_case = UpdateOrderStatusUseCase::new(Arc::new(repo), Arc::new(message_publisher));
            let result = use_case.handle(update_status_dto.order_id, update_status_dto.order_status_id).await;
            match result {
                Ok(updated_order) => 
                    match updated_order {
                        Some(_) => HttpResponse::Ok().finish(),
                        None => HttpResponse::BadRequest().body(
                            format!(
                            "No order found with the given id [{0}].",
                            update_status_dto.order_id
                            )
                        )
                    },
                Err(e) => {
                    if let Some(_invalid_status) = e.downcast_ref::<InvalidOrderStatusUpdateError>() {
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

#[delete("/{order_id}/product/{order_product_id}")]
pub async fn delete_order_product(path: web::Path<(i32, i32)>) -> impl Responder {
    let (order_id, order_product_id) = path.into_inner();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let repo = DynamoDbOrderProductRepository::new(dynamo_db_client.clone());
            let use_case = DeleteOrderProductUseCase::new(Box::new(repo));
            let result = use_case.handle(order_id, order_product_id).await;
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Database connection error")
    }
}

#[put("/product")]
pub async fn put_order_product(order_product_dto: web::Json<PutOrderProductDTO>) -> impl Responder {
    let order_product = order_product_dto.into_inner().into();
    let get_instance_result = DynamoDbFactory::get_instance().await;
    match get_instance_result {
        Ok(dynamo_db_client)=> {
            let repo = DynamoDbOrderProductRepository::new(dynamo_db_client.clone());
            let use_case = PutOrderProductUseCase::new(Box::new(repo));
            let result = use_case.handle(order_product).await;
            match result {
                Ok(_) => HttpResponse::Ok().finish(),
                Err(_) => HttpResponse::InternalServerError().body("Internal server error")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Database connection error")
    }
}