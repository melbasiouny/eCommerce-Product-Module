//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::crud::profile;

use actix_web::{web, Error, HttpResponse};
use mongodb::Client;
use serde::Deserialize;

/// Struct representing the query parameters for adding a product.
#[derive(Deserialize)]
pub struct ProductQuery {
    /// The product ID formatted as (P0000...).
    pub pid: String,

    /// The seller ID formatted as (S0000...).
    pub sid: String,

    /// The product name.
    pub name: String,

    /// The product description.
    pub description: String,

    /// The product image as a URL.
    pub image: String,

    /// The product category.
    pub category: String,

    /// The product price.
    pub price: f32,

    /// The product stock level.
    pub stock: u32,
}

/// Retrieve products sold by the specified seller id.
///
/// This function is an Actix web handler for retrieving products sold by a specific seller. It receives the seller id as a path parameter and delegates the operation to the `profile::retrieve_seller_products` function.

/// # Parameters
///
/// - `path`: Path parameters containing the seller id.
/// - `client`: MongoDB client data.

/// # Returns
///
/// - Returns an `HttpResponse` indicating success or an error. If the operation was successful and the seller has products, it returns `Ok(HttpResponse::Ok().content_type("application/json").json(products))`. If the seller exists but has no products, it returns `Ok(HttpResponse::NoContent().finish())`. If the operation was not successful, it returns `Ok(HttpResponse::InternalServerError().finish())`.
#[get("/api/profile/{sid}/products")]
pub async fn seller_products(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
) -> HttpResponse {
    let (products, success) =
        profile::retrieve_seller_products(path.0.clone(), client.get_ref().clone()).await;
    if success {
        if !products.is_empty() {
            HttpResponse::Ok()
                .content_type("application/json")
                .json(products)
        } else {
            HttpResponse::NoContent().finish()
        }
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

/// Add a product to the seller's profile.
///
/// This function is an Actix web handler for adding a product to a seller's profile. It receives product information as query parameters and delegates the operation to the `profile::list_product` function.

/// # Parameters
///
/// - `query`: Query parameters containing the product information.
/// - `client`: MongoDB client data.
/// - `ms_client`: MeiliSearch client data.

/// # Returns
///
/// - Returns an `HttpResponse` indicating success or an error, such as `Ok(HttpResponse::Ok())`, `Ok(HttpResponse::Conflict())`, or `Ok(HttpResponse::InternalServerError())`.
#[post("/api/profile/seller/add/product")]
pub async fn add_product(
    query: web::Query<ProductQuery>,
    client: web::Data<Client>,
    ms_client: web::Data<meilisearch_sdk::Client>,
) -> Result<HttpResponse, Error> {
    match profile::list_product(
        query.into_inner(),
        client.get_ref().clone(),
        ms_client.get_ref().clone(),
    )
    .await
    {
        Ok(true) => Ok(HttpResponse::Ok().finish()),
        Ok(false) => Ok(HttpResponse::Conflict().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Remove a product from the seller's profile.
///
/// This function is an Actix web handler for removing a product from a seller's profile. It receives the product ID as a path parameter and delegates the operation to the `profile::delist_product` function.

/// # Parameters
///
/// - `path`: Path parameter containing the product ID.
/// - `client`: MongoDB client data.
/// - `ms_client`: MeiliSearch client data.

/// # Returns
///
/// - Returns an `HttpResponse` indicating success or an error, such as `Ok(HttpResponse::Ok())`, `Ok(HttpResponse::NoContent())`, or `Ok(HttpResponse::InternalServerError())`.
#[delete("/api/profile/seller/remove/product/{pid}")]
pub async fn remove_product(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
    ms_client: web::Data<meilisearch_sdk::Client>,
) -> Result<HttpResponse, Error> {
    match profile::delist_product(
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
