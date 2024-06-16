
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Verifier;
use jwt::SignWithKey;
use crate::{api::auth_api::TokenClaims, repository::schema::users::dsl::*};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::{models::users::User, repository::database::Database};
use diesel::prelude::*;

use serde::{Serialize,Deserialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct AuthUser {
    #[serde(default)]
    pub id: String,
    pub email: String,
    pub pass: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[post("/users")]
pub async fn create_users(db: web::Data<Database>, new_users: web::Json<User>) -> HttpResponse {
    let user = db.create_users(new_users.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[post("/auth")]
pub async fn auth(db: web::Data<Database>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes()
    ).unwrap();
    let username = credentials.user_id();
    let password = credentials.password();
    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(password) => {
            let passs = users.select(pass).filter(email.eq(username)).first::<String>(&mut db.pool.get().unwrap());
            match passs {
                Ok(passs) => {
                    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_password(password)
                        .with_hash(passs)
                        .with_secret_key(hash_secret)
                        .verify();

                    let iddd = users.select(id).filter(email.eq(username)).first::<String>(&mut db.pool.get().unwrap());
                    if is_valid.expect("Sdedtesgrs") {
                        let claims = TokenClaims {id: iddd.expect("cool")};
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        return HttpResponse::Ok().json(token_str);
                    } else {
                        return HttpResponse::Unauthorized().json("Incorrect username or password");
                    }
                }
                Err(error) => return HttpResponse::InternalServerError().json(format!("{:?} help", error)),
            }   
        }
    }
}

#[get("/users/{id}")]
pub async fn get_users_by_id(db: web::Data<Database>, idd: web::Path<String>) -> HttpResponse {
    let user = db.get_users_by_id(&idd);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Users not found"),
    }
}

#[get("/users")]
pub async fn get_users(db: web::Data<Database>) -> HttpResponse {
    let user = db.get_users();
    HttpResponse::Ok().json(user)
}

#[delete("/users/{id}")]
pub async fn delete_users_by_id(db: web::Data<Database>, idd: web::Path<String>) -> HttpResponse {
    let user = db.delete_users_by_id(&idd);
    match user {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[put("/users/{id}")]
pub async fn update_users_by_id(db: web::Data<Database>, idd: web::Path<String>, updated_users: web::Json<User>) -> HttpResponse {
    let user = db.update_users_by_id(&idd, updated_users.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("Users not found"),
    }
}

