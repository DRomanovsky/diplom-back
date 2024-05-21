use actix_web::{web, get, post, delete, put, HttpResponse, ResponseError, http::StatusCode };
use thiserror::Error;
use std::error::Error;
use serde_json::Value;
use crate::{models::{products_categories::ProductCategory, attr::Att}, repository::database::Database};

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
#[post("/products_categories")]
async fn create_products_categories(
    db: web::Data<Database>,
    body: web::Bytes,
) -> Result<HttpResponse, MyError> {
    let json_data: Value = serde_json::from_slice(&body).map_err(|err| {
        MyError::CustomError(format!("Invalid JSON: {}", err))
    })?;

    let new_products_categories: ProductCategory = serde_json::from_value(json_data.get("new_products_categories").cloned().ok_or_else(|| MyError::CustomError("Missing 'new_products_categories' field".to_string()))?).map_err(|err| MyError::CustomError(format!("Invalid 'new_products_categories': {}", err)))?;

    let new_products_categories_attr: Vec<Att> = serde_json::from_value(json_data.get("new_products_categories_attr").cloned().ok_or_else(|| MyError::CustomError("Missing 'new_products_categories_attr' field".to_string()))?).map_err(|err| MyError::CustomError(format!("Invalid 'new_products_categories_attr': {}", err)))?;

    let products_categories = db
        .create_products_categories(new_products_categories, new_products_categories_attr)
        .map_err(|e| MyError::Other(Box::new(e)))?;

    Ok(HttpResponse::Ok().json(products_categories))
}

#[get("/products_categories/{id}")]
async fn get_products_categories_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, MyError> {
    let poggers = db.get_products_categories_by_id(&id);
    match poggers {
        Some(poggers) => Ok(HttpResponse::Ok().json(poggers)),
        None => Ok(HttpResponse::NotFound().body("Product Category not found")),
    }
}

#[get("/products_categories")]
async fn get_products_categories(db: web::Data<Database>) -> HttpResponse {
    let products_categories = db.get_products_categories();
    HttpResponse::Ok().json(products_categories)
}

#[delete("/products_categories/{id}")]
async fn delete_products_categories_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> HttpResponse{
    let products_categories = db.delete_products_categories_by_id(&id);
    match products_categories {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Product Category not found"),
    }
}

#[put("/products_categories/{id}")]
async fn update_products_categories_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_products_categories: web::Json<ProductCategory>,
    updated_products_categories_attr: web::Json<Vec<Att>>,
) -> Result<HttpResponse, MyError> {
    let products_categories = db.update_products_categories_by_id(
        &id,
        updated_products_categories.into_inner(),
        updated_products_categories_attr.into_inner(),
    );
    match products_categories {
        Some(products_categories) => Ok(HttpResponse::Ok().json(products_categories)),
        None => Ok(HttpResponse::NotFound().body("Product Category not found")),
    }
}