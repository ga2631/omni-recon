# 🏛️ Thiết kế Kiến trúc Hệ thống OmniRecon

Tài liệu này mô tả chi tiết kiến trúc kỹ thuật của **OmniRecon** — nền tảng đối soát và cảnh báo dòng tiền cho nhà bán hàng đa kênh.

---

## 1. Mục tiêu Thiết kế (Design Principles)

1. **Hiệu năng Cực cao (High Throughput & Low Latency)**: Xử lý hàng triệu bản ghi sao kê từ nhiều sàn TMĐT trong vài giây bằng công cụ OLAP nhúng **DuckDB** kết hợp ngôn ngữ **Rust**.
2. **Khả năng Tự lưu trữ & Độc lập (Self-hosted & Zero-vendor Lock-in)**: Toàn bộ hệ thống được đóng gói thành các Docker containers gọn nhẹ, không phụ thuộc vào dịch vụ đám mây độc quyền.
3. **Mô-đun hóa Đa kênh (Pluggable Channel Adapters)**: Dễ dàng mở rộng và bổ sung parser cho các kênh bán mới (Shopee, TikTok Shop, Lazada, Tiki, Shopify, WooCommerce) và đơn vị vận chuyển (GHN, GHTK, Viettel Post, J&T).
4. **Cảnh báo Thời gian thực (Real-time Anomaly Detection)**: Tự động phát hiện các bất thường về phí sàn, chênh lệch tiền thu hộ COD và độ trễ dòng tiền, gửi thông báo qua Webhook/Telegram/Slack/Lark.

---

## 2. Sơ đồ Kiến trúc Tổng thể (High-Level Architecture)

```mermaid
graph TD
    User["👤 Nhà Bán Hàng / Kế toán viên"] -->|HTTP / HTTPS| Nginx["🌐 Nginx Gateway (Port 80/443)"]
    
    subgraph FrontendApp["Frontend (Vue 3 + Vite + Tailwind CSS)"]
        Nginx -->|/ (Static Assets)| SPA["Single Page Application"]
        SPA --> Store["Pinia Stores (Recon, Filters, Alerts)"]
        SPA --> Charts["ECharts / Chart.js Analytics"]
    end

    subgraph BackendCluster["Backend API & Task Coordinator (Rust Axum)"]
        Nginx -->|/api/v1/*| API["Axum REST API Server"]
        Nginx -->|/ws| WSS["WebSocket Server (Live Progress)"]
        API --> Auth["JWT & RBAC Middleware"]
        API --> ChannelService["Channel & Config Manager"]
        API --> UploadService["Statement File Ingestion"]
    end

    subgraph BackgroundWorkers["Background Job Workers (Rust)"]
        Worker["Redis Job Consumer Worker"]
    end

    subgraph AnalyticalEngine["ETL & Reconciliation Engine (Rust + DuckDB)"]
        Worker --> Engine["DuckDB Reconciliation Engine"]
        Engine --> Calamine["High-Speed Excel/CSV Parser"]
        Engine --> ParquetStore["Parquet Columnar Storage (data/storage)"]
        Engine --> RuleMatcher["Multi-way Discrepancy Matcher"]
    end

    subgraph DataStorage["Data Persistence"]
        API <--> Postgres[("🐘 PostgreSQL 16\n(Users, Channels, Rules, Jobs)")]
        API <--> RedisQueue[("⚡ Redis 7\n(Task Queue, PubSub, Cache)")]
        Worker <--> RedisQueue
        Worker --> Postgres
    end

    subgraph NotificationHub["Notification Dispatcher"]
        Worker --> Notifier["Alert Dispatcher"]
        Notifier --> Telegram["✈️ Telegram Bot"]
        Notifier --> Slack["💬 Slack Webhook"]
        Notifier --> Lark["🐦 Lark / Feishu"]
    end
```

---

## 3. Các Thành phần Hệ thống

### 3.1. Frontend (`frontend/`)
- **Công nghệ**: Vue 3 (Composition API, `<script setup>`), Vite 5, Tailwind CSS 3, Pinia, Vue Router 4, Lucide Icons.
- **Tính năng**:
  - Dashboard tổng quan dòng tiền: Doanh thu gộp, doanh thu thực nhận, tổng phí sàn, tiền COD đang giữ, chênh lệch cần khiếu nại.
  - Bộ lọc tương tác đa chiều (Reactive Filters): Lọc theo khoảng ngày, theo gian hàng (shop), theo sàn (Shopee/Lazada/TikTok), theo trạng thái đơn (Đã khớp, Lệch phí, Lệch COD, Chưa đối soát).
  - Tải lên bảng kê trực quan với tiến trình xử lý real-time qua WebSocket.
  - Quản lý cảnh báo và xuất báo cáo khiếu nại Excel/CSV.

### 3.2. Backend API Server (`backend/server/`)
- **Công nghệ**: Rust (Axum web framework), Tokio runtime, Tower-HTTP (CORS, Compression, Tracing), SQLx (kết nối PostgreSQL an toàn kiểu tĩnh), Jsonwebtoken.
- **Vai trò**:
  - Cung cấp RESTful API và WebSocket endpoint.
  - Quản lý xác thực người dùng, gian hàng (channels), lịch sử tải file và cấu hình cảnh báo.
  - Đẩy các tác vụ đối soát nặng vào hàng đợi Redis để worker xử lý bất đồng bộ.

### 3.3. ETL & Reconciliation Engine (`backend/engine/`)
- **Công nghệ**: DuckDB nhúng trong Rust (`duckdb-rs`), thư viện đọc Excel siêu tốc `calamine`, định dạng dữ liệu dạng cột `Apache Parquet`.
- **Vai trò**:
  - Phân tích cú pháp các file bảng kê từ Shopee, Lazada, TikTok, GHN, GHTK...
  - Chuyển đổi dữ liệu thô sang cấu trúc chuẩn hóa (Normalized Parquet Schema).
  - Thực thi các truy vấn SQL OLAP phức tạp với DuckDB để so khớp 3 chiều:
    1. Đơn hàng nội bộ (OMS) vs Sao kê sàn TMĐT.
    2. Đơn hàng vận chuyển (3PL) vs Tiền thu hộ COD thực nhận.
    3. Phiên thanh toán sàn vs Sao kê biến động số dư ngân hàng.

### 3.4. Background Worker (`backend/worker/`)
- **Công nghệ**: Rust, Redis (Reliable Queue / Stream), Cron scheduler.
- **Vai trò**:
  - Tiêu thụ (consume) các tác vụ đối soát từ Redis.
  - Gọi DuckDB engine xử lý dữ liệu theo lô (batch processing).
  - Cập nhật tiến độ qua Redis Pub/Sub đến WebSocket server.
  - Ghi nhận các bản ghi bất thường (discrepancy items) vào PostgreSQL và kích hoạt cảnh báo.

### 3.5. Cơ sở dữ liệu & Hạ tầng Lưu trữ
- **PostgreSQL 16**: Lưu trữ quan hệ (Users, Organization, Channels, Statement Batches, Discrepancy Alerts, Alert Rules).
- **Redis 7**: Hàng đợi tác vụ (Job Queue), Pub/Sub cho WebSocket và bộ nhớ đệm (Cache).
- **DuckDB & Local Parquet Storage**: Phân tích dữ liệu giao dịch lớn tốc độ cao không làm nghẽn cơ sở dữ liệu chính.
- **Nginx**: Reverse Proxy đóng vai trò cổng vào duy nhất (Port 80/443), phân phối yêu cầu tĩnh tới Frontend và API tới Backend.
