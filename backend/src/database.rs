//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::structures::Product;

use futures::TryStreamExt;
use log::{error, info};
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::io::{self, Error};

/// Establish a connection to the MongoDB database.
///
/// This function establishes a connection to a MongoDB database using the provided URL. It creates a MongoDB client and verifies the connection.
///
/// # Returns
///
/// Returns a `Result` containing the MongoDB `Client` if the connection is successful, or an `Error` if an error occurs during connection setup.
pub async fn establish_connection() -> Result<Client, Error> {
    // Parse the client options from the MongoDB URL.
    let client_options = match ClientOptions::parse("mongodb://0.0.0.0:27017").await {
        Ok(options) => options,
        Err(error) => {
            error!("Failed to parse client options: {}", error);
            return Err(io::Error::new(io::ErrorKind::Other, "Database error"));
        }
    };

    // Create a MongoDB client with the parsed options.
    let client = match Client::with_options(client_options) {
        Ok(client) => client,
        Err(error) => {
            error!("Failed to create MongoDB client: {}", error);
            return Err(io::Error::new(io::ErrorKind::Other, "Database error"));
        }
    };

    // Verify the database connection by pinging it.
    let ping_result = client
        .database("ecommerce_db")
        .run_command(doc! {"ping": 1}, None)
        .await;

    match ping_result {
        Ok(_) => {
            info!("Successfully connected to the database");
            Ok(client)
        }
        Err(error) => {
            error!("Failed to ping the database: {}", error);
            return Err(io::Error::new(io::ErrorKind::Other, "Database error"));
        }
    }
}

/// Index products in MeiliSearch.
///
/// This function indexes products in MeiliSearch, a search engine. It fetches products from the MongoDB database, adds them to MeiliSearch, and configures search settings.
///
/// # Parameters
///
/// - `client`: A reference to the MongoDB `Client` used to fetch products.
///
/// # Returns
///
/// Returns a `Result` containing the MeiliSearch `Client` if indexing is successful, or an `Error` if an error occurs during indexing.
pub async fn index_products(client: &Client) -> Result<meilisearch_sdk::Client, Error> {
    // Create a MeiliSearch client with the provided server URL and API key.
    let ms_client = meilisearch_sdk::Client::new(
        "http://0.0.0.0:7700",
        Some("KbKaO3ANX62fyv6gu8UtJlgPwEjKKmaH-s-d9KSiDzE"),
    );

    // Fetch products from the MongoDB database.
    let products_doc = fetch_products(client).await.unwrap();

    info!("Indexing {} product(s)", products_doc.len());

    // Configure searchable attributes, ranking rules and filtering.
    let searchable_attributes = ["name", "description"];
    ms_client
        .index("products")
        .set_searchable_attributes(&searchable_attributes)
        .await
        .unwrap();

    let ranking_rules = ["typo", "words", "proximity", "attribute"];
    ms_client
        .index("products")
        .set_ranking_rules(&ranking_rules)
        .await
        .unwrap();

    ms_client
        .index("products")
        .set_filterable_attributes(&["category"])
        .await
        .unwrap();

    // Add documents to the MeiliSearch index.
    let task_info = ms_client
        .index("products")
        .add_documents(&products_doc, Some("pid"))
        .await
        .unwrap();

    // Check the status of the indexing task.
    loop {
        let task_status = ms_client.get_task(&task_info).await.unwrap();

        if task_status.is_failure() {
            error!("Indexing task failed");
            return Err(io::Error::new(io::ErrorKind::Other, "Indexing error"));
        } else if task_status.is_success() {
            info!("Successfully indexed {} product(s)", products_doc.len());
            break;
        }

        actix_rt::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    Ok(ms_client)
}

/// Fetch products from the MongoDB database.
///
/// This function retrieves a list of products from the MongoDB database.
///
/// # Parameters
///
/// - `client`: A reference to the MongoDB `Client` used to fetch products.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `Product` structs if the fetch operation is successful, or an `Error` if an error occurs during fetching.
async fn fetch_products(client: &Client) -> Result<Vec<Product>, Error> {
    // Get a collection of products from the MongoDB database.
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let cursor = match collection.find(None, None).await {
        Ok(cursor) => cursor,
        Err(error) => {
            error!("Error while fetching products: {}", error);
            return Err(io::Error::new(io::ErrorKind::Other, "Database error"));
        }
    };

    // Convert the fetched products to a vector.
    let products: Vec<Product> = cursor.try_collect().await.unwrap_or_else(|_| vec![]);

    Ok(products)
}
