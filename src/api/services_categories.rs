use actix_multipart::Multipart;
use actix_web::{delete, get, http:: StatusCode, post, put, web, HttpResponse, ResponseError };
use thiserror::Error;
use std::error::Error;
use serde_json::Value;
use futures_util::StreamExt;

use mime::{Mime, IMAGE_BMP, IMAGE_JPEG, IMAGE_PNG};
use uuid::Uuid;
use image::{ DynamicImage, imageops::FilterType };  
use futures_util:: TryStreamExt as _ ;
use tokio::fs;
use tokio::io::AsyncWriteExt as _;

use crate::{models::services_categories::ServiceCategory, repository::database::Database};

#[derive(Error, Debug)]
pub enum MyError {
    #[error("An error occurred: {0}")]
    CustomError(String),
    #[error(transparent)]
    Other(#[from] Box<dyn Error + Send + Sync + 'static>),
}
impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match self {
            MyError::CustomError(message) => HttpResponse::BadRequest().body(message.clone()),
            MyError::Other(ref err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            MyError::CustomError(_) => StatusCode::BAD_REQUEST,
            MyError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[post("/services_categories")]
pub async fn create_services_categories(
    db: web::Data<Database>, 
    mut payload: Multipart, 
) -> Result<HttpResponse, MyError> {
    let max_file_count: usize = 1;
    let mut current_count: usize = 0;
    let legal_filetypes:[Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_BMP];
    let filename = Uuid::new_v4();
    let dir: &str = "upload/";
    loop {
        if current_count == max_file_count { break; }
        if let Ok(Some(mut field)) = payload.try_next().await {
            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() { continue; }
            if !legal_filetypes.contains(&filetype.unwrap()) { continue; }
            let destination:String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }
            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                uploaded_img
                    .resize_exact(300, 300, FilterType::Gaussian)
                    .save(format!("{}{}.png", dir, filename.to_string())).unwrap();
                
            }).await.unwrap().await;

        } else { break; }
        current_count += 1;
    }
    // Json data handling
    let mut json_data = None;
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition(); 
        let field_name = content_disposition.get_name().ok_or(MyError::CustomError("Missing field name".to_string())).unwrap();
        if field_name == "data" {
            let mut data = Vec::new();
            while let Some(chunk) = field.try_next().await.unwrap() {
                data.extend_from_slice(&chunk);
            }
            json_data = Some(serde_json::from_slice::<Value>(&data).unwrap());
        }
    }
    let json_data = json_data.ok_or(MyError::CustomError("Missing 'data' field".to_string())).unwrap();

    let new_services_categories: ServiceCategory = serde_json::from_value(json_data.get("new_services_categories").cloned().ok_or_else(|| MyError::CustomError("Missing 'new_services_categories' field".to_string())).unwrap()).map_err(|err| MyError::CustomError(format!("Invalid 'new_services_categories': {}", err))).unwrap();
    let image_path = dir.to_owned() + &filename.to_string().to_owned() + ".png";
    
    let services_categories = db
        .create_services_categories(image_path, new_services_categories).await
        .map_err(|e| MyError::Other(Box::new(e)))?;
    
    Ok(HttpResponse::Ok().json(services_categories))

}

#[get("/services_categories/{id}")]
pub async fn get_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let poggers = db.get_services_categories_by_id(&id);
    match poggers {
        Some(poggers) => HttpResponse::Ok().json(poggers),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}

#[get("/services_categories")]
pub async fn get_services_categories(db: web::Data<Database>) -> HttpResponse {
    let services_categories = db.get_services_categories();
    HttpResponse::Ok().json(services_categories)
}

#[delete("/services_categories/{id}")]
pub async fn delete_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let services_categories = db.delete_services_categories_by_id(&id);
    match services_categories {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}
#[put("/services_categories/{id}")]
pub async fn update_services_categories_by_id(db: web::Data<Database>, id: web::Path<String>, mut payload: Multipart) -> HttpResponse {
    let max_file_count: usize = 1;
    let mut current_count: usize = 0;
    let legal_filetypes:[Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_BMP];
    let filename = Uuid::new_v4();
    let dir: &str = "upload/";

    let mut json_data = None;
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition();
        let field_name = content_disposition.get_name().ok_or(MyError::CustomError("Missing 'data' field".to_string())).unwrap();

        if field_name == "image" {
            if current_count == max_file_count {
                continue;
            }

            let filetype: Option<&Mime> = field.content_type();
            if filetype.is_none() {
                continue;
            }

            if !legal_filetypes.contains(&filetype.unwrap()) {
                continue;
            }

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap()
            );

            let mut saved_file: fs::File = fs::File::create(&destination).await.unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).await.unwrap();
            }

            web::block(move || async move {
                let uploaded_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).await.unwrap();
                uploaded_img
                    .resize_exact(300, 300, FilterType::Gaussian)
                    .save(format!("{}{}.png", dir, filename.to_string())).unwrap();
            }).await.unwrap().await;

            current_count += 1;
        } else if field_name == "data" {
            let mut data = Vec::new();
            while let Some(chunk) = field.try_next().await.unwrap() {
                data.extend_from_slice(&chunk);
            }

            json_data = Some(serde_json::from_slice::<Value>(&data).unwrap());
        }
    }

    let json_data = json_data.ok_or(MyError::CustomError("Error json data".to_string())).unwrap();

    let updated_services_categories: ServiceCategory = serde_json::from_value(json_data.get("updated_services_categories").cloned().ok_or_else(|| MyError::CustomError("Missing 'updated_services_categories' field".to_string())).unwrap())
        .map_err(|err| MyError::CustomError(format!("Invalid 'updated_services_categories': {}", err))).unwrap();

    let image_path = if current_count > 0 {
        dir.to_owned() + &filename.to_string().to_owned() + ".png"
    } else {
        "".to_string()
    };

    let services_categories = db
        .update_services_categories_by_id(&id, image_path, updated_services_categories).await;

    match services_categories {
        Some(services_categories) => HttpResponse::Ok().json(services_categories),
        None => HttpResponse::NotFound().body("Service Category not found"),
    }
}