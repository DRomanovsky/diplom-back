use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::users::User, repository::database::Database};


#[post("/users")]
pub async fn create_users(db: web::Data<Database>, new_users: web::Json<User>) -> HttpResponse {
    let users = db.create_users(new_users.into_inner());
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_users_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let users = db.get_users_by_id(&id);

    match users {
        Some(users) => HttpResponse::Ok().json(users),
        None => HttpResponse::NotFound().body("Users not found"),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[delete("/users/{id}")]
pub async fn delete_users_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let users = db.delete_users_by_id(&id);
    match users {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/users/{id}")]
pub async fn update_users_by_id(db: web::Data<Database>, id: web::Path<String>, updated_users: web::Json<User>) -> HttpResponse {
    let users = db.update_users_by_id(&id, updated_users.into_inner());
    match users {
        Some(users) => HttpResponse::Ok().json(users),
        None => HttpResponse::NotFound().body("Users not found"),
    }
}

