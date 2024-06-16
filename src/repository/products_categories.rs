pub mod products_categories {
    use crate::models::products_categories::ProductCategory;
    use crate::models::attr::Att;
    use crate::repository::schema::products_categories::dsl::*;
    use crate::repository::schema::attr::dsl::*;
    use crate::repository::database::Database;    
    use std::io::Result;
    use chrono::prelude::*;
    use diesel::prelude::*;
    use std::fs::remove_file;
    impl Database {
        pub fn get_products_categories(&self) -> Vec<(ProductCategory, Vec<Att>)> {
            let conn = &mut self.pool.get().unwrap();
            let product_categories = products_categories
                .load::<ProductCategory>(conn)
                .expect("Error loading all products categories");

            product_categories
                .into_iter()
                .map(|product_category| {
                    let attrs = attr
                        .filter(category_id.eq(&product_category.id))
                        .load::<Att>(conn)
                        .expect("Error loading attributes");

                    (product_category, attrs)
                })
                .collect()
        }

        pub async fn create_products_categories(
            &self,
            image_path: String,
            product_category: ProductCategory,
            atts: Vec<Att>,
        ) -> Result<ProductCategory> {

            let product_category = ProductCategory {
                id: uuid::Uuid::new_v4().to_string(),
                imagepath: Some(image_path),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                ..product_category
            };
            
            let conn = &mut self.pool.get().unwrap();
            diesel::insert_into(products_categories)
                .values(&product_category)
                .execute(conn)
                .expect("Error creating new products categories");

            for mut att in atts {
                att.id = uuid::Uuid::new_v4().to_string();
                att.category_id = product_category.id.clone();
                att.created_at = Some(Utc::now().naive_utc());
                att.updated_at = Some(Utc::now().naive_utc());

                diesel::insert_into(attr)
                    .values(&att)
                    .execute(conn)
                    .expect("Error creating new attribute");
            }

            Ok(product_category)
        }

        pub async fn update_products_categories_by_id(
            &self,
            products_categories_id: &str,
            image_path: String,
            mut product_category: ProductCategory,
            atts: Vec<Att>,
        ) -> Option<(ProductCategory, Vec<Att>)> {
            let conn = &mut self.pool.get().unwrap();
            product_category.updated_at = Some(Utc::now().naive_utc());
            let dir: &str = "upload/";
            let delete_image_path = product_category.imagepath;
            match delete_image_path{
                Some(delete_image_path) => remove_file(dir.to_owned() + delete_image_path.as_str()).unwrap(),
                None => println!("Error deleting product image")
            }
            product_category.imagepath = Some(image_path);
            diesel::update(products_categories.find(products_categories_id))
                .set(&product_category)
                .get_result::<ProductCategory>(conn)
                .expect("Error updating product categories by id");

            diesel::delete(attr.filter(category_id.eq(products_categories_id)))
                .execute(conn)
                .expect("Error deleting attributes");
           
            for mut att in atts {
                att.id = uuid::Uuid::new_v4().to_string();
                att.category_id = product_category.id.clone();
                att.created_at = Some(Utc::now().naive_utc());
                att.updated_at = Some(Utc::now().naive_utc());

                diesel::insert_into(attr)
                    .values(&att)
                    .execute(conn)
                    .expect("Error creating new attribute");
            }

            let attrs = attr
                .filter(category_id.eq(&product_category.id))
                .load::<Att>(conn)
                .expect("Error loading attributes");

            Some((product_category, attrs))
        }

        pub fn get_products_categories_by_id(&self, products_categories_id: &str) -> Option<(ProductCategory, Vec<Att>)> {
            let conn = &mut self.pool.get().unwrap();
            let product_category = products_categories
                .find(products_categories_id)
                .get_result::<ProductCategory>(conn)
                .expect("Error loading products categories by id");

            let attrs = attr
                .filter(category_id.eq(products_categories_id))
                .load::<Att>(conn)
                .expect("Error loading attributes");

            Some((product_category, attrs))
        }

        pub fn delete_products_categories_by_id(&self, products_categories_id: &str) -> Option<usize> {
            let conn = &mut self.pool.get().unwrap();
            diesel::delete(attr.filter(category_id.eq(products_categories_id)))
                .execute(conn)
                .expect("Error deleting attributes");
            let dir: &str = "upload/";
            let products_category:ProductCategory = products_categories
                .find(products_categories_id)
                .get_result::<ProductCategory>(conn)
                .expect("Error loading products categories by id");
            let image_path = products_category.imagepath;
            match image_path{
                Some(image_path) => remove_file(dir.to_owned() + image_path.as_str()).unwrap(),
                None => println!("Error deleting product image")
            }
            let count = diesel::delete(products_categories.find(products_categories_id))
                .execute(conn)
                .expect("Error deleting products categories by id");
            
            Some(count)
        }

        
    }
}
