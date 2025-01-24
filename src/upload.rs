use axum::http::{header::HeaderMap, StatusCode};
use axum::{extract::Multipart, response::IntoResponse};
use dotenv::dotenv;
use std::env;
use tokio::io::AsyncWriteExt;
use crate::vonage;

pub async fn upload_handler(headers: HeaderMap, mut multipart: Multipart) -> impl IntoResponse {
    dotenv().ok();

    if let Some(number) = headers.get("Number") {
        let from_number = number.to_str().expect("Work").to_string();
        println!("{}", from_number);


        let mut file_name = String::new();
        let mut file_data = Vec::new();

        while let Some(field) = multipart.next_field().await.unwrap() {
            if let Some(name) = field.file_name() {
                file_name = name.to_string();
            }

            if field.content_type().unwrap() != "text/csv" {
                return (
                    StatusCode::BAD_REQUEST,
                    "Only CSV files allowed".to_string(),
                );
            }

            let data = field.bytes().await.unwrap();
            file_data.extend_from_slice(&data);
        }

        if !file_data.is_empty() {
            let mut file = tokio::fs::File::create("tmp/uploaded_file.csv")
                .await
                .unwrap();
            file.write_all(&file_data).await.unwrap();
            println!("Uploaded file: {}, size: {}", file_name, file_data.len());
            let uri = env::var("URI").unwrap();
            vonage::call::prep_call(file_name, from_number, uri).await;
            return (StatusCode::OK, "File uploaded successfully!".to_string());
        }

        return (StatusCode::BAD_REQUEST, "No file found".to_string());
    }

    (StatusCode::UNAUTHORIZED, "No token found".to_string())
}
