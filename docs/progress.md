# Покрытие Zalo Official Account API

Документ фиксирует список REST-методов, структур данных и нефункциональных требований, которые необходимо реализовать для полного покрытия возможностей Zalo Official Account API версий v3.0. Каждый пункт помечается по мере появления рабочего кода и тестов.

## Общие задачи

- [ ] Регистрация OA, выпуск App ID, Secret Key, OA ID и настройка защищённого хранения учётных данных.
- [x] Клиент для обмена Access/Refresh Token'ами и автоматического обновления по истечении срока действия.
  - **Реализация:** [`crates/zalo-http/src/oauth.rs`](../crates/zalo-http/src/oauth.rs) — `OAuthClient::get_access_token()`, `OAuthClient::refresh_token()`
  - **Реализация:** [`crates/zalo-http/src/client/token/`](../crates/zalo-http/src/client/token/) — `TokenManager`, `AccessTokenInfo`, `SecureToken`
- [ ] Подсистема rate limiting и ретраев с учётом лимита в 10 запросов/с на OA.
  - **Частично:** [`crates/zalo-http/src/rate_limiter/`](../crates/zalo-http/src/rate_limiter/) — `RateLimiter` (требует завершения)
- [x] Единая обработка ошибок с маппингом кодов `-201`, `-202`, `-204`, `-205`, `-210`, `-211`, `-213`, `-214`, `-215`, `-216`, `-240`.
  - **Реализация:** [`crates/zalo-http/src/error.rs`](../crates/zalo-http/src/error.rs) — `HttpError::from_api_response()`

## Модель данных

- [x] `AccessTokenInfo` — токен, refresh token, время истечения.
  - **Реализация:** [`crates/zalo-http/src/client/token/info.rs`](../crates/zalo-http/src/client/token/info.rs)
- [x] `MessageRecipient` (`user_id`).
  - **Реализация:** [`crates/zalo-types/src/message.rs`](../crates/zalo-types/src/message.rs) — `Recipient`
- [ ] `MessageContent` — текст и вложения (`image`, `file`, `template`).
  - **Частично:** [`crates/zalo-types/src/message.rs`](../crates/zalo-types/src/message.rs) — `TextPayload`
  - **Частично:** [`crates/zalo-types/src/image.rs`](../crates/zalo-types/src/image.rs) — `ImagePayload`
- [ ] `AttachmentPayload` — token или URL, размеры изображения.
- [ ] `FollowerProfile` — идентификатор, имя, телефон, email, адрес, город, дата рождения.
  - **Частично:** [`crates/zalo-types/src/user.rs`](../crates/zalo-types/src/user.rs) — `UserProfile` (базовые поля)
- [x] `ConversationSummary` и `ConversationMessage` для списка чатов и истории переписки.
  - **Реализация:** [`crates/zalo-types/src/conversation.rs`](../crates/zalo-types/src/conversation.rs)
- [x] `TagInfo` и `FollowerTagAssignment`.
  - **Реализация:** [`crates/zalo-types/src/tag.rs`](../crates/zalo-types/src/tag.rs) — `TagInfo`, `TagFollowerRequest`
- [x] `ArticleDraft`, `ArticleVideoUpload`, `ArticleVerification`.
  - **Реализация:** [`crates/zalo-types/src/article.rs`](../crates/zalo-types/src/article.rs)
- [x] `StoreProduct`, `StoreOrder`, `OrderItem`, `ShippingInfo`.
  - **Реализация:** [`crates/zalo-types/src/store.rs`](../crates/zalo-types/src/store.rs)
- [x] `WebhookEvent` — поля `app_id`, `sender`, `recipient`, `event_name`, `timestamp`, `message`, `mac`.
  - **Реализация:** [`crates/zalo-types/src/webhook.rs`](../crates/zalo-types/src/webhook.rs)

## Messaging API (`https://openapi.zalo.me/v3.0/oa/message/{messageType}`)

- [x] Отправка текстового сообщения (`message.text`).
  - **Реализация:** [`crates/zalo-http/src/client_inner/client.rs`](../crates/zalo-http/src/client_inner/client.rs) — `OaClient::send_text_message()`
- [ ] Отправка изображения (attachment type `image` + `payload.token` или `payload.url`).
  - **Типы готовы:** [`crates/zalo-types/src/image.rs`](../crates/zalo-types/src/image.rs) — `SendImageRequest`
- [ ] Отправка файла (attachment type `file`).
- [ ] Отправка списочного шаблона (`attachment.type = template`, `template_type = list`, `elements`, `buttons`).
- [ ] Поддержка типов сообщений `cs`, `transaction`, `promotion` и правил 24-часового окна.
  - **Типы готовы:** [`crates/zalo-types/src/message.rs`](../crates/zalo-types/src/message.rs) — `MessageType`

## Управление подписчиками

- [ ] `GET /v3.0/oa/getoa` — базовая информация OA.
- [x] `GET /v3.0/oa/getprofile` — профиль пользователя по `user_id`.
  - **Реализация:** [`crates/zalo-http/src/client_inner/client.rs`](../crates/zalo-http/src/client_inner/client.rs) — `OaClient::get_user_profile()`
- [x] `GET /v3.0/oa/getfollowers` — постраничный список подписчиков (`offset`, `count`).
  - **Реализация:** [`crates/zalo-http/src/client_inner/client.rs`](../crates/zalo-http/src/client_inner/client.rs) — `OaClient::list_followers()`
- [ ] `POST /v3.0/oa/updatefollowerinfo` — обновление полей `name`, `phone`, `email`, `address`, `city`, `birthday`.

