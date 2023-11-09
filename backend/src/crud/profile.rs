//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::routes::profile::ProductQuery;
use crate::structures::Product;

use actix_web::Error;
use mongodb::bson::doc;
use mongodb::Client;

/// List a product in the database.
///
/// This function attempts to add a new product to the database. If the product with the same `pid` already exists, it returns `false`. If the addition is successful, it returns `true`.

/// # Parameters
///
/// - `query`: Product information to be added.
/// - `client`: MongoDB client instance used for database access.
/// - `ms_client`: MeiliSearch client instance used for indexing.

/// # Returns
///
/// - Returns `true` if the product is successfully listed, `false` if the product with the same `pid` already exists, and an `Error` in case of a database or indexing error.
pub async fn list_product(
    query: ProductQuery,
    client: Client,
    ms_client: meilisearch_sdk::Client,
) -> Result<bool, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": query.pid.clone() };

    if let Ok(Some(_)) = collection.find_one(filter, None).await {
        Ok(false)
    } else {
        let product = Product {
            pid: query.pid.clone(),
            sid: query.sid.clone(),
            name: query.name.clone(),
            description: query.description.clone(),
            image: query.image.clone(),
            category: query.category.clone(),
            price: query.price,
            stock: query.stock,
            ..Product::default()
        };

        let _ = collection.insert_one(product.clone(), None).await;
        let mut product_doc: Vec<Product> = Vec::new();

        product_doc.push(product.clone());
        ms_client
            .index("products")
            .add_documents(&product_doc, Some("pid"))
            .await
            .unwrap();

        Ok(true)
    }
}

/// Delist a product from the database.
///
/// This function attempts to remove a product from the database. If the product with the given `pid` exists and is successfully delisted, it returns `true`. If the product does not exist, it returns `false`.

/// # Parameters
///
/// - `pid`: The `pid` of the product to be delisted.
/// - `client`: MongoDB client instance used for database access.
/// - `ms_client`: MeiliSearch client instance used for indexing.

/// # Returns
///
/// - Returns `true` if the product is successfully delisted, `false` if the product with the given `pid` does not exist, and an `Error` in case of a database or indexing error.
pub async fn delist_product(
    pid: String,
    client: Client,
    ms_client: meilisearch_sdk::Client,
) -> Result<bool, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": pid.clone() };

    if let Ok(Some(_)) = collection.find_one_and_delete(filter, None).await {
        ms_client
            .index("products")
            .delete_document(pid.clone())
            .await
            .unwrap();

        Ok(true)
    } else {
        Ok(false)
    }
}
