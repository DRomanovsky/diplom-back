pub mod products {
    use crate::models::products::Product;
    use crate::models::attr::Att;
    use crate::models::attr_value::AttVal;
    use crate::repository::schema::products::dsl::*;
    use crate::repository::schema::attr::dsl::*;
    use crate::repository::schema::attr_value::dsl::*;
    use crate::repository::database::Database;

    use std::fmt::Error;
    use chrono::prelude::*;
    use diesel::prelude::*;
    use std::fs::remove_file;
    impl Database {
        pub fn get_products(&self) -> Vec<(Product, Vec<(Att, AttVal)>)> {
            let conn = &mut self.pool.get().unwrap();
            let other_products = products
                .load::<Product>(conn)
                .expect("Error loading all products");
        
            let mut result = Vec::new();
        
            for product in other_products {
                let attrs = attr
                    .load::<Att>(conn)
                    .expect("Error loading attributes");
        
                let mut product_attrs = Vec::new();
        
                for att in attrs {
                    let attr_values = attr_value
                        .filter(attr_id.eq(&att.id))
                        .filter(product_id.eq(&product.id))
                        .load::<AttVal>(conn)
                        .expect("Error loading attribute values");
        
                    for att_value in attr_values {
                        product_attrs.push((att.clone(), att_value));
                    }
                }
        
                result.push((product, product_attrs));
            }
        
            result
        }

        pub fn create_product(
            &self,
            image_path: String,
            product: Product,
            attrs_with_values: Vec<(String, AttVal)>,
        ) -> Result<Product, Error> {
            let conn = &mut self.pool.get().unwrap();
            let product = Product {
                id: uuid::Uuid::new_v4().to_string(),
                image: image_path,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                ..product
            };

            diesel::insert_into(products)
                .values(&product)
                .execute(conn)
                .expect("Error creating new product");

            for (att, mut att_value) in attrs_with_values {
                att_value.attr_id = att;
                att_value.product_id = product.id.clone();
                att_value.created_at = Some(Utc::now().naive_utc());
                att_value.updated_at = Some(Utc::now().naive_utc());

                diesel::insert_into(attr_value)
                    .values(&att_value)
                    .execute(conn)
                    .expect("Error creating new attribute value");
            }

            Ok(product)
        }

        pub async fn update_product_by_id(
            &self,
            other_product_id: &str,
            image_path: String,
            mut product: Product,
            attrs_with_values: Vec<(String, AttVal)>,
        ) -> Option<Product> {
            let conn = &mut self.pool.get().unwrap();
            product.updated_at = Some(Utc::now().naive_utc());
            let old_product = products
                .find(other_product_id)
                .get_result::<Product>(conn)
                .expect("Error loading product by id");
            let old_image_path;     
            if image_path == "" {
                old_image_path = old_product.image;
                product.image = old_image_path;
            } else {
                product.image = image_path;
                let dir: &str = "./";
                let delete_image_path = old_product.image.clone();
                remove_file(dir.to_owned() + delete_image_path.as_str()).unwrap();
            }
            diesel::update(products.find(other_product_id))
                .set(&product)
                .get_result::<Product>(conn)
                .expect("Error updating product by id");

            diesel::delete(attr_value.filter(product_id.eq(other_product_id)))
                .execute(conn)
                .expect("Error deleting attribute values");

            for (att, mut att_value) in attrs_with_values {
                att_value.attr_id = att;
                att_value.product_id = product.id.clone();
                att_value.created_at = Some(Utc::now().naive_utc());
                att_value.updated_at = Some(Utc::now().naive_utc());
    
                diesel::insert_into(attr_value)
                    .values(&att_value)
                    .execute(conn)
                    .expect("Error creating new attribute value");
                }
            Some(product)
        }
        
        pub fn get_product_by_id(&self, productid: &str) -> Option<(Product, Vec<(Att, AttVal)>)> {
            let conn = &mut self.pool.get().unwrap();
            let product = products
                .find(productid)
                .first::<Product>(conn)
                .ok()?;
        
            let attrs_with_values = attr
                .load::<Att>(conn)
                .expect("Error loading attributes")
                .into_iter()
                .flat_map(|att| {
                    let attr_values = attr_value
                        .filter(attr_id.eq(&att.id))
                        .filter(product_id.eq(&product.id))
                        .load::<AttVal>(conn)
                        .expect("Error loading attribute values");
        
                    attr_values.into_iter().map(move |att_value| (att.clone(), att_value))
                })
                .collect::<Vec<_>>();
        
            Some((product, attrs_with_values))
        }
        pub fn delete_product_by_id(&self, other_product_id: &str) -> Option<usize> {
            let conn = &mut self.pool.get().unwrap();
            diesel::delete(attr_value.filter(product_id.eq(other_product_id)))
                .execute(conn)
                .expect("Error deleting attribute values");
            let dir: &str = "./";
            let product: Product = products
                .find(other_product_id)
                .get_result::<Product>(conn)
                .expect("Error loading products categories by id");
            let image_path = product.image;
            remove_file(dir.to_owned() + image_path.as_str()).unwrap();
            let count = diesel::delete(products.find(other_product_id))
                .execute(conn)
                .expect("Error deleting product by id");
            Some(count)
        }

    }
}