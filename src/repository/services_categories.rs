pub mod services_categories {
    use crate::models::services_categories::ServiceCategory;
    use crate::repository::schema::services_categories::dsl::*;
    use crate::repository::database::Database;

    use chrono::prelude::*;
    use diesel::prelude::*;
    use std::io::Result;
    use std::fs::remove_file;

    impl Database {
        pub fn get_services_categories(&self) -> Vec<ServiceCategory> {
            services_categories
            .load::<ServiceCategory>(&mut self.pool.get().unwrap())
            .expect("Error loading all services categories")
        }
        
        pub async fn create_services_categories(
                &self, 
                image_path: String,
                service_category: ServiceCategory
            ) -> Result<ServiceCategory> {

            let service_category = ServiceCategory {
                id: uuid::Uuid::new_v4().to_string(),
                imagepath: Some(image_path),
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
            let conn = &mut self.pool.get().unwrap();
            let count = diesel::delete(services_categories.find(services_categories_id))
            .execute(conn)
            .expect("Error deleting services categories by id");
            
            let dir: &str = "./";
            let services_category:ServiceCategory = services_categories
                .find(services_categories_id)
                .get_result::<ServiceCategory>(conn)
                .expect("Error loading products categories by id");
            let image_path = services_category.imagepath;
            match image_path{
                Some(image_path) => remove_file(dir.to_owned() + image_path.as_str()).unwrap(),
                None => println!("Error deleting service image")
            }
            Some(count)
        }
        
        
        pub async fn update_services_categories_by_id(&self, services_categories_id: &str, image_path: String, mut service_category: ServiceCategory) -> Option<ServiceCategory> {
            let conn = &mut self.pool.get().unwrap();
            service_category.updated_at = Some(Utc::now().naive_utc());
            
            let old_service_category = services_categories
                .find(services_categories_id)
                .get_result::<ServiceCategory>(&mut self.pool.get().unwrap())
                .expect("Error loading services categories by id");
            let old_image_path;     
            if image_path == "" {
                old_image_path = old_service_category.imagepath;
                service_category.imagepath = old_image_path;
            } else {
                service_category.imagepath = Some(image_path);
                let dir: &str = "./";
                let delete_image_path = old_service_category.imagepath.clone();
                match delete_image_path{
                    Some(delete_image_path) => remove_file(dir.to_owned() + delete_image_path.as_str()).unwrap(),
                    None => println!("Error deleting service image")
                }
            }
            let service_category = diesel::update(services_categories.find(services_categories_id))
                .set(&service_category)
                .get_result::<ServiceCategory>(conn)
                .expect("Error updating service categories by id");
            Some(service_category)
        }
    }
}

