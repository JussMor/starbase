use std::io::Result;
use ntex_cors::Cors;
use handlebars::Handlebars;
use lazy_static::lazy_static;
use dotenv::dotenv;
use mongodb::{Client,Database, options::ClientOptions};
use std::env;
use ntex::web::{App, server};
mod handlers;

lazy_static! {
    pub static ref HANDLEBARS: Handlebars<'static> = {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("hello", "src/views/index.hbs")
            .expect("Failed to register template");
        handlebars
            .register_template_file("hello2","src/views/prueba.hbs")
            .expect("Failed to register template");
        handlebars
    };
}


async fn configure_mongodb() -> Database {
    let  connection_string = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    let client = Client::with_options(connection_string).unwrap();
    
    client.database("image_cloud")
}


#[ntex::main]
async fn main() -> Result<()> {
    // Load Enviroment Variables
    dotenv().ok();

    let database  = configure_mongodb().await;

    // Create Env Variables
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set");

    server(move || {
        App::new()
        .state(database.clone())
        .wrap(
            Cors::new()
            .allowed_origin("http://localhost:4321")
            .finish())
            .configure(handlers::hello::config) 
            .configure(handlers::hello2::config)
            .configure(handlers::resizing_image::config)
            .configure(handlers::retrieve_image::config)
            .configure(handlers::list_images::config)
    })
    .bind(server_address)?
    .run()
    .await
}