## Диалоги

- [ ] `GET /v3.0/oa/listrecentchat` — получение последних диалогов (`offset`, `count`).
  - **Типы готовы:** [`crates/zalo-types/src/conversation.rs`](../crates/zalo-types/src/conversation.rs) — `RecentChatQuery`, `RecentChatList`
- [ ] `GET /v3.0/oa/conversation` — история сообщений по `user_id`.
  - **Типы готовы:** [`crates/zalo-types/src/conversation.rs`](../crates/zalo-types/src/conversation.rs) — `ConversationQuery`, `ConversationHistory`

## Управление медиа

- [ ] `POST /v3.0/oa/upload/image` — multipart upload или `image_url`.
  - **Типы готовы:** [`crates/zalo-types/src/media.rs`](../crates/zalo-types/src/media.rs) — `MediaUploadResponse`
- [ ] `POST /v3.0/oa/upload/file` — multipart upload или `file_url`.
- [ ] `POST /v3.0/oa/upload/gif` — multipart upload или `gif_url`.

## Управление тегами

- [ ] `GET /v3.0/oa/tag/gettagsofoa` — постраничный список тегов.
  - **Типы готовы:** [`crates/zalo-types/src/tag.rs`](../crates/zalo-types/src/tag.rs) — `TagListQuery`, `TagList`
- [ ] `POST /v3.0/oa/tag/tagfollower` — привязка тега (`user_id`, `tag_id`).
  - **Типы готовы:** [`crates/zalo-types/src/tag.rs`](../crates/zalo-types/src/tag.rs) — `TagFollowerRequest`, `TagOperationResponse`
- [ ] `POST /v3.0/oa/tag/rmfollowerfromtag` — удаление тега у пользователя.

## Контент OA

- [ ] `POST /v3.0/article/create` — создание статьи (title, description, author, cover, body, status, comment).
  - **Типы готовы:** [`crates/zalo-types/src/article.rs`](../crates/zalo-types/src/article.rs) — `CreateArticleRequest`
- [ ] `POST /v3.0/article/upload_video/preparevideo` — подготовка загрузки видео (`video_name`, `video_size`).
  - **Типы готовы:** [`crates/zalo-types/src/article.rs`](../crates/zalo-types/src/article.rs) — `VideoUploadPrepareRequest`, `VideoUploadPrepareResponse`
- [ ] `POST /v3.0/article/upload_video/verify` — подтверждение видео (`upload_id`).
  - **Типы готовы:** [`crates/zalo-types/src/article.rs`](../crates/zalo-types/src/article.rs) — `VideoUploadVerifyRequest`
- [ ] `GET /v3.0/article/verify` — получение деталей статьи (`token`).
  - **Типы готовы:** [`crates/zalo-types/src/article.rs`](../crates/zalo-types/src/article.rs) — `ArticleVerification`, `ArticleVerificationQuery`

## Магазин OA Store

- [ ] `POST /v3.0/store/product/create` — создание товара (name, price, description, code, photos, status).
  - **Типы готовы:** [`crates/zalo-types/src/store.rs`](../crates/zalo-types/src/store.rs) — `CreateProductRequest`, `StoreProduct`
- [ ] `POST /v3.0/store/product/update` — обновление товара.
- [ ] `POST /v3.0/store/order/create` — создание заказа (user_id, shipping.*, items, shipping_fee, discount, total).
  - **Типы готовы:** [`crates/zalo-types/src/store.rs`](../crates/zalo-types/src/store.rs) — `CreateOrderRequest`, `StoreOrder`
- [ ] `POST /v3.0/store/order/update` — обновление статуса заказа (`status`, `reason`).
- [ ] `GET /v3.0/store/order/getorder` — получение заказа по `id`.
- [ ] `GET /v3.0/store/order/getorderofoa` — список заказов (`offset`, `count`, `status`).
  - **Типы готовы:** [`crates/zalo-types/src/store.rs`](../crates/zalo-types/src/store.rs) — `OrderListQuery`, `OrderList`

## Вебхуки и события

- [x] HTTPS webhook endpoint с проверкой подписи MAC.
  - **Реализация:** [`crates/zalo-bot/src/webhook.rs`](../crates/zalo-bot/src/webhook.rs) — `WebhookVerifier::verify()`
- [ ] Обработка событий `follow`, `unfollow`.
- [ ] Обработка событий сообщений `user_send_text`, `user_send_image`, `user_send_file`, `user_send_sticker`, `user_send_gif`, `user_send_location`.
  - **Типы готовы:** [`crates/zalo-types/src/webhook.rs`](../crates/zalo-types/src/webhook.rs) — `WebhookEventType`
- [ ] Обработка событий взаимодействия `user_click_link`, `user_click_button`, `user_received_message`, `user_seen_message`.
- [ ] Квитирование webhook'ов и backoff при временных ошибках.

## Тестирование и наблюдаемость

- [x] Модульные тесты для всех публичных API-клиентов и десериализации ответов.
  - **Реализация:** [`crates/zalo-http/src/error.rs`](../crates/zalo-http/src/error.rs#L110-L180) — тесты `HttpError`
- [ ] Интеграционные тесты против sandbox/OA mock'ов.
- [x] Трассировка (`tracing`) без утечек секретов.
  - **Реализация:** [`crates/zalo-bot/src/observability.rs`](../crates/zalo-bot/src/observability.rs) — `init_tracing()`, `build_tracing_dispatch()`
- [ ] Метрики по rate limit, ошибкам и времени ответа.
