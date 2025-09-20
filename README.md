# Zalo Bot

> Production-ready, multilingual documentation for the Rust integration with the Zalo Official Account (OA) platform.

**Translations:** [Русский](docs/README.ru.md) · [Tiếng Việt](docs/README.vi.md)

## Project goals

This repository delivers a safe, end-to-end integration layer for the Zalo Official Account API. The current focus is to document
all required capabilities, stabilise the Rust workspace layout, and provide building blocks for bots and mini apps that comply
with the official platform policies.

## Zalo Official Account API overview

- **Base URL:** `https://openapi.zalo.me/v3.0/oa/`. Most endpoints extend this prefix with a domain-specific segment such as
  `message/cs` or `tag/gettagsofoa`.
- **Message types:** Customer Service (`cs`), Transaction (`transaction`), and Promotion (`promotion`). The chosen type affects
  delivery constraints and permissible content.
- **Content formats:** plain text, attachments (`image`, `file`), and list-based templates with buttons.
- **Request headers:** each call must provide a valid OA `access_token` header and the appropriate `Content-Type` (`application/json`
  or `multipart/form-data`).

## Authentication and token lifecycle

1. Create an OA at [Zalo Business](https://business.zalo.me/).
2. Register an app on [Zalo Developers](https://developers.zalo.me/) to obtain the App ID, Secret Key, OA ID, and Access Token.
3. Store and rotate both the Access Token and Refresh Token. Errors `-204` and `-240` indicate an expired token or a required
   migration to API v3.
4. Configure HTTPS webhooks for inbound events and verify the MAC signature for every payload.

## Platform constraints

- **Rate limiting:** typical OA quotas allow roughly 10 requests per second. Error `-210` indicates the limit was exceeded.
- **24-hour messaging window:** sending a message beyond the 24-hour window triggers error `-214`; bots must track the latest
  user interaction timestamp.
- **Content policy enforcement:** violations result in error `-215`. Implement validation and moderation before dispatching
  messages.

## Key API domains

| Domain | Representative endpoints | Purpose |
| --- | --- | --- |
| Messaging | `message/{messageType}` | Text, media, list, and template message delivery. |
| Subscriber management | `getoa`, `getprofile`, `getfollowers`, `updatefollowerinfo` | Access OA profiles, follower lists, and update metadata. |
| Conversations | `listrecentchat`, `conversation` | Retrieve chat lists and conversation history. |
| Media | `upload/image`, `upload/file`, `upload/gif` | Upload attachments for later use. |
| Tags | `tag/gettagsofoa`, `tag/tagfollower`, `tag/rmfollowerfromtag` | Manage audience segmentation through tags. |
| OA content | `article/create`, `article/upload_video/*`, `article/verify` | Publish articles, manage videos, and verify drafts. |
| Store | `store/product/*`, `store/order/*` | Maintain OA Store products and orders. |
| Webhook | Custom URL | Receive follow/unfollow events, inbound messages, clicks, and MAC verification callbacks. |

A detailed checklist of methods and data contracts lives in [`docs/progress.md`](docs/progress.md).

## Webhooks and events

OA webhooks receive POST payloads containing `app_id`, `sender.id`, `recipient.id`, `event_name`, `timestamp`, `message`, and `mac`.
The integration must verify the MAC signature and support the following events:

- **User lifecycle:** `follow`, `unfollow`.
- **Messaging:** `user_send_text`, `user_send_image`, `user_send_file`, `user_send_sticker`, `user_send_gif`, `user_send_location`.
- **Interactions:** `user_click_link`, `user_click_button`, `user_received_message`, `user_seen_message`.

## Diagnostics and error handling

Common Zalo OA API error codes:

| Code | Meaning |
| --- | --- |
| `-201` | Required parameters are missing. |
| `-202` | Parameter values are invalid. |
| `-204` | Access Token is invalid or expired. |
| `-205` | OA lacks sufficient permissions. |
| `-210` | Rate limit exceeded. |
| `-211` | OA failed verification. |
| `-213` | User is not subscribed to the OA. |
| `-214` | Message sent outside the 24-hour window. |
| `-215` | Content violates platform policies. |
| `-216` | Duplicate message detected. |
| `-240` | Legacy API v2 is in use and must be upgraded. |

## How to work with this repository

- Track endpoint coverage and required business logic in [`docs/progress.md`](docs/progress.md).
- Contribute ADRs and runnable examples alongside subsystem implementations to document architectural decisions and failure modes.
- Before coding against a specific endpoint, review authentication requirements, rate limits, and payload formats for the
  corresponding domain.

## Rust workspace structure

This repository is organised as a multi-crate workspace:

- `crates/zalo-types` — shared types, configuration loader (`ConfigLoader`), and error mapping built on [`masterror`](https://crates.io/crates/masterror).
- `crates/zalo-sdk` — lightweight Mini App SDK providing context validation and handshake payload generation.
- `crates/zalo-bot` — OA bot utilities: tracing initialisation (`init_tracing`) and webhook signature verification (`WebhookVerifier`).
- `examples/miniapp-leptos` — sample Mini App that demonstrates SDK usage.
- `examples/bot-axum` — webhook bootstrap example powered by `zalo-bot`.

### Configuration

`ConfigLoader` reads environment variables prefixed with `ZALO_BOT_` and an optional TOML file. Supported sections:

- `environment` — one of `development`, `staging`, or `production`.
- `[logging]` — fields `filter` (expression for `tracing_subscriber::EnvFilter`) and `format` (`text` or `json`).

### Quality gates

Run the following commands before submitting changes to guarantee consistent formatting, linting, tests, and documentation:

```
cargo +nightly fmt --
cargo clippy -D warnings
cargo test --all
cargo doc --no-deps
```

Security and dependency hygiene checks use `cargo audit` and `cargo deny`.

