mod config;

use crate::config::{yaml_from_file_or_create, Config};
use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() {
    let config = yaml_from_file_or_create::<Config>("config.yml".as_ref())
        .expect("Failed to load or create config.yml.");
    println!("Bind address is: {}", config.bind_address);

    HttpServer::new(|| App::new().service(Files::new("/", "frontend/dist")))
        .bind(&config.bind_address)
        .expect("Failed to bind to address")
        .run()
        .await
        .expect("Failed to run Actix.");
}
