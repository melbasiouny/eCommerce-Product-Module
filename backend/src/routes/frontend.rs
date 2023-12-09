use actix_web::{web, Error, HttpResponse};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct CartData {
    /// The product ID formatted as (P0000...).
    pub id: String,

    /// The seller ID formatted as (S0000...).
    pub sellerid: String,

    /// The product name.
    pub name: String,

    /// The product description.
    pub description: String,

    /// The product image as a URL.
    pub imgurl: String,

    /// The product category.
    pub cost: f64,
}

#[get("/api/frontend/addtocart/{uid}")]
pub async fn route_cart(
    path: web::Path<(String,)>,
    json: web::Json<CartData>,
) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::new();

    client
        .post(format!("https://shoppingcart.honeybeeks.net/api/upload/{}", path.0.clone()))
        .json(&json)
        .send();

    Ok(HttpResponse::Ok().finish())
}

#[get("/api/frontend/addtowishlist/{uid}")]
pub async fn route_wishlist(
    path: web::Path<(String,)>,
    json: web::Json<CartData>,
) -> Result<HttpResponse, Error> {
    let client = reqwest::Client::new();

    client
        .post(format!("https://shoppingcart.honeybeeks.net/api/wishlist/upload/{}", path.0.clone()))
        .json(&json)
        .send();

    Ok(HttpResponse::Ok().finish())
}