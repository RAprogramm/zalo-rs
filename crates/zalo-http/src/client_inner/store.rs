// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Store API endpoints.

use crate::client_inner::{endpoint_url, get_with_query, post_json};
use crate::error::HttpResult;
use zalo_types::store::{CreateOrderRequest, CreateProductRequest, OrderList, OrderListQuery, StoreOrder, StoreProduct, OrderStatus};

/// Creates a product.
pub async fn create_product(
    token: &str,
    request: CreateProductRequest,
) -> HttpResult<StoreProduct> {
    let url = endpoint_url("store/product/create")?;

    tracing::debug!(
        endpoint = %url,
        name = %request.name,
        code = %request.code,
        "creating product"
    );

    post_json(token, url, &request).await
}

/// Updates a product.
pub async fn update_product(
    token: &str,
    _product_id: impl Into<String>,
    request: CreateProductRequest,
) -> HttpResult<StoreProduct> {
    let url = endpoint_url("store/product/update")?;

    tracing::debug!(
        endpoint = %url,
        name = %request.name,
        "updating product"
    );

    post_json(token, url, &request).await
}

/// Gets a product by ID.
pub async fn get_product(
    token: &str,
    product_id: impl AsRef<str>,
) -> HttpResult<StoreProduct> {
    let url = endpoint_url("store/product/detail")?;

    tracing::debug!(
        endpoint = %url,
        product_id = product_id.as_ref(),
        "fetching product"
    );

    get_with_query(token, url, &[("product_id", product_id.as_ref())]).await
}

/// Lists products.
pub async fn list_products(
    token: &str,
    offset: Option<u64>,
    count: Option<u64>,
) -> HttpResult<Vec<StoreProduct>> {
    let url = endpoint_url("store/product/list")?;

    tracing::debug!(
        endpoint = %url,
        offset = ?offset,
        count = ?count,
        "listing products"
    );

    let mut query = Vec::new();
    if let Some(o) = offset {
        query.push(("offset", o.to_string()));
    }
    if let Some(c) = count {
        query.push(("count", c.to_string()));
    }

    get_with_query(token, url, &query).await
}

/// Creates an order.
pub async fn create_order(
    token: &str,
    request: CreateOrderRequest,
) -> HttpResult<StoreOrder> {
    let url = endpoint_url("store/order/create")?;

    tracing::debug!(
        endpoint = %url,
        user_id = %request.user_id,
        "creating order"
    );

    post_json(token, url, &request).await
}

/// Updates an order.
pub async fn update_order(
    token: &str,
    order_id: impl Into<String>,
    status: OrderStatus,
    reason: Option<String>,
) -> HttpResult<StoreOrder> {
    let url = endpoint_url("store/order/update")?;

    let order_id_str = order_id.into();

    tracing::debug!(
        endpoint = %url,
        order_id = %order_id_str,
        status = ?status,
        "updating order"
    );

    #[derive(serde::Serialize)]
    struct UpdateOrderBody {
        order_id: String,
        status: OrderStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
    }

    let body = UpdateOrderBody {
        order_id: order_id_str,
        status,
        reason,
    };

    post_json(token, url, &body).await
}

/// Gets an order by ID.
pub async fn get_order(
    token: &str,
    order_id: impl AsRef<str>,
) -> HttpResult<StoreOrder> {
    let url = endpoint_url("store/order/detail")?;

    tracing::debug!(
        endpoint = %url,
        order_id = order_id.as_ref(),
        "fetching order"
    );

    get_with_query(token, url, &[("order_id", order_id.as_ref())]).await
}

/// Lists orders.
pub async fn list_orders(
    token: &str,
    query: OrderListQuery,
) -> HttpResult<OrderList> {
    let url = endpoint_url("store/order/list")?;

    tracing::debug!(
        endpoint = %url,
        offset = ?query.offset,
        count = ?query.count,
        status = ?query.status,
        "listing orders"
    );

    get_with_query(token, url, &query).await
}
