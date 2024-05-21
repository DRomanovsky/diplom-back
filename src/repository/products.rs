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

    impl Database {
        pub fn get_products(&self) -> Vec<(Product, Vec<(Att, Vec<AttVal>)>)> {
            let conn = &mut self.pool.get().unwrap();
            let other_products = products
                .load::<Product>(conn)
                .expect("Error loading all products");
        
            other_products
                .into_iter()
                .map(|product| {
                    let attrs = attr
                        .load::<Att>(conn)
                        .expect("Error loading attributes");
        
                    let attrs_with_values = attrs.into_iter().map(|att| {
                        let attr_values = attr_value
                            .filter(attr_id.eq(&att.id))
                            .filter(product_id.eq(&product.id))
                            .load::<AttVal>(conn)
                            .expect("Error loading attribute values");
        
                        (att, attr_values)
                    }).collect::<Vec<_>>();
        
                    (product, attrs_with_values)
                })
                .collect()
        }

        pub fn create_product(
            &self,
            product: Product,
            attrs_with_values: Vec<(Att, Vec<AttVal>)>,
        ) -> Result<Product, Error> {
            let conn = &mut self.pool.get().unwrap();
            let product = Product {
                id: uuid::Uuid::new_v4().to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
                ..product
            };

            diesel::insert_into(products)
                .values(&product)
                .execute(conn)
                .expect("Error creating new product");

            for (mut att, attr_values) in attrs_with_values {
                att.id = uuid::Uuid::new_v4().to_string();
                att.created_at = Some(Utc::now().naive_utc());
                att.updated_at = Some(Utc::now().naive_utc());

                diesel::insert_into(attr)
                    .values(&attr)
                    .execute(conn)
                    .expect("Error creating new attribute");

                for mut att_value in attr_values {
                    att_value.attr_id = att.id.clone();
                    att_value.product_id = product.id.clone();
                    att_value.created_at = Some(Utc::now().naive_utc());
                    att_value.updated_at = Some(Utc::now().naive_utc());

                    diesel::insert_into(attr_value)
                        .values(&att_value)
                        .execute(conn)
                        .expect("Error creating new attribute value");
                }
            }

            Ok(product)
        }

        pub fn get_product_by_id(&self, other_product_id: &str) -> Option<(Product, Vec<(Att, Vec<AttVal>)>)> {
            let conn = &mut self.pool.get().unwrap();
            let product = products
                .find(other_product_id)
                .get_result::<Product>(conn)
                .expect("Error loading product by id");
        
            let attrs_with_values = attr
                .load::<Att>(conn)
                .expect("Error loading attributes")
                .into_iter()
                .map(|att| {
                    let attr_values = attr_value
                        .filter(attr_id.eq(&att.id))
                        .filter(product_id.eq(product_id))
                        .load::<AttVal>(conn)
                        .expect("Error loading attribute values");
        
                    (att, attr_values)
                })
                .collect::<Vec<_>>();
        
            Some((product, attrs_with_values))
        }

        pub fn delete_product_by_id(&self, other_product_id: &str) -> Option<usize> {
            let conn = &mut self.pool.get().unwrap();
            let count = diesel::delete(products.find(other_product_id))
                .execute(conn)
                .expect("Error deleting product by id");

            diesel::delete(attr_value.filter(product_id.eq(other_product_id)))
                .execute(conn)
                .expect("Error deleting attribute values");

            Some(count)
        }

        pub fn update_product_by_id(
            &self,
            other_product_id: &str,
            mut product: Product,
            attrs_with_values: Vec<(Att, Vec<AttVal>)>,
        ) -> Option<(Product, Vec<(Att, Vec<AttVal>)>)> {
            let conn = &mut self.pool.get().unwrap();
            product.updated_at = Some(Utc::now().naive_utc());
            diesel::update(products.find(other_product_id))
                .set(&product)
                .get_result::<Product>(conn)
                .expect("Error updating product by id");

            diesel::delete(attr_value.filter(product_id.eq(other_product_id)))
                .execute(conn)
                .expect("Error deleting attribute values");

            for (mut att, attr_values) in attrs_with_values {
                att.id = uuid::Uuid::new_v4().to_string();
                att.created_at = Some(Utc::now().naive_utc());
                att.updated_at = Some(Utc::now().naive_utc());

                diesel::insert_into(attr)
                    .values(&attr)
                    .execute(conn)
                    .expect("Error creating new attribute");

                for mut att_value in attr_values {
                    att_value.attr_id = att.id.clone();
                    att_value.product_id = product.id.clone();
                    att_value.created_at = Some(Utc::now().naive_utc());
                    att_value.updated_at = Some(Utc::now().naive_utc());

                    diesel::insert_into(attr_value)
                        .values(&attr_value)
                        .execute(conn)
                        .expect("Error creating new attribute value");
                }
            }

            let attrs = attr
                .load::<Att>(conn)
                .expect("Error loading attributes");

            let attrs_with_values = attrs.into_iter().map(|att| {
                let attr_values = attr_value
                    .filter(attr_id.eq(&att.id))
                    .filter(product_id.eq(&product.id))
                    .load::<AttVal>(conn)
                    .expect("Error loading attribute values");

                (att, attr_values)
            }).collect::<Vec<_>>();

            Some((product, attrs_with_values))
        }
    }
}