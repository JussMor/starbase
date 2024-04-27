use ntex::web::{get, HttpResponse, Responder, ServiceConfig};
use std::fs;
use std::time::Duration;
use mongodb::Database;
use mongodb::options::{GridFsBucketOptions, WriteConcern};
use futures_util::io::AsyncWriteExt;
use ntex::web::types::State;
use tokio::task::spawn_blocking;
use crate::HANDLEBARS;

#[derive(Debug, serde::Serialize)]
struct SomeData {
    message: String,
}

#[derive(Debug, serde::Serialize)]
struct ImageData {
    message: String,
}

#[get("/test")]
async fn hello2(db: State<Database>) -> impl Responder {
    let data = SomeData {
        message: "Hello Page 2".to_string(),
    };

    let body = HANDLEBARS.render("hello2", &data).unwrap();
    let collection = db.collection::<ImageData>("images");

    let data = vec![
        ImageData {
            message: "Junior".to_string()
        }
    ];
    collection.insert_many(data, None).await.unwrap();

    let wc = WriteConcern::builder().w_timeout(Duration::new(5, 0)).build();
    let bucket_options = GridFsBucketOptions::builder()
        .bucket_name("images_files".to_string())
        .write_concern(wc)
        .build();
    
    let bucket = db.gridfs_bucket(bucket_options);

    let file_bytes = match spawn_blocking(|| fs::read("src/handlers/gradient.jpg")).await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(e)) => {
            eprintln!("Error reading file: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            // Handle the spawn_blocking error
            eprintln!("Error executing spawn_blocking: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut upload_stream = bucket.open_upload_stream("example", None);
    upload_stream.write_all(&file_bytes).await.unwrap();

    println!("Document uploaded with ID: {}", upload_stream.id());
    upload_stream.close().await.unwrap();

    HttpResponse::Ok().body(body)
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(hello2);
}
