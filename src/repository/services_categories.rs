pub mod services_categories {
    use crate::models::services_categories::ServiceCategory;
    use crate::repository::schema::services_categories::dsl::*;
    use crate::repository::database::Database;
    
    use std::fmt::Error;
    use chrono::prelude::*;
    use diesel::prelude::*;

    impl Database {
        pub fn get_services_categories(&self) -> Vec<ServiceCategory> {
            services_categories
            .load::<ServiceCategory>(&mut self.pool.get().unwrap())
            .expect("Error loading all services categories")
        }
        
        pub fn create_services_categories(&self, service_category: ServiceCategory) -> Result<ServiceCategory, Error> {
            let service_category = ServiceCategory {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                ..service_category
            };
            diesel::insert_into(services_categories)
            .values(&service_category)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new services categories");
            Ok(service_category)
        }
        
        pub fn get_services_categories_by_id(&self, services_categories_id: &str) -> Option<ServiceCategory> {
            let service_category = services_categories
            .find(services_categories_id)
            .get_result::<ServiceCategory>(&mut self.pool.get().unwrap())
            .expect("Error loading services categories by id");
            Some(service_category)
        }
        
        pub fn delete_services_categories_by_id(&self, services_categories_id: &str) -> Option<usize> {
            let count = diesel::delete(services_categories.find(services_categories_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting services categories by id");
            Some(count)
        }
        
        
        pub fn update_services_categories_by_id(&self, services_categories_id: &str, mut service_category: ServiceCategory) -> Option<ServiceCategory> {
            service_category.updated_at = Some(Utc::now().naive_utc());
            let service_category = diesel::update(services_categories.find(services_categories_id))
            .set(&service_category)
            .get_result::<ServiceCategory>(&mut self.pool.get().unwrap())
            .expect("Error updating service categories by id");
            Some(service_category)
        }
    }
}

