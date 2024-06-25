use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::products)]
pub struct Product {
    #[serde(default)]
    pub id: String,
    pub title: String,
    pub price: Option<String>,
    pub acc: Option<i32>,
    pub description: Option<String>,
    pub image: String,
    pub status: bool, 
    pub category_id: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}