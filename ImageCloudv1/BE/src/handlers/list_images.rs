use futures_util::AsyncReadExt;
use mongodb::bson::oid::ObjectId;
use ntex::web::{get, HttpResponse, Responder, ServiceConfig, HttpRequest};
use ntex::web::types::State;
use std::str::FromStr;
use std::time::Duration;
use mongodb::{Database, bson::Bson, options::{GridFsBucketOptions, WriteConcern}};
use ntex_bytes::Bytes;

#[get("/images/{filename}")]
async fn list_one_image(db: State<Database>, req: HttpRequest) -> impl Responder {
    let wc = WriteConcern::builder().w_timeout(Duration::new(5, 0)).build();
    let bucket_options = GridFsBucketOptions::builder()
        .bucket_name("images_files".to_string())
        .write_concern(wc)
        .build();

    let bucket = db.gridfs_bucket(bucket_options);

    let filename = req.match_info().get("filename").unwrap_or_default();
    let filter = ObjectId::from_str(filename).expect("Could not convert to ObjectId");

    match bucket.open_download_stream(Bson::ObjectId(filter)).await {
        Ok(mut stream) => {
            // Read the image content from the stream
            let mut content = Vec::new();
            let _result =  stream.read_to_end(&mut content).await;

            // Build the response with the image content
            let bytes_content = Bytes::from(content);
            HttpResponse::Ok()
                .content_type("image/jpeg") // Adjust content type based on your image type
                .body(bytes_content)
        }
        Err(_) => {
            // Handle error when the image is not found
            HttpResponse::NotFound().finish()
        }
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(list_one_image);
}