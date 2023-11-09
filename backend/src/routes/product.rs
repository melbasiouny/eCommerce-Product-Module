//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use crate::crud::product;

use actix_web::{web, Error, HttpResponse};
use mongodb::Client;
use serde::{Deserialize, Serialize};

/// Represents the data structure for product information.
#[derive(Default, Deserialize, Serialize)]
pub struct Data {
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

    /// The product total number of sales.
    pub sales: u32,

    /// The product rating using a five-star system.
    pub rating: f32,
}

/// Represents the data structure for a simplified product view.
#[derive(Default, Deserialize, Serialize)]
pub struct ProductView {
    /// The product ID formatted as (P0000...).
    pub pid: String,

    /// The seller ID formatted as (S0000...).
    pub sid: String,

    /// The product name.
    pub name: String,

    /// The product image as a URL.
    pub image: String,

    /// The product price.
    pub price: f32,

    /// The product rating using a five-star system.
    pub rating: f32,
}

/// Represents the query parameters for a product search.
#[derive(Deserialize)]
pub struct SearchQuery {
    /// The search category.
    pub category: String,

    /// The search query.
    pub search: String,
}

/// Represents the query parameters for paginating product data.
#[derive(Deserialize)]
pub struct PageQuery {
    /// The page number for pagination.
    pub page: u32,
}

/// Represents the query parameters for updating product data.
#[derive(Deserialize)]
pub struct UpdateQuery {
    /// The product image as a URL.
    pub image: String,

    /// The product price.
    pub price: f32,

    /// The product stock level.
    pub stock: u32,

    /// The product total number of sales.
    pub sales: u32,

    /// The product rating using a five-star system.
    pub rating: f32,
}

/// Retrieve data for a specific product.
///
/// This function is an Actix web handler for fetching detailed data for a product based on its ID.

/// # Parameters
///
/// - `path`: Path parameter containing the product ID.
/// - `client`: MongoDB client data.

/// # Returns
///
/// - Returns an `HttpResponse` with the product data in JSON format if found, `HttpResponse::NoContent()` if not found, or an `HttpResponse::InternalServerError()` in case of an error.
#[get("/api/product/{pid}/data")]
pub async fn data(
    path: web::Path<(String,)>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    match product::retrieve_data(path.0.clone(), client.get_ref().clone()).await {
        Ok(Some(data)) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(data)),
        Ok(None) => Ok(HttpResponse::NoContent().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Search for products based on a query.
///
/// This function is an Actix web handler for searching for products based on a search query.

/// # Parameters
///
/// - `query`: Query parameters containing the search query.
/// - `ms_client`: MeiliSearch client data.

/// # Returns
///
/// - Returns an `HttpResponse` with search results in JSON format if successful, or an `HttpResponse::InternalServerError()` in case of an error.
#[get("/api/product")]
pub async fn search(
    query: web::Query<SearchQuery>,
    _client: web::Data<Client>,
    ms_client: web::Data<meilisearch_sdk::Client>,
) -> Result<HttpResponse, Error> {
    match product::retrieve_products(query.into_inner(), ms_client.get_ref().clone()).await {
        Ok(products) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(products)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Paginate and retrieve product data.
///
/// This function is an Actix web handler for paginating and fetching product data for a specific page.

/// # Parameters
///
/// - `query`: Query parameters containing the page number for pagination.
/// - `client`: MongoDB client data.

/// # Returns
///
/// - Returns an `HttpResponse` with paginated product data in JSON format if successful, or an `HttpResponse::InternalServerError()` in case of an error.
#[get("/api/product/view")]
pub async fn page(
    query: web::Query<PageQuery>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    match product::retrieve_paginated_products(query.into_inner(), client.get_ref().clone()).await {
        Ok(products) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(products)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

/// Update product information.
///
/// This function is an Actix web handler for updating a product. It receives the product ID as a path parameter and query parameters and delegates the operation to the `profile::update_product` function.
///
/// # Parameters
///
/// - `path`: The product ID (pid) extracted from the URL path.
/// - `query`: The update query containing the fields to be updated, parsed from the request query parameters.
/// - `client`: The Actix web data object containing the MongoDB client connection.
///
/// # Returns
///
/// - Returns an `HttpResponse` indicating success or an error, such as `Ok(HttpResponse::Ok())`, `Ok(HttpResponse::NoContent())`, or `Ok(HttpResponse::InternalServerError())`.
#[patch("/api/product/{pid}")]
pub async fn update(
    path: web::Path<(String,)>,
    query: web::Query<UpdateQuery>,
    client: web::Data<Client>,
    ms_client: web::Data<meilisearch_sdk::Client>,
) -> Result<HttpResponse, Error> {
    match product::update_product(
        path.0.clone(),
        query.into_inner(),
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
