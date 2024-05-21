use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::services::Service, repository::database::Database};


#[post("/services")]
pub async fn create_services(db: web::Data<Database>, new_services: web::Json<Service>) -> HttpResponse {
    let services = db.create_services(new_services.into_inner());
    match services {
        Ok(services) => HttpResponse::Ok().json(services),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/services/{id}")]
pub async fn get_services_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let services = db.get_services_by_id(&id);

    match services {
        Some(services) => HttpResponse::Ok().json(services),
        None => HttpResponse::NotFound().body("Service not found"),
    }
}

#[get("/services")]
pub async fn get_services(db: web::Data<Database>) -> HttpResponse {
    let services = db.get_services();
    HttpResponse::Ok().json(services)
}

#[delete("/services/{id}")]
pub async fn delete_services_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let services = db.delete_services_by_id(&id);
    match services {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Service not found"),
    }
}

#[put("/services/{id}")]
pub async fn update_services_by_id(db: web::Data<Database>, id: web::Path<String>, updated_services: web::Json<Service>) -> HttpResponse {
    let services = db.update_services_by_id(&id, updated_services.into_inner());
    match services {
        Some(services) => HttpResponse::Ok().json(services),
        None => HttpResponse::NotFound().body("Service not found"),
    }
}

