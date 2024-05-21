pub mod services{
    use crate::models::services::Service;
    use crate::repository::schema::services::dsl::*;
    use crate::repository::database::Database;

    use std::fmt::Error;
    use chrono::prelude::*;
    use diesel::prelude::*;

    impl Database {
        pub fn get_services(&self) -> Vec<Service> {
            services
            .load::<Service>(&mut self.pool.get().unwrap())
            .expect("Error loading all services")
        }
    
        pub fn create_services(&self, service: Service) -> Result<Service, Error> {
            let service = Service {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                ..service
            };
            diesel::insert_into(services)
            .values(&service)
            .execute(&mut self.pool.get().unwrap())
            .expect("Error creating new services");
            Ok(service)
        }
    
        pub fn get_services_by_id(&self, services_id: &str) -> Option<Service> {
            let service = services
            .find(services_id)
            .get_result::<Service>(&mut self.pool.get().unwrap())
            .expect("Error loading services by id");
            Some(service)
        }
    
        pub fn delete_services_by_id(&self, services_id: &str) -> Option<usize> {
            let count = diesel::delete(services.find(services_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting services by id");
            Some(count)
        }
    
    
        pub fn update_services_by_id(&self, services_id: &str, mut service: Service) -> Option<Service> {
            service.updated_at = Some(Utc::now().naive_utc());
            let service = diesel::update(services.find(services_id))
            .set(&service)
            .get_result::<Service>(&mut self.pool.get().unwrap())
            .expect("Error updating service by id");
            Some(service)
        }
    }    
}