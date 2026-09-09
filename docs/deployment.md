# 🚀 Hướng dẫn Triển khai Hệ thống (Deployment Guide)

Tài liệu này hướng dẫn chi tiết cách chạy **OmniRecon** trên môi trường phát triển (Development với Hot-Reload) và môi trường sản xuất (Production).

---

## 1. Môi trường Phát triển Hot-Reload (Development Mode)

Khi lập trình, chế độ Development cung cấp **Hot-Reloading** tức thì cho cả Backend (Rust) và Frontend (Vue 3 / Vite) mà không cần build lại toàn bộ image:

- ⚡ **Frontend**: Vite HMR (Hot Module Replacement) cập nhật giao diện trong vài mili-giây ngay khi lưu file `.vue`.
- 🦀 **Backend**: Tích hợp `cargo-watch` tự động biên dịch lại mã nguồn Rust (`omni-server`) và lưu cache `target/` vào Docker volume.

### Khởi chạy môi trường Dev:
```bash
# Sử dụng Makefile tiện ích:
make dev

# Hoặc lệnh docker compose trực tiếp:
docker compose -f docker-compose.dev.yml up
```

### Dừng môi trường Dev:
```bash
make dev-down
# hoặc: docker compose -f docker-compose.dev.yml down
```

### Các cổng dịch vụ trong chế độ Dev:
- 🌐 **Web Dashboard & Gateway**: `http://localhost` (hoặc `http://localhost:3000` cho Vite trực tiếp)
- 🔌 **Backend REST API**: `http://localhost:8080/api/v1`
- 🐘 **PostgreSQL**: `localhost:5432`
- ⚡ **Redis**: `localhost:6379`

---

## 2. Triển khai Môi trường Sản xuất (Production Mode)

### Yêu cầu Phần cứng Tối thiểu:
| Thành phần | Môi trường Nhỏ (< 50,000 đơn/tháng) | Môi trường Lớn (> 500,000 đơn/tháng) |
| ---------- | ----------------------------------- | ----------------------------------- |
| **CPU** | 2 Cores | 4 - 8 Cores |
| **RAM** | 4 GB | 8 - 16 GB (DuckDB tận dụng RAM để tăng tốc truy vấn) |
| **Ổ cứng** | 40 GB SSD / NVMe | 120+ GB SSD / NVMe |
| **HĐH** | Ubuntu 22.04 LTS, Debian 12, macOS | Ubuntu 22.04 LTS, Debian 12 |

### Khởi chạy Production:
```bash
cp .env.example .env
docker compose up -d
```

---

## 3. Sao lưu Dữ liệu Định kỳ (Backup & Restore)

### Sao lưu PostgreSQL:
```bash
docker compose exec postgres pg_dump -U omnirecon omnirecon > backup_$(date +%Y%m%d).sql
```

### Sao lưu Thư mục Bronze Storage:
```bash
tar -czvf storage_backup_$(date +%Y%m%d).tar.gz ./data/
```
