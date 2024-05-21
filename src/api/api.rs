use actix_web::web;
use crate::api::{users, services, services_categories, products_categories, products};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        //products_categories
        .service(products_categories::create_products_categories)
        .service(products_categories::get_products_categories_by_id)
        .service(products_categories::get_products_categories)
        .service(products_categories::delete_products_categories_by_id)
        .service(products_categories::update_products_categories_by_id)
        //products
        .service(products::create_products)
        .service(products::get_product_by_id)
        .service(products::get_products)
        .service(products::delete_product_by_id)
        .service(products::update_product_by_id)
        //users
        .service(users::create_users)
        .service(users::get_users_by_id)
        .service(users::get_users)
        .service(users::delete_users_by_id)
        .service(users::update_users_by_id)
        //services_categories
        .service(services_categories::create_services_categories)
        .service(services_categories::get_services_categories_by_id)
        .service(services_categories::get_services_categories)
        .service(services_categories::delete_services_categories_by_id)
        .service(services_categories::update_services_categories_by_id)
        //services
        .service(services::create_services)
        .service(services::get_services_by_id)
        .service(services::get_services)
        .service(services::delete_services_by_id)
        .service(services::update_services_by_id)
    );
}
