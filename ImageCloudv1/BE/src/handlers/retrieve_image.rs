use futures_util::TryStreamExt;
use mongodb::bson::doc;
use ntex::web::{get, HttpResponse, Responder, ServiceConfig};
use ntex::web::types::State;
use std::time::Duration;
use mongodb::Database;
use mongodb::options::{GridFsBucketOptions, WriteConcern};



#[get("/images")]
async fn list_images(db: State<Database>) -> impl Responder { 

    let wc = WriteConcern::builder().w_timeout(Duration::new(5, 0)).build();
    let bucket_options = GridFsBucketOptions::builder()
        .bucket_name("images_files".to_string())
        .write_concern(wc)
        .build();
    
    let bucket = db.gridfs_bucket(bucket_options);

    let filter = doc! {};
    let mut cursor = bucket.find(filter, None).await.unwrap();
    let mut response_body = String::new();


    // Iterate over the files in the cursor
    while let Some(file) = cursor.try_next().await.unwrap() {
        // Retrieve information about the file
        let filename = file.filename.unwrap_or_else(|| "Unnamed File".to_string());

        // Assuming you have a route like "/images/{filename}" to access each image
        let image_url = format!("/images/{}", filename); // Use the actual field containing the filename

        // Append the file information to the response body
        response_body.push_str(&format!("File: {}\nImage URL: {}\n\n", filename, image_url));
    }

    HttpResponse::Ok().body(response_body)
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(list_images);
}