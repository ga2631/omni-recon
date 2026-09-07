# 🚀 Hướng dẫn Triển khai Hệ thống (Deployment Guide)

Tài liệu này hướng dẫn chi tiết cách triển khai **OmniRecon** trên môi trường máy chủ cục bộ (Self-hosted) hoặc máy chủ đám mây (Cloud VPS / Dedicated Server).

---

## 1. Yêu cầu Phần cứng Tối thiểu

| Thành phần | Môi trường Nhỏ (< 50,000 đơn/tháng) | Môi trường Lớn (> 500,000 đơn/tháng) |
| ---------- | ----------------------------------- | ----------------------------------- |
| **CPU** | 2 Cores | 4 - 8 Cores |
| **RAM** | 4 GB | 8 - 16 GB (DuckDB tận dụng RAM để tăng tốc truy vấn) |
| **Ổ cứng** | 40 GB SSD / NVMe | 120+ GB SSD / NVMe |
| **HĐH** | Ubuntu 22.04 LTS, Debian 12, macOS | Ubuntu 22.04 LTS, Debian 12 |

---

## 2. Triển khai Nhanh bằng Docker Compose (Khuyến nghị)

### Bước 1: Chuẩn bị Thư mục & Biến môi trường
```bash
git clone https://github.com/your-org/omni-recon.git
cd omni-recon
cp .env.example .env
```

Chỉnh sửa các tham số bảo mật trong tệp `.env`:
```ini
APP_SECRET=hay-doi-chuoi-bi-mat-nay-it-nhat-32-ky-tu-ngau-nhien
POSTGRES_PASSWORD=mat-khau-csdl-an-toan-cua-ban
```

### Bước 2: Khởi chạy
```bash
docker compose up -d
```

### Bước 3: Kiểm tra Trạng thái Container
```bash
docker compose ps
```

---

## 3. Cấu hình Tên miền & SSL / HTTPS với Let's Encrypt

Khi triển khai trên IP Public / Tên miền chính thức:
1. Trỏ DNS Domain `A Record` về IP máy chủ của bạn.
2. Cài đặt **Certbot**:
   ```bash
   sudo apt install certbot python3-certbot-nginx
   sudo certbot --nginx -d omnirecon.yourdomain.com
   ```
3. Cập nhật cấu hình Nginx trong `deploy/nginx/default.conf` để kích hoạt SSL Certificate.

---

## 4. Sao lưu Dữ liệu Định kỳ (Backup & Restore)

### Sao lưu PostgreSQL:
```bash
docker compose exec postgres pg_dump -U omnirecon omnirecon > backup_$(date +%Y%m%d).sql
```

### Sao lưu Thư mục Dữ liệu Parquet / Storage:
```bash
tar -czvf storage_backup_$(date +%Y%m%d).tar.gz ./data/
```
