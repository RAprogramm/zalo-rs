# Zalo Bot

> Tài liệu đa ngôn ngữ cho lớp tích hợp Zalo Official Account (OA) viết bằng Rust.

**Ngôn ngữ khác:** [English](../README.md)

## Mục tiêu dự án

Kho mã này xây dựng lớp tích hợp an toàn, toàn diện cho Zalo Official Account API. Trọng tâm hiện tại là ghi lại đầy đủ các khả
ăng cần thiết, ổn định cấu trúc workspace Rust và cung cấp các khối xây dựng cho bot và mini app tuân thủ chính sách chính thức
của nền tảng.

## Tổng quan Zalo Official Account API

- **Base URL:** `https://openapi.zalo.me/v3.0/oa/`. Hầu hết endpoint sẽ nối thêm đoạn đường dẫn theo từng miền như `message/cs`
  hoặc `tag/gettagsofoa`.
- **Loại tin nhắn:** Customer Service (`cs`), Transaction (`transaction`) và Promotion (`promotion`). Lựa chọn này ảnh hưởng tới
  giới hạn gửi và loại nội dung được phép.
- **Định dạng nội dung:** văn bản thuần, tệp đính kèm (`image`, `file`) và các mẫu (template) dạng danh sách với nút bấm.
- **Header yêu cầu:** mỗi lệnh gọi phải có header `access_token` hợp lệ của OA và `Content-Type` phù hợp (`application/json`
  hoặc `multipart/form-data`).

## Xác thực và vòng đời token

1. Tạo OA trên [Zalo Business](https://business.zalo.me/).
2. Đăng ký ứng dụng tại [Zalo Developers](https://developers.zalo.me/) để lấy App ID, Secret Key, OA ID và Access Token.
3. Lưu trữ và xoay vòng Access Token lẫn Refresh Token. Lỗi `-204` và `-240` báo hiệu token hết hạn hoặc cần nâng cấp lên API v3.
4. Cấu hình webhook HTTPS cho sự kiện đến và xác minh chữ ký MAC cho mọi payload.

## Giới hạn nền tảng

- **Giới hạn tần suất:** hạn ngạch phổ biến cho OA khoảng 10 yêu cầu mỗi giây. Lỗi `-210` nghĩa là đã vượt giới hạn.
- **Cửa sổ tin nhắn 24 giờ:** gửi tin vượt quá 24 giờ kể từ lần tương tác cuối sẽ sinh lỗi `-214`; bot cần lưu dấu thời điểm tương tác mới nhất.
- **Tuân thủ chính sách nội dung:** vi phạm sẽ trả về lỗi `-215`. Cần kiểm tra và kiểm duyệt thông điệp trước khi gửi.

## Các miền API chính

| Miền | Endpoint tiêu biểu | Mục đích |
| --- | --- | --- |
| Nhắn tin | `message/{messageType}` | Gửi tin văn bản, media, danh sách và template. |
| Quản lý người theo dõi | `getoa`, `getprofile`, `getfollowers`, `updatefollowerinfo` | Lấy thông tin OA, danh sách follower và cập nhật metadata. |
| Hội thoại | `listrecentchat`, `conversation` | Lấy danh sách chat và lịch sử hội thoại. |
| Media | `upload/image`, `upload/file`, `upload/gif` | Tải tệp đính kèm để sử dụng sau. |
| Thẻ (tag) | `tag/gettagsofoa`, `tag/tagfollower`, `tag/rmfollowerfromtag` | Quản lý phân khúc người dùng bằng tag. |
| Nội dung OA | `article/create`, `article/upload_video/*`, `article/verify` | Xuất bản bài viết, quản lý video và kiểm duyệt bản nháp. |
| Cửa hàng | `store/product/*`, `store/order/*` | Quản lý sản phẩm và đơn hàng trên OA Store. |
| Webhook | URL tuỳ chỉnh | Nhận sự kiện follow/unfollow, tin nhắn đến, click và xác minh MAC. |

Danh sách nhiệm vụ chi tiết cho phương thức và hợp đồng dữ liệu nằm tại [`docs/progress.md`](progress.md).

## Webhook và sự kiện

Webhook của OA nhận payload POST gồm `app_id`, `sender.id`, `recipient.id`, `event_name`, `timestamp`, `message` và `mac`. Lớp
 tích hợp cần xác minh chữ ký MAC và hỗ trợ các sự kiện sau:

- **Vòng đời người dùng:** `follow`, `unfollow`.
- **Tin nhắn:** `user_send_text`, `user_send_image`, `user_send_file`, `user_send_sticker`, `user_send_gif`, `user_send_location`.
- **Tương tác:** `user_click_link`, `user_click_button`, `user_received_message`, `user_seen_message`.

## Chẩn đoán và xử lý lỗi

Các mã lỗi phổ biến của Zalo OA API:

| Mã | Ý nghĩa |
| --- | --- |
| `-201` | Thiếu tham số bắt buộc. |
| `-202` | Giá trị tham số không hợp lệ. |
| `-204` | Access Token không hợp lệ hoặc đã hết hạn. |
| `-205` | OA không đủ quyền. |
| `-210` | Vượt quá giới hạn tần suất. |
| `-211` | OA không vượt qua bước xác minh. |
| `-213` | Người dùng chưa theo dõi OA. |
| `-214` | Tin gửi ngoài cửa sổ 24 giờ. |
| `-215` | Nội dung vi phạm chính sách. |
| `-216` | Tin nhắn trùng lặp. |
| `-240` | Đang dùng API v2 cũ, cần nâng cấp. |

## Làm việc với kho mã

- Theo dõi phạm vi endpoint và logic liên quan trong [`docs/progress.md`](progress.md).
- Khi xây dựng subsystem, bổ sung ADR và ví dụ có thể chạy để ghi lại quyết định kiến trúc và kịch bản lỗi.
- Trước khi lập trình theo endpoint cụ thể, hãy kiểm tra yêu cầu xác thực, giới hạn tần suất và định dạng payload cho miền tương ứng.

## Cấu trúc workspace Rust

Kho mã được tổ chức thành workspace nhiều crate:

- `crates/zalo-types` — các kiểu dùng chung, bộ nạp cấu hình (`ConfigLoader`) và ánh xạ lỗi dựa trên [`masterror`](https://crates.io/crates/masterror).
- `crates/zalo-sdk` — SDK gọn nhẹ cho Mini App, hỗ trợ kiểm tra ngữ cảnh và sinh handshake payload.
- `crates/zalo-bot` — tiện ích cho OA bot: khởi tạo tracing (`init_tracing`) và xác minh chữ ký webhook (`WebhookVerifier`).
- `examples/miniapp-leptos` — ví dụ Mini App minh hoạ cách dùng SDK.
- `examples/bot-axum` — ví dụ khởi tạo webhook bằng `zalo-bot`.

### Cấu hình

`ConfigLoader` đọc biến môi trường với tiền tố `ZALO_BOT_` và (nếu có) tệp TOML. Các phần được hỗ trợ:

- `environment` — một trong `development`, `staging` hoặc `production`.
- `[logging]` — trường `filter` (biểu thức cho `tracing_subscriber::EnvFilter`) và `format` (`text` hoặc `json`).

### Quy trình kiểm soát chất lượng

Chạy các lệnh sau trước khi gửi thay đổi để đảm bảo định dạng, lint, kiểm thử và tài liệu nhất quán:

```
cargo +nightly fmt --
cargo clippy -D warnings
cargo test --all
cargo doc --no-deps
```

Kiểm tra bảo mật và phụ thuộc sử dụng `cargo audit` và `cargo deny`.

