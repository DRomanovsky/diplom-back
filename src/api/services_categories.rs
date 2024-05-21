use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::services_categories::ServiceCategory, repository::database::Database};


#[post("/services_categories")]
pub async fn create_services_categories(db: web::Data<Database>, new_services_categories: web::Json<ServiceCategory>) -> HttpResponse {
    let services_categories = db.create_services_categories(new_services_categories.into_inner());
    match services_categories {
        Ok(services_categories) => HttpResponse::Ok().json(services_categories),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/services_categories/{id}")]
pub async fn get_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let poggers = db.get_services_categories_by_id(&id);
    match poggers {
        Some(poggers) => HttpResponse::Ok().json(poggers),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}

#[get("/services_categories")]
pub async fn get_services_categories(db: web::Data<Database>) -> HttpResponse {
    let services_categories = db.get_services_categories();
    HttpResponse::Ok().json(services_categories)
}

#[delete("/services_categories/{id}")]
pub async fn delete_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let services_categories = db.delete_services_categories_by_id(&id);
    match services_categories {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}

#[put("/services_categories/{id}")]
pub async fn update_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>, updated_services_categories: web::Json<ServiceCategory>) -> HttpResponse {
    let services_categories = db.update_services_categories_by_id(&id, updated_services_categories.into_inner());
    match services_categories {
        Some(services_categories) => HttpResponse::Ok().json(services_categories),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}
