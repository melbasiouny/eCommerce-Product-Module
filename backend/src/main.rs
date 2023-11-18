//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

#[macro_use]
extern crate actix_web;
extern crate log;
extern crate mongodb;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use std::{env, fs, io};

mod crud;
mod database;
mod routes;
mod structures;

async fn not_found_handler(request: HttpRequest) -> HttpResponse {
    let _ = request.path().to_string();
    let content = fs::read_to_string("backend/src/pages/404.html")
        .unwrap_or_else(|_| "404 Not Found".to_string());

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(content)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Set up logging.
    // env::set_var("RUST_LOG", "error,info,actix_web=debug,actix_server=info");
    // env_logger::init();

    // Establish a connection to the database.
    let client = database::establish_connection().await.unwrap();

    // Index products from the database.
    let products_index = database::index_products(&client).await.unwrap();

    // Create shared data objects to share the client and product index across services.
    let shared_client = web::Data::new(client);
    let shared_products_index = web::Data::new(products_index);

    // Configure the HTTP server.
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600);

        App::new()
            // .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(shared_client.clone())
            .app_data(shared_products_index.clone())
            .service(routes::analytics::data)
            .service(routes::analytics::increment)
            .service(routes::product::data)
            .service(routes::product::search)
            .service(routes::product::page)
            .service(routes::product::update)
            .service(routes::profile::add_product)
            .service(routes::profile::remove_product)
            .default_service(web::to(not_found_handler))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
