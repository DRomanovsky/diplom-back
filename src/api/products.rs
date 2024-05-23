use actix_web::{web, get, post, delete, put, HttpResponse, ResponseError, http::StatusCode};
use thiserror::Error;
use std::error::Error;
use serde_json::Value;
use crate::{
    models::{
        products::Product,
        attr::Att,
        attr_value::AttVal
    },
    repository::database::Database
};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An error occurred: {0}")]
    CustomError(String),
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send + Sync + 'static>),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::CustomError(message) => HttpResponse::BadRequest().body(message.clone()),
            MyError::Other(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::CustomError(_) => StatusCode::BAD_REQUEST,
            MyError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// #[post("/products")]
// pub async fn create_products(db: web::Data<Database>, new_product: web::Json<Product>, new_attrs: web::Json<Vec<(String, AttVal)>>) -> HttpResponse {
//     let product = db.create_product(new_product.into_inner(), new_attrs.into_inner());
//     match product {
//         Ok(product) => HttpResponse::Ok().json(product),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }
// }
#[post("/products")]

pub async fn create_product(
    db: web::Data<Database>,
    body: web::Bytes,
) -> Result<HttpResponse, MyError> {
    let json_data: Value = serde_json::from_slice(&body).map_err(|err| {
        MyError::CustomError(format!("Invalid JSON: {}", err))
    })?;

    let new_product: Product = serde_json::from_value(json_data.get("new_products").cloned().ok_or_else(|| MyError::CustomError("Missing 'new_products' field".to_string()))?).map_err(|err| MyError::CustomError(format!("Invalid 'new_product': {}", err)))?;

    let new_products_attr: Vec<(String, AttVal)> = serde_json::from_value(json_data.get("new_products_attr").cloned().ok_or_else(|| MyError::CustomError("Missing 'new_products_attr' field".to_string()))?).map_err(|err| MyError::CustomError(format!("Invalid 'new_products_attr': {}", err)))?;

    let products = db
        .create_product(new_product, new_products_attr)
        .map_err(|e| MyError::Other(Box::new(e)))?;

    Ok(HttpResponse::Ok().json(products))
}

#[get("/products/{id}")]
pub async fn get_product_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let product = db.get_product_by_id(&id);
    match product {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}

#[get("/products")]
pub async fn get_products(db: web::Data<Database>) -> HttpResponse {
    let products = db.get_products();
    HttpResponse::Ok().json(products)
}

#[delete("/products/{id}")]
pub async fn delete_product_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let result = db.delete_product_by_id(&id);
    match result {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}

#[put("/products/{id}")]
pub async fn update_product_by_id(db: web::Data<Database>, id: web::Path<String>, updated_product: web::Json<Product>, updated_attrs: web::Json<Vec<(Att, Vec<AttVal>)>>) -> HttpResponse {
    let product = db.update_product_by_id(&id, updated_product.into_inner(), updated_attrs.into_inner());
    match product {
        Some(product) => HttpResponse::Ok().json(product),
        None => HttpResponse::NotFound().body("Product not found"),
    }
}