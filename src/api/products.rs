use actix_web::{web, get, post, delete, put, HttpResponse};

use crate::{
    models::{
        products::Product,
        attr::Att,
        attr_value::AttVal
    },
    repository::database::Database
};

#[post("/products")]
pub async fn create_products(db: web::Data<Database>, new_product: web::Json<Product>, new_attrs: web::Json<Vec<(Att, Vec<AttVal>)>>) -> HttpResponse {
    let product = db.create_product(new_product.into_inner(), new_attrs.into_inner());
    match product {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
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