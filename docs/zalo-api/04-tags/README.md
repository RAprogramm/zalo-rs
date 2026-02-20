# Теги

Управление тегами подписчиков.

**Реализация:** В разработке (см. [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md))

---

## Методы

| Метод | Файл | Описание |
|-------|------|----------|
| **List** | [list/README.md](list/README.md) | Список тегов |
| **Operations** | [operations/README.md](operations/README.md) | Добавить/удалить |

---

## Структуры

```rust
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub follower_count: u64,
}
```

---

[← Users](../03-users/README.md) | [List →](list/README.md)
