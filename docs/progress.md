# Покрытие Zalo Official Account API

Документ фиксирует список REST-методов, структур данных и нефункциональных требований, которые необходимо реализовать для полного покрытия возможностей Zalo Official Account API версий v3.0. Каждый пункт помечается по мере появления рабочего кода и тестов.

## Общие задачи

- [ ] Регистрация OA, выпуск App ID, Secret Key, OA ID и настройка защищённого хранения учётных данных.
- [ ] Клиент для обмена Access/Refresh Token'ами и автоматического обновления по истечении срока действия.
- [ ] Подсистема rate limiting и ретраев с учётом лимита в 10 запросов/с на OA.
- [ ] Единая обработка ошибок с маппингом кодов `-201`, `-202`, `-204`, `-205`, `-210`, `-211`, `-213`, `-214`, `-215`, `-216`, `-240`.

## Модель данных

- [ ] `AccessTokenInfo` — токен, refresh token, время истечения.
- [ ] `MessageRecipient` (`user_id`).
- [ ] `MessageContent` — текст и вложения (`image`, `file`, `template`).
- [ ] `AttachmentPayload` — token или URL, размеры изображения.
- [ ] `FollowerProfile` — идентификатор, имя, телефон, email, адрес, город, дата рождения.
- [ ] `ConversationSummary` и `ConversationMessage` для списка чатов и истории переписки.
- [ ] `TagInfo` и `FollowerTagAssignment`.
- [ ] `ArticleDraft`, `ArticleVideoUpload`, `ArticleVerification`.
- [ ] `StoreProduct`, `StoreOrder`, `OrderItem`, `ShippingInfo`.
- [ ] `WebhookEvent` — поля `app_id`, `sender`, `recipient`, `event_name`, `timestamp`, `message`, `mac`.

## Messaging API (`https://openapi.zalo.me/v3.0/oa/message/{messageType}`)

- [ ] Отправка текстового сообщения (`message.text`).
- [ ] Отправка изображения (attachment type `image` + `payload.token` или `payload.url`).
- [ ] Отправка файла (attachment type `file`).
- [ ] Отправка списочного шаблона (`attachment.type = template`, `template_type = list`, `elements`, `buttons`).
- [ ] Поддержка типов сообщений `cs`, `transaction`, `promotion` и правил 24-часового окна.

## Управление подписчиками

- [ ] `GET /v3.0/oa/getoa` — базовая информация OA.
- [ ] `GET /v3.0/oa/getprofile` — профиль пользователя по `user_id`.
- [ ] `GET /v3.0/oa/getfollowers` — постраничный список подписчиков (`offset`, `count`).
- [ ] `POST /v3.0/oa/updatefollowerinfo` — обновление полей `name`, `phone`, `email`, `address`, `city`, `birthday`.

## Диалоги

- [ ] `GET /v3.0/oa/listrecentchat` — получение последних диалогов (`offset`, `count`).
- [ ] `GET /v3.0/oa/conversation` — история сообщений по `user_id`.

## Управление медиа

- [ ] `POST /v3.0/oa/upload/image` — multipart upload или `image_url`.
- [ ] `POST /v3.0/oa/upload/file` — multipart upload или `file_url`.
- [ ] `POST /v3.0/oa/upload/gif` — multipart upload или `gif_url`.

## Управление тегами

- [ ] `GET /v3.0/oa/tag/gettagsofoa` — постраничный список тегов.
- [ ] `POST /v3.0/oa/tag/tagfollower` — привязка тега (`user_id`, `tag_id`).
- [ ] `POST /v3.0/oa/tag/rmfollowerfromtag` — удаление тега у пользователя.

## Контент OA

- [ ] `POST /v3.0/article/create` — создание статьи (title, description, author, cover, body, status, comment).
- [ ] `POST /v3.0/article/upload_video/preparevideo` — подготовка загрузки видео (`video_name`, `video_size`).
- [ ] `POST /v3.0/article/upload_video/verify` — подтверждение видео (`upload_id`).
- [ ] `GET /v3.0/article/verify` — получение деталей статьи (`token`).

## Магазин OA Store

- [ ] `POST /v3.0/store/product/create` — создание товара (name, price, description, code, photos, status).
- [ ] `POST /v3.0/store/product/update` — обновление товара.
- [ ] `POST /v3.0/store/order/create` — создание заказа (user_id, shipping.*, items, shipping_fee, discount, total).
- [ ] `POST /v3.0/store/order/update` — обновление статуса заказа (`status`, `reason`).
- [ ] `GET /v3.0/store/order/getorder` — получение заказа по `id`.
- [ ] `GET /v3.0/store/order/getorderofoa` — список заказов (`offset`, `count`, `status`).

## Вебхуки и события

- [ ] HTTPS webhook endpoint с проверкой подписи MAC.
- [ ] Обработка событий `follow`, `unfollow`.
- [ ] Обработка событий сообщений `user_send_text`, `user_send_image`, `user_send_file`, `user_send_sticker`, `user_send_gif`, `user_send_location`.
- [ ] Обработка событий взаимодействия `user_click_link`, `user_click_button`, `user_received_message`, `user_seen_message`.
- [ ] Квитирование webhook'ов и backoff при временных ошибках.

## Тестирование и наблюдаемость

- [ ] Модульные тесты для всех публичных API-клиентов и десериализации ответов.
- [ ] Интеграционные тесты против sandbox/OA mock'ов.
- [ ] Трассировка (`tracing`) без утечек секретов.
- [ ] Метрики по rate limit, ошибкам и времени ответа.
