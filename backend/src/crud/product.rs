//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::routes::product::{Data, PageQuery, ProductView, SearchQuery, UpdateQuery};
use crate::structures::Product;

use actix_web::Error;
use futures::StreamExt;
use meilisearch_sdk::SearchResults;
use mongodb::bson::doc;
use mongodb::options::{FindOptions, UpdateOptions};
use mongodb::Client;

/// Retrieve detailed data for a specific product.
///
/// This function queries the MongoDB database to retrieve detailed product data based on its ID.

/// # Parameters
///
/// - `pid`: The product ID to retrieve data for.
/// - `client`: MongoDB client instance used for database access.

/// # Returns
///
/// - Returns a `Result` containing an `Option<Data>`, where `Some(data)` represents success with the detailed product data, and `None` indicates that the data was not found. An `Error` is returned in case of a database query error.
pub async fn retrieve_data(pid: String, client: Client) -> Result<Option<Data>, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": pid };

    if let Ok(Some(product)) = collection.find_one(filter, None).await {
        let data = Data {
            pid: product.pid.clone(),
            sid: product.sid.clone(),
            name: product.name.clone(),
            description: product.description.clone(),
            image: product.image.clone(),
            category: product.category.clone(),
            price: product.price,
            stock: product.stock,
            sales: product.sales,
            rating: product.rating,
            clicks: product.clicks
        };

        Ok(Some(data))
    } else {
        Ok(None)
    }
}

/// Retrieve products based on a search query and category filter.
///
/// This function searches for products in the MeiliSearch index based on a search query and a category filter.

/// # Parameters
///
/// - `query`: Query parameters containing the search query and category filter.
/// - `client`: MeiliSearch client instance used for searching.

/// # Returns
///
/// - Returns a `Result` containing a `Vec<Product>` if the search is successful. If there are no products found, an empty `Vec` is returned. If an error occurs during the search, an `Error` is returned.
pub async fn retrieve_products(
    query: SearchQuery,
    client: meilisearch_sdk::Client,
) -> Result<Vec<Product>, Error> {
    let search: SearchResults<Product>;

    if !query.category.is_empty() {
        search = client
            .index("products")
            .search()
            .with_query(query.search.as_str())
            .with_filter(format!("category = \"{}\"", query.category.as_str()).as_str())
            .execute()
            .await
            .unwrap();
    } else {
        search = client
            .index("products")
            .search()
            .with_query(query.search.as_str())
            .execute()
            .await
            .unwrap();
    }

    let products: Vec<Product> = search.hits.into_iter().map(|hit| hit.result).collect();

    Ok(products)
}

/// Retrieve paginated product views.
///
/// This function retrieves product views for a specific page based on pagination parameters.

/// # Parameters
///
/// - `query`: Query parameters containing the page number.
/// - `client`: MongoDB client instance used for database access.

/// # Returns
///
/// - Returns a `Result` containing an `Option<Vec<ProductView>>` if the retrieval is successful. If no product views are found, `None` is returned. An `Error` is returned in case of a database query error.
pub async fn retrieve_paginated_products(
    query: PageQuery,
    client: Client,
) -> Result<Option<Vec<ProductView>>, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let limit = 18;
    let skip = (query.page - 1) * limit;
    let options = FindOptions::builder()
        .skip(skip as u64)
        .limit(limit as i64)
        .build();
    let mut cursor = collection.find(None, options).await.unwrap();
    let mut products_view: Vec<ProductView> = Vec::new();

    while let Some(Ok(product)) = cursor.next().await {
        let product_view = ProductView {
            pid: product.pid.clone(),
            sid: product.sid.clone(),
            name: product.name.clone(),
            image: product.image.clone(),
            price: product.price,
            stock: product.stock,
            sales: product.sales,
            rating: product.rating,
            clicks: product.clicks
        };

        products_view.push(product_view);
    }

    Ok(Some(products_view))
}

/// Updates a product in the MongoDB collection.
///
/// # Arguments
///
/// * `pid` - The product ID (pid) of the product to be updated.
/// * `query` - The update query containing the fields to be updated.
/// * `client` - The MongoDB client.
///
/// # Returns
///
/// A `Result` indicating whether the update was successful.
pub async fn update_product(
    pid: String,
    query: UpdateQuery,
    client: Client,
    ms_client: meilisearch_sdk::Client,
) -> Result<bool, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": pid.clone() };
    let update = doc! {
        "$set": {
            "image": query.image.clone(),
            "price": query.price,
            "stock": query.stock,
            "sales": query.sales,
            "rating": query.rating,
        }
    };

    let options = UpdateOptions::builder().upsert(false).build();

    if let Ok(Some(product)) = collection.find_one(filter.clone(), None).await {
        collection
            .update_one(filter, update, options)
            .await
            .expect("Failed to update product");

        ms_client
            .index("products")
            .add_or_replace(
                &[Product {
                    pid: pid.clone(),
                    sid: product.sid.clone(),
                    name: product.name.clone(),
                    description: product.description.clone(),
                    image: product.image.clone(),
                    category: product.category.clone(),
                    price: query.price,
                    stock: query.stock,
                    sales: query.sales,
                    rating: query.rating,
                    clicks: product.clicks,
                }],
                None,
            )
            .await
            .unwrap();

        Ok(true)
    } else {
        Ok(false)
    }
}
