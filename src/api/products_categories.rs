use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::{products_categories::ProductCategory, attr::Att}, repository::database::Database};


#[post("/products_categories")]
pub async fn create_products_categories(db: web::Data<Database>, new_products_categories: web::Json<ProductCategory>, new_products_categories_attr: web::Json<Vec<Att>>) -> HttpResponse {
    let products_categories = db.create_products_categories(new_products_categories.into_inner(), new_products_categories_attr.into_inner());
    match products_categories {
        Ok(products_categories) => HttpResponse::Ok().json(products_categories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/products_categories/{id}")]
pub async fn get_products_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let poggers = db.get_products_categories_by_id(&id);
    match poggers {
        Some(poggers) => HttpResponse::Ok().json(poggers),
        None => HttpResponse::NotFound().body("Product Category not found"),
    }
}

#[get("/products_categories")]
pub async fn get_products_categories(db: web::Data<Database>) -> HttpResponse {
    let products_categories = db.get_products_categories();
    HttpResponse::Ok().json(products_categories)
}

#[delete("/products_categories/{id}")]
pub async fn delete_products_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let products_categories = db.delete_products_categories_by_id(&id);
    match products_categories {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Product Category not found"),
    }
}

#[put("/products_categories/{id}")]
pub async fn update_products_categories_by_id(db: web::Data<Database>, id: web::Path<String>, updated_products_categories: web::Json<ProductCategory>, updated_products_categories_attr: web::Json<Vec<Att>>) -> HttpResponse {
    let products_categories = db.update_products_categories_by_id(&id, updated_products_categories.into_inner(), updated_products_categories_attr.into_inner());
    match products_categories {
        Some(products_categories) => HttpResponse::Ok().json(products_categories),
        None => HttpResponse::NotFound().body("Product Category not found"),
    }
}
