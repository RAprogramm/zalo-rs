# Архитектура Rust-платформы

## Цели

- Единый фундамент для сервисов и интеграций бота.
- Предсказуемые ошибки и единая система логирования.
- Простая конфигурация через переменные окружения и TOML.

## Структура workspace

- `crates/core`
  - `config`: модель `AppConfig`, загрузчик `ConfigLoader`, валидация и значения по умолчанию.
  - `error`: доменные ошибки (`CoreError`, `ConfigError`, `ObservabilityError`) поверх `masterror::AppError`.
  - `observability`: построение `tracing`-subscriber (`build_tracing_dispatch`) и установка глобального логирования (`init_tracing`).
- `crates/app`
  - Точка входа `main`, использующая `ConfigLoader` и `init_tracing` для запуска.

## Обработка ошибок

- Весь пользовательский код работает через `CoreResult<T> = AppResult<T, CoreError>`.
- `CoreError` маппится в `masterror::AppError`, что гарантирует стабильные категории (`AppErrorKind`).
- Внешние ошибки (`figment`, `tracing_subscriber`) оборачиваются в доменные типы и не прячутся.

## Конфигурация

- Значения по умолчанию безопасны: `environment = development`, `logging.filter = "info"`, `logging.format = "text"`.
- Источники: дефолты → TOML-файл (если указан) → переменные окружения `ZALO_BOT_*` с разделителем `__` для вложенных полей.

## Логирование

- Используем `tracing` + `tracing-subscriber`.
- Поддерживаем текстовый и JSON-формат; JSON включает feature `json` у `tracing-subscriber`.
- `init_tracing` устанавливает глобального подписчика один раз, повторные вызовы возвращают `ObservabilityError::Install`.

## Дальнейшее развитие

- Добавить интеграции (БД, HTTP-клиенты) как отдельные модули/крэйты.
- Описывать ключевые решения в `docs/ARCH.md` с обновлением по мере роста функциональности.
- Расширить тестирование: property-based и fuzz для парсеров, интеграционные тесты для сетевых модулей.
