//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::crud::analytics;

use actix_web::{web, Error, HttpResponse};
use mongodb::Client;
use serde::{Deserialize, Serialize};

/// Represents the data structure for analytics information.
#[derive(Default, Deserialize, Serialize)]
pub struct Data {
    /// The product ID formatted as (P0000...).
    pub pid: String,

    /// The seller ID formatted as (S0000...).
    pub sid: String,

    /// The product stock level.
    pub stock: u32,

    /// The product total number of sales.
    pub sales: u32,

    /// The product rating using a five-star system.
    pub rating: f32,

    /// The product number of clicks.
    pub clicks: u32,
}

/// Retrieve analytics data for a specific product.
///
/// This function is an Actix web handler for fetching analytics data for a product based on its ID.

/// # Parameters
///
/// - `path`: Path parameter containing the product ID.
/// - `client`: MongoDB client data.

/// # Returns
///
/// - Returns an `HttpResponse` with analytics data in JSON format if found, `HttpResponse::NoContent()` if not found, or an `HttpResponse::InternalServerError()` in case of an error.
#[get("/api/analytics/{pid}")]
pub async fn data(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    match analytics::retrieve_data(path.0.clone(), client.get_ref().clone()).await {
        Ok(Some(data)) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(data)),
        Ok(None) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Increment the number of clicks for a specific product.
///
/// This function is an Actix web handler for incrementing the number of clicks for a product based on its ID.

/// # Parameters
///
/// - `path`: Path parameter containing the product ID.
/// - `client`: MongoDB client data.

/// # Returns
///
/// - Returns an `HttpResponse` indicating success or an error, such as `Ok(HttpResponse::Ok())`, `Ok(HttpResponse::NoContent())`, or `Ok(HttpResponse::InternalServerError())`.
#[post("/api/analytics/{pid}/clicks/increment")]
pub async fn increment(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
    ms_client: web::Data<meilisearch_sdk::Client>,
) -> Result<HttpResponse, Error> {
    match analytics::increment_clicks(
        path.0.clone(),
        client.get_ref().clone(),
        ms_client.get_ref().clone(),
    )
    .await
    {
        Ok(true) => Ok(HttpResponse::Ok().finish()),
        Ok(false) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
