use axum::{http::header, response::{IntoResponse, Response}};
use tokio::fs;

pub async fn img_response(image_resource:&String) -> Response{
    let image_resource = match image::open(image_resource) {
        Ok(image) => image,
        Err(_) => return (axum::http::StatusCode::NOT_FOUND,"Image not found").into_response()
    };

    let resized_image = image::imageops::resize(&image_resource, 500, 500, image::imageops::FilterType::Triangle);
    
    let image_resource = image::DynamicImage::ImageRgba8(resized_image);

    let mut img_bytes:Vec<u8> = Vec::new();
    
    image_resource.write_to(&mut std::io::Cursor::new(&mut img_bytes),
    image::ImageFormat::Png).expect("Can't parse image");

    println!("Showing image");

    ([(header::CONTENT_TYPE, "image/png")], img_bytes).into_response()
}

pub async fn count_folder_imgs() -> i32 {
    let mut entries = match fs::read_dir("./images").await {
        Ok(e) => e,
        Err(_) => return -1
    };
    
    let mut img_qtd = 0;

    while let Ok(Some(_entry)) = entries.next_entry().await{
        img_qtd += 1;
    }

    return img_qtd;
}