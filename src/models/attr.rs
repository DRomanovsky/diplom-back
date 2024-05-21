use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::attr)]
pub struct Att {
    #[serde(default)]
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub category_id: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}