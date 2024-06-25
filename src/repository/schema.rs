// @generated automatically by Diesel CLI.

diesel::table! {
    attr (id) {
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        category_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    attr_value (attr_id, product_id) {
        #[max_length = 255]
        attr_id -> Varchar,
        #[max_length = 255]
        product_id -> Varchar,
        #[max_length = 255]
        value -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        price -> Nullable<Varchar>,
        acc -> Nullable<Int4>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        image -> Varchar,
        status -> Bool,
        #[max_length = 255]
        category_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products_categories (id) {
        id -> Varchar,
        title -> Nullable<Varchar>,
        imagepath -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    services (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        title -> Varchar,
        #[max_length = 255]
        price -> Nullable<Varchar>,
        #[max_length = 255]
        category_id -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    services_categories (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        imagepath -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        pass -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(attr -> products_categories (category_id));
diesel::joinable!(attr_value -> attr (attr_id));
diesel::joinable!(attr_value -> products (product_id));
diesel::joinable!(products -> products_categories (category_id));
diesel::joinable!(services -> services_categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    attr,
    attr_value,
    products,
    products_categories,
    services,
    services_categories,
    users,
);
