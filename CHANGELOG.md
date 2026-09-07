# Changelog

Toàn bộ các thay đổi quan trọng của dự án **OmniRecon** sẽ được ghi lại trong tệp này.

Định dạng dựa trên [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
và dự án tuân thủ [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Khởi tạo kiến trúc Monorepo (`backend`, `frontend`, `deploy`, `docs`).
- Tích hợp giấy phép AGPL-3.0 và điều khoản bản quyền `NOTICE`.
- Cấu hình triển khai Docker Compose thống nhất (PostgreSQL, Redis, Rust Axum, Vue 3, Nginx).
- Khung sườn Backend Rust với Cargo Workspace (`common`, `engine`, `server`, `worker`).
- Khung sườn Frontend Vue 3 + Vite + Tailwind CSS + Pinia Dashboard.
- Tài liệu kiến trúc và quy chuẩn định dạng bảng kê đa kênh.
