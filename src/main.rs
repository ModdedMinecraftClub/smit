mod config;

use crate::config::{yaml_from_file_or_create, Config};
use actix_web::{App, HttpServer};
use actix_web_static_files::{build_hashmap_from_included_dir, include_dir, Dir, ResourceFiles};

const FRONTEND_DIR: Dir = include_dir!("frontend/dist");

#[actix_web::main]
async fn main() {
    let config = yaml_from_file_or_create::<Config>("config.yml".as_ref())
        .expect("Failed to load or create config.yml.");
    println!("Bind address is: {}", config.bind_address);

    HttpServer::new(|| {
        let frontend_hash_map = build_hashmap_from_included_dir(&FRONTEND_DIR);
        App::new().service(ResourceFiles::new("/", frontend_hash_map).resolve_not_found_to_root())
    })
    .bind(&config.bind_address)
    .expect("Failed to bind to address")
    .run()
    .await
    .expect("Failed to run Actix.");
}
