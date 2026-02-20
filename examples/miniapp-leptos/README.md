# Zalo Mini App Example (Leptos 0.8)

Пример Mini App для Zalo на базе Leptos 0.8 с использованием signals.

## Быстрый старт

### 1. Установка trunk

```bash
cargo install trunk
```

### 2. Запуск

```bash
cd examples/miniapp-leptos
trunk serve --open
```

### 3. Сборка для production

```bash
trunk build --release
```

## Структура проекта

```
miniapp-leptos/
├── Cargo.toml          # Зависимости
├── style.css           # Стили
├── index.html          # HTML шаблон (создаётся trunk)
└── src/
    └── lib.rs          # Основной код приложения
```

## Особенности Leptos 0.8

### Signals

```rust
use leptos::prelude::*;

// Создаём сигнал
let count = create_rw_signal(0);

// Чтение
let value = count.get();

// Запись
count.set(10);
*count.write() += 1;

// Derived signal
let doubled = Signal::derive(move || count() * 2);
```

### Callback

```rust
let on_click = Callback::new(move |ev: MouseEvent| {
    count.update(|v| *v += 1);
});

view! {
    <button on:click=on_click>"Click"</button>
}
```

### Event handling

```rust
use leptos::event_target_value;

let on_input = move |ev: Event| {
    let value = event_target_value(&ev);
    text.set(value);
};

view! {
    <input
        type="text"
        value=move || text.get()
        on:input=on_input
    />
}
```

## Интеграция с Zalo SDK

### Авторизация

```rust
use zalo_sdk::auth::AuthorizeRequest;

let request = AuthorizeRequest::new(app_id)
    .with_scopes(vec!["user_info", "phone_number"]);
```

### Получение информации о пользователе

```rust
use zalo_sdk::user::get_user_info;

let user_info = get_user_info()?;
```

### Storage

```rust
use zalo_sdk::storage::{SetStorageRequest, StorageKey, StorageValue};

let key = StorageKey::new("user_prefs")?;
let value = StorageValue::new("dark_mode")?;
```

### Share

```rust
use zalo_sdk::share::ShareRequest;

let request = ShareRequest::new()
    .title("Check this out!")
    .thumbnail("https://...");
```

## Конфигурация

Отредактируйте `src/lib.rs`:

```rust
const APP_ID: &str = "your-app-id";
const OA_ID: &str = "your-oa-id";
```

## Deployment

### 1. Сборка

```bash
trunk build --release
```

### 2. Загрузка на хостинг

Содержимое `dist/` папки загрузите на любой статический хостинг:
- GitHub Pages
- Vercel
- Netlify
- Zalo Mini App hosting

### 3. Регистрация в Zalo

1. Откройте [Zalo Developers](https://developers.zalo.me/)
2. Создайте новое Mini App
3. Укажите URL вашего приложения
4. Настройте OAuth redirect URI

## Лицензия

MIT
