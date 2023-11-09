//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::routes::analytics::Data;
use crate::structures::Product;

use actix_web::Error;
use mongodb::bson::doc;
use mongodb::Client;

/// Retrieve analytics data for a specific product.
///
/// This function queries the MongoDB database to retrieve analytics data for a product based on its ID.

/// # Parameters
///
/// - `pid`: The product ID to retrieve analytics data for.
/// - `client`: MongoDB client instance used for database access.

/// # Returns
///
/// - Returns a `Result` containing an `Option<Data>`, where `Some(data)` represents success with the analytics data, and `None` indicates that the data was not found. An `Error` is returned in case of a database query error.
pub async fn retrieve_data(pid: String, client: Client) -> Result<Option<Data>, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": pid };

    if let Ok(Some(product)) = collection.find_one(filter, None).await {
        let data = Data {
            pid: product.pid.clone(),
            sid: product.sid.clone(),
            stock: product.stock,
            sales: product.sales,
            rating: product.rating,
            clicks: product.clicks,
        };

        Ok(Some(data))
    } else {
        Ok(None)
    }
}

/// Increment the number of clicks for a specific product.
///
/// This function updates the number of clicks for a product in the MongoDB database based on its ID.

/// # Parameters
///
/// - `pid`: The product ID to increment clicks for.
/// - `client`: MongoDB client instance used for database access.

/// # Returns
///
/// - Returns `true` if the click increment operation is successful, `false` if it fails, and an `Error` in case of a database query error.
pub async fn increment_clicks(
    pid: String,
    client: Client,
    ms_client: meilisearch_sdk::Client,
) -> Result<bool, Error> {
    let collection = client
        .database("ecommerce_db")
        .collection::<Product>("products");
    let filter = doc! { "pid": pid.clone() };
    let update = doc! { "$inc": { "clicks": 1 } };

    if let Ok(_) = collection.update_one(filter.clone(), update, None).await {
        let product = collection
            .find_one(filter.clone(), None)
            .await
            .unwrap()
            .unwrap();

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
                    price: product.price,
                    stock: product.stock,
                    sales: product.sales,
                    rating: product.rating,
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
