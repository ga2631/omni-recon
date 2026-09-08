# Hướng dẫn Đóng góp Mã nguồn (Contributing Guide)

Cảm ơn bạn đã quan tâm đến việc đóng góp cho dự án mã nguồn mở **OmniRecon**! 🎉

Dưới đây là các hướng dẫn giúp bạn tham gia phát triển dự án một cách thuận tiện và hiệu quả.

---

## 1. Nguyên tắc Đóng góp

- **Tôn trọng Bản quyền**: Mọi đóng góp đều phải tuân thủ giấy phép **AGPL-3.0** và điều khoản trong tệp [NOTICE](NOTICE).
- **Chất lượng Mã nguồn**: Mã nguồn cần được kiểm tra format (`cargo fmt`, `prettier`), chạy test vượt qua 100% trước khi gửi Pull Request.
- **Tài liệu hóa**: Khi thêm tính năng mới hoặc thay đổi logic nghiệp vụ, vui lòng cập nhật tài liệu tương ứng trong thư mục `docs/`.

---

## 2. Quy ước Đặt tên Commit (Conventional Commits)

Chúng tôi sử dụng chuẩn [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Thêm tính năng mới (ví dụ: `feat(engine): add TikTok Shop settlement parser`)
- `fix:` Sửa lỗi (ví dụ: `fix(frontend): correct COD mismatch badge color`)
- `docs:` Thay đổi hoặc bổ sung tài liệu (ví dụ: `docs: update deployment guide for Nginx SSL`)
- `style:` Thay đổi định dạng code (khoảng trắng, format, linting)
- `refactor:` Tái cấu trúc mã nguồn không thay đổi tính năng
- `perf:` Cải thiện hiệu năng xử lý (ví dụ: `perf(duckdb): optimize batch join query`)
- `test:` Thêm hoặc sửa đổi unit tests / integration tests
- `chore:` Cập nhật dependencies, build scripts, cấu hình CI/CD

---

## 3. Quy trình Đóng góp qua GitHub

1. **Fork** repository về tài khoản cá nhân của bạn.
2. Tạo nhánh tính năng mới từ `main`:
   ```bash
   git checkout -b feat/my-awesome-feature
   ```
3. Thực hiện thay đổi mã nguồn và commit:
   ```bash
   git commit -m "feat(module): description of changes"
   ```
4. Đảm bảo toàn bộ test và linter vượt qua:

   ```bash
   # Kiểm tra Rust Backend
   cd backend && cargo check && cargo test

   # Kiểm tra Vue Frontend
   cd ../frontend && npm run build
   ```

5. Push nhánh lên GitHub và tạo **Pull Request** (PR) vào nhánh `main` của repo gốc.
6. Mô tả rõ ràng mục tiêu của PR, các thay đổi chính và ảnh chụp màn hình kiểm thử (nếu liên quan đến giao diện người dùng).

---

## 4. Báo cáo Lỗi & Đề xuất Tính năng

Nếu bạn phát hiện lỗi (bug) hoặc có ý tưởng cải tiến:

- Mở một **Issue** trên GitHub.
- Mô tả chi tiết các bước tái hiện lỗi kèm log và ảnh chụp màn hình (nếu có).
- Đối với lỗ hổng bảo mật, vui lòng làm theo hướng dẫn trong [SECURITY.md](SECURITY.md).
