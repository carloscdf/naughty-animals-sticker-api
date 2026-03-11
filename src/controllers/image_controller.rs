use axum::{extract::Path, http::StatusCode, response::{Response, IntoResponse}};
use rand::RngExt;

use crate::services::image_service;

pub async fn input_path(Path(id): Path<String>) -> Response{
    let path = format!("images/sticker ({}).png", id);

    image_service::img_response(&path).await
}

pub async fn random_path() -> Response{
    let img_qtd = image_service::count_folder_imgs().await;

    if img_qtd <= 0{
        return (StatusCode::INTERNAL_SERVER_ERROR, "Folder doesn't exist or is empty").into_response();
    }

    let index = {
        let mut rng = rand::rng();
        rng.random_range(1..=img_qtd)
    };
    
    let path = format!("images/sticker ({}).png", index);
    image_service::img_response(&path).await
}

