use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::api::{users,services, services_categories, products_categories, products};

use super::auth_api::validator;


pub fn config(cfg: &mut web::ServiceConfig) {
    let bearer_middleware = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/api")
        //products_categories
        .service(products_categories::get_products_categories_by_id)
        .service(products_categories::get_products_categories)
        //products
        .service(products::get_product_by_id)
        .service(products::get_products)
        //users
        .service(users::create_users)
        //services_categories
        .service(services_categories::get_services_categories_by_id)
        .service(services_categories::get_services_categories)
        //services
        .service(services::get_services_by_id)
        .service(services::get_services)
        .service(users::auth)
    );

    cfg.service(
        web::scope("/protapi")
        .wrap(bearer_middleware)
        //products_categories
        .service(products_categories::create_products_categories)
        .service(products_categories::delete_products_categories_by_id)
        .service(products_categories::update_products_categories_by_id)
        //products
        .service(products::create_product)
        .service(products::delete_product_by_id)
        .service(products::update_product_by_id)
        //users
        .service(users::get_users_by_id)
        .service(users::get_users)
        .service(users::delete_users_by_id)
        .service(users::update_users_by_id)
        //services_categories
        .service(services_categories::create_services_categories)
        .service(services_categories::delete_services_categories_by_id)
        .service(services_categories::update_services_categories_by_id)
        //services
        .service(services::create_services)
        .service(services::delete_services_by_id)
        .service(services::update_services_by_id)
    );
}
