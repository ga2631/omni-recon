# 📋 Đặc tả Định dạng Bảng kê Đa kênh (Data Formats Specification)

Tài liệu này cung cấp định dạng chuẩn của các bảng kê sao kê thu nhập và vận chuyển từ các kênh TMĐT phổ biến tại Việt Nam được OmniRecon hỗ trợ.

---

## 1. Kênh Shopee (Báo cáo Thu nhập / Income Statement)

- **Định dạng file**: Excel (`.xlsx`), CSV (`UTF-8`)
- **Tên sheet**: `Income`, `Report` hoặc sheet đầu tiên
- **Các trường dữ liệu quan trọng**:

| Tên cột Shopee (VN) | Mã trường chuẩn OmniRecon | Kiểu dữ liệu | Ý nghĩa |
| ------------------- | ------------------------- | ------------ | ------- |
| Mã đơn hàng | `order_id` | String | Mã định danh đơn hàng trên sàn |
| Mã phiên thanh toán | `payout_id` | String | Mã lô rút tiền về ví / ngân hàng |
| Giá bán gốc | `original_price` | Decimal | Tổng giá trị hàng hóa niêm yết |
| Giảm giá của Shop | `seller_discount` | Decimal | Khuyến mãi do shop chi trả |
| Voucher của Shopee | `platform_voucher` | Decimal | Shopee tài trợ chiết khấu |
| Phí cố định | `commission_fee` | Decimal | Phí hoa hồng phần trăm của sàn |
| Phí dịch vụ | `service_fee` | Decimal | Phí gói Freeship Xtra, Hoàn xu Xtra |
| Phí thanh toán | `payment_fee` | Decimal | Phí xử lý giao dịch điện tử (thường 4-5%) |
| Phí vận chuyển | `shipping_fee` | Decimal | Phí ship thực tính vào đơn |
| Số tiền nhận được | `net_settlement` | Decimal | Số tiền thực tế đổ về số dư |
| Thời gian thanh toán | `settled_at` | DateTime | Ngày hoàn tất chuyển tiền |

---

## 2. Kênh TikTok Shop (Báo cáo Quyết toán / Settlement Report)

- **Định dạng file**: Excel (`.xlsx`)
- **Các trường dữ liệu quan trọng**:

| Tên cột TikTok Shop | Mã trường chuẩn OmniRecon | Kiểu dữ liệu | Ý nghĩa |
| ------------------- | ------------------------- | ------------ | ------- |
| Order ID | `order_id` | String | Mã đơn hàng |
| Settlement ID | `payout_id` | String | Mã phiên thanh toán |
| Subtotal Before Discount | `subtotal` | Decimal | Giá trị đơn hàng trước giảm |
| Seller Discount | `seller_discount` | Decimal | Giảm giá người bán |
| TikTok Shop Discount | `platform_voucher` | Decimal | Voucher tài trợ bởi TikTok |
| Marketplace Commission Fee | `commission_fee` | Decimal | Phí hoa hồng nền tảng |
| Transaction Fee | `payment_fee` | Decimal | Phí giao dịch |
| Affiliate Commission | `affiliate_fee` | Decimal | Hoa hồng trả cho KOC/KOL |
| Shipping Fee Incentive | `shipping_subsidy`| Decimal | Trợ giá vận chuyển |
| Actual Amount Transferred | `net_settlement` | Decimal | Tiền thực nhận sau khi cấn trừ |
| Settlement Time | `settled_at` | DateTime | Thời gian quyết toán |

---

## 3. Kênh Lazada (Sao kê Tài khoản / Account Statement)

- **Định dạng file**: CSV (`UTF-8`), Excel (`.xlsx`)
- **Các trường dữ liệu quan trọng**:
  - `Order Number` (`order_id`)
  - `Transaction Date` (`settled_at`)
  - `Fee Type` (Item Price Credit, Payment Fee, Commission, Shipping Fee Paid by Customer, Shipping Fee Paid by Seller)
  - `Amount` (`amount`)
  - `Statement Number` (`payout_id`)

---

## 4. Đơn vị Vận chuyển (GHN / GHTK / Viettel Post / Ninja Van)

- **Định dạng file**: Excel (`.xlsx`), CSV
- **Các trường dữ liệu quan trọng**:

| Tên trường | Mã chuẩn OmniRecon | Ý nghĩa |
| ---------- | ------------------ | ------- |
| Mã vận đơn / Tracking Code | `tracking_code` | Mã theo dõi hành trình kiện hàng |
| Mã đơn hàng đối tác | `order_id` | Mã đơn liên kết với shop |
| Tiền thu hộ COD | `cod_amount` | Số tiền shipper thu từ người mua |
| Phí cước vận chuyển | `shipping_fee` | Cước phí chuyển phát |
| Phí bảo hiểm hàng hóa | `insurance_fee` | Phí bảo hiểm nếu có |
| Cân nặng quy đổi (gram) | `charged_weight` | Trọng lượng bị tính phí |
| Trạng thái giao hàng | `delivery_status` | Giao thành công / Đang chuyển / Hoàn hàng |
| Ngày giao thành công | `delivered_at` | Thời điểm giao hàng |
