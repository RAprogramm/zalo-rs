# Storage API

Кэш ключ-значение для Mini App.

**Реализация:** [`crates/zalo-sdk/src/storage.rs`](../../crates/zalo-sdk/src/storage.rs)

---

## setItem / getItem

```rust
use zalo_sdk::storage::{SetStorageRequest, GetStorageRequest, StorageKey, StorageValue};

// Сохранение
let key = StorageKey::new("user_prefs")?;
let value = StorageValue::new("dark_mode")?;
let request = SetStorageRequest::new(key, value);

// Чтение
let request = GetStorageRequest::new(StorageKey::new("user_prefs")?);
let response = get_storage(request)?;
```

**Ограничения:**
- Ключ: макс. 128 байт
- Значение: макс. 4096 байт

**Структуры:** [`storage.rs`](../../crates/zalo-sdk/src/storage.rs#L15-L70)

---

[← SDK](README.md) | [Payment →](payment/README.md)
