use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::Once;
use ntex::web::{get, HttpResponse, ServiceConfig, Responder, HttpRequest};
use ntex_bytes::Bytes;
use reqwest;
// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

#[get("/resizing/{image_url}")]
async fn resizing_image(req: HttpRequest) -> impl Responder {

    START.call_once(|| {
        magick_wand_genesis();
    });


    let _image_url = req.match_info().get("image_url").unwrap_or_default();
    let image_data = match reqwest::get("https://cdn.jobsity.com/picture_home_1_2_153924836f/picture_home_1_2_153924836f.webp").await {
        Ok(res) => res.bytes().await.unwrap(),
        Err(err) => {
            eprintln!("Failed to fetch image data: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Create a new MagickWand and load image data
    let wand = MagickWand::new();
    if let Err(err) = wand.read_image_blob(&image_data) {
    eprintln!("Failed to read image: {:?}", err);
    return HttpResponse::InternalServerError().finish();
}


    // Resize the image
    let width = 1000; // Specify your desired width
    let height = 1000; // Specify your desired height
    wand.resize_image(width, height, magick_rust::FilterType::default());

    // Get the resized image data
    let resized_data = wand.write_image_blob("webp").unwrap();

    // Return the resized image as the response
    HttpResponse::Ok()
        .content_type("image/webp")
        .body(Bytes::from(resized_data))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(resizing_image);
}