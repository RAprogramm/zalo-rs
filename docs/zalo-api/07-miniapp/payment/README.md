# Payment API

Оплата через Zalo Pay.

**Реализация:** [`crates/zalo-sdk/src/payment.rs`](../../crates/zalo-sdk/src/payment.rs)

---

## checkout

```rust
use zalo_sdk::payment::{CheckoutRequest, Amount, OrderId};

let request = CheckoutRequest::new()
    .order_id(OrderId::new("ORDER_123")?)
    .amount(Amount::new(500000))  // VND
    .description("Оплата заказа");

let response = checkout(request)?;
```

**Структуры:** [`payment.rs`](../../crates/zalo-sdk/src/payment.rs#L15-L60)

---

[← SDK](README.md) | [Navigation →](navigation/README.md)
