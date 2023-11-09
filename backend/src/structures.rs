//! Copyright (c) 2023 Mostafa Elbasiouny
//!
//! This software may be modified and distributed under the terms of the MIT license.
//! See the LICENSE file for details.

use serde::{Deserialize, Serialize};

/// A struct representing a product.
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Product {
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

    /// The product number of clicks.
    pub clicks: u32,
}
