# 🔌 Đặc tả REST & WebSocket API (API Specification)

API của OmniRecon được xây dựng trên nền tảng **Rust Axum**, cung cấp các endpoint hiệu năng cao, xác thực an toàn bằng JWT.

- **Base URL (Local)**: `http://localhost/api/v1`
- **WebSocket URL**: `ws://localhost/ws`
- **Content-Type**: `application/json` (hoặc `multipart/form-data` khi tải file)

---

## 1. Xác thực & Phân quyền (Authentication)

| Method | Endpoint | Mô tả |
| ------ | -------- | ----- |
| `POST` | `/api/v1/auth/login` | Đăng nhập bằng Email & Mật khẩu, trả về JWT Access Token |
| `POST` | `/api/v1/auth/refresh`| Làm mới Access Token |
| `GET`  | `/api/v1/auth/me` | Lấy thông tin tài khoản hiện tại |

---

## 2. Quản lý Gian hàng & Kênh bán (Channels)

| Method | Endpoint | Mô tả |
| ------ | -------- | ----- |
| `GET`  | `/api/v1/channels` | Danh sách tất cả các shop / kênh TMĐT / ĐVVC đã kết nối |
| `POST` | `/api/v1/channels` | Thêm mới một gian hàng / cấu hình kết nối |
| `PUT`  | `/api/v1/channels/:id` | Cập nhật thông tin gian hàng & biểu phí cấu hình |
| `DELETE`| `/api/v1/channels/:id` | Xóa hoặc hủy kích hoạt gian hàng |

---

## 3. Tải lên & Quản lý Bảng kê (Statements)

| Method | Endpoint | Mô tả |
| ------ | -------- | ----- |
| `POST` | `/api/v1/statements/upload` | Tải lên file bảng kê Excel / CSV (`multipart/form-data`) |
| `GET`  | `/api/v1/statements/batches`| Lấy lịch sử các đợt tải bảng kê |
| `GET`  | `/api/v1/statements/batches/:id` | Xem chi tiết tiến độ xử lý của một đợt |

---

## 4. Đối soát & Báo cáo Dòng tiền (Reconciliation & Analytics)

| Method | Endpoint | Mô tả |
| ------ | -------- | ----- |
| `POST` | `/api/v1/reconciliation/run` | Kích hoạt phiên đối soát theo điều kiện lọc |
| `GET`  | `/api/v1/reconciliation/items` | Truy vấn danh sách chi tiết đơn hàng đối soát |
| `GET`  | `/api/v1/reconciliation/summary` | Lấy số liệu tổng hợp (Tổng tiền, tổng phí, lệch COD) |
| `GET`  | `/api/v1/dashboard/cashflow-metrics` | Số liệu biểu đồ dòng tiền theo thời gian |

---

## 5. Trung tâm Cảnh báo (Alerts)

| Method | Endpoint | Mô tả |
| ------ | -------- | ----- |
| `GET`  | `/api/v1/alerts` | Lấy danh sách cảnh báo bất thường |
| `PUT`  | `/api/v1/alerts/:id/resolve` | Đánh dấu đã xử lý / giải quyết khiếu nại |
| `GET`  | `/api/v1/alerts/rules` | Xem danh sách quy tắc cảnh báo |
| `POST` | `/api/v1/alerts/rules` | Tạo mới hoặc cập nhật ngưỡng cảnh báo |

---

## 6. WebSocket Kênh Thời gian thực (`/ws`)

- **Topic `recon:progress`**: Đẩy % tiến độ phân tích và xử lý bảng kê của DuckDB.
- **Topic `alerts:new`**: Đẩy thông báo ngay khi phát hiện đơn hàng bị lệch tiền nghiêm trọng.
