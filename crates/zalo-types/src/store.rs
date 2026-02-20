// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Store and Order types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Product status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    /// Product is active.
    Active,
    /// Product is inactive.
    Inactive,
    /// Product is out of stock.
    OutOfStock,
}

/// Product information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreProduct {
    /// Product ID.
    pub id: String,
    /// Product name.
    pub name: String,
    /// Product description.
    pub description: String,
    /// Product code/SKU.
    pub code: String,
    /// Price in VND.
    pub price: u64,
    /// Photo URLs.
    pub photos: Vec<String>,
    /// Product status.
    pub status: ProductStatus,
    /// Creation timestamp.
    pub created_at: u64,
    /// Last update timestamp.
    pub updated_at: u64,
}

/// Request to create a product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductRequest {
    /// Product name.
    pub name: String,
    /// Product description.
    pub description: String,
    /// Product code/SKU.
    pub code: String,
    /// Price in VND.
    pub price: u64,
    /// Photo URLs.
    pub photos: Vec<String>,
    /// Product status.
    pub status: ProductStatus,
}

/// Shipping information for an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingInfo {
    /// Recipient name.
    pub name: String,
    /// Recipient phone.
    pub phone: String,
    /// Recipient address.
    pub address: String,
    /// City/Province.
    pub city: String,
    /// District.
    pub district: String,
    /// Ward.
    pub ward: String,
}

/// Order item (product in an order).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    /// Product ID.
    pub product_id: String,
    /// Product name.
    pub product_name: String,
    /// Quantity.
    pub quantity: u64,
    /// Price per item in VND.
    pub price: u64,
}

/// Order status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// Order is pending.
    Pending,
    /// Order is confirmed.
    Confirmed,
    /// Order is being prepared.
    Preparing,
    /// Order is shipped.
    Shipped,
    /// Order is delivered.
    Delivered,
    /// Order is cancelled.
    Cancelled,
}

/// Store order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreOrder {
    /// Order ID.
    pub id: String,
    /// User ID who placed the order.
    pub user_id: String,
    /// Order items.
    pub items: Vec<OrderItem>,
    /// Shipping information.
    pub shipping: ShippingInfo,
    /// Shipping fee in VND.
    pub shipping_fee: u64,
    /// Discount amount in VND.
    pub discount: u64,
    /// Total amount in VND.
    pub total: u64,
    /// Order status.
    pub status: OrderStatus,
    /// Creation timestamp.
    pub created_at: u64,
    /// Last update timestamp.
    pub updated_at: u64,
}

/// Request to create an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    /// User ID.
    pub user_id: String,
    /// Order items.
    pub items: Vec<OrderItem>,
    /// Shipping information.
    pub shipping: ShippingInfo,
    /// Shipping fee in VND.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_fee: Option<u64>,
    /// Discount amount in VND.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<u64>,
}

/// Query parameters for listing orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderListQuery {
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    /// Filter by status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
}

/// List of orders response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    /// List of orders.
    pub orders: Vec<StoreOrder>,
    /// Total number of orders.
    pub total: u64,
}
