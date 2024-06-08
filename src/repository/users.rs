pub mod users {
    use crate::models::users::User;
    use crate::repository::schema::users::dsl::*;
    use crate::repository::database::Database;

    use core::hash;
    use std::fmt::Error;
    use chrono::prelude::*;
    use diesel::{prelude::*, query_builder::bind_collector};

    use crate::{api::auth_api::TokenClaims};
    use actix_web_httpauth::extractors::basic::BasicAuth;
    use argonautica::{Hasher, Verifier};
    use chrono::NaiveDateTime;
    use hmac::{Hmac, Mac};
    use jwt::SignWithKey;
    use serde::{Deserialize, Serialize};
    use sha2::Sha256;

    impl Database{
        pub fn get_users(&self) -> Vec<User> {
            users
            .load::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading all users")
        }
    
        pub fn create_users(&self, user: User) -> Result<User, Error> {
            
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
            let mut hasher = Hasher::default();
            let poggers = hasher
                .with_password(&user.pass)
                .with_secret_key(hash_secret)
                .hash()
                .unwrap();
            let user = User {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                pass: poggers,
                ..user
            };
            diesel::insert_into(users)
            .values(&user)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new users");
            Ok(user)
        }
    
        pub fn get_users_by_id(&self, users_id: &str) -> Option<User> {
            let user = users
            .find(users_id)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error loading users by id");
            Some(user)
        }
    
        pub fn delete_users_by_id(&self, users_id: &str) -> Option<usize> {
            let count = diesel::delete(users.find(users_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting users by id");
            Some(count)
        }
    
    
        pub fn update_users_by_id(&self, users_id: &str, mut user: User) -> Option<User> {
            user.updated_at = Some(Utc::now().naive_utc());
            let user = diesel::update(users.find(users_id))
            .set(&user)
            .get_result::<User>(&mut self.pool.get().unwrap())
            .expect("Error updating users by id");
            Some(user)
        }
    }
    
}