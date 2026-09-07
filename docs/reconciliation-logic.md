# 🔍 Thuật toán & Quy tắc Đối soát (Reconciliation Logic)

Tài liệu này mô tả chi tiết các mô hình và quy tắc đối soát đa kênh được hiện thực hóa trong **OmniRecon ETL & Engine**.

---

## 1. Mô hình Đối soát 3 Chiều (3-Way Reconciliation Model)

```
        ┌─────────────────────────┐
        │  1. ĐƠN HÀNG NỘI BỘ    │
        │     (OMS / ERP / POS)   │
        └────────────┬────────────┘
                     │
         [So khớp Mã đơn & Doanh thu]
                     │
                     ▼
        ┌─────────────────────────┐      [So khớp Phí ship & COD]      ┌─────────────────────────┐
        │   2. BẢNG KÊ SÀN TMĐT   │ ◄────────────────────────────────► │  3. SAO KÊ ĐƠN VỊ VẬN   │
        │ (Shopee, TikTok, Lazada)│                                    │  CHUYỂN (GHN, GHTK,...) │
        └────────────┬────────────┘                                    └─────────────────────────┘
                     │
        [So khớp Mã phiên thanh toán & Số tiền về]
                     │
                     ▼
        ┌─────────────────────────┐
        │ 4. BIẾN ĐỘNG SỐ DƯ NH   │
        │     (Bank Statement)    │
        └─────────────────────────┘
```

---

## 2. Các Quy tắc So khớp Chi tiết

### 2.1. So khớp Đơn hàng Sàn (E-Commerce Settlement Matching)

| Tiêu chí | Công thức kiểm tra | Mức độ cảnh báo |
| -------- | ------------------- | --------------- |
| **Doanh thu sản phẩm** | `DoanhThu_San == DoanhThu_OMS` | 🟡 Cảnh báo nếu chênh lệch > 0 |
| **Phí cố định & Phí thanh toán** | `TongPhi_San <= DoanhThu * TyLePhiToiDa` (theo chính sách từng sàn) | 🔴 Cảnh báo lệch biểu phí nếu vượt ngưỡng |
| **Trợ giá / Voucher sàn** | `TienThucNhan_San == DoanhThu - PhiSan + TroGiaSan` | 🔴 Cảnh báo nếu số tiền thực nhận không đúng công thức sàn |
| **Đơn hủy / Trả hàng hoàn tiền** | Kiểm tra đơn đã hoàn tiền có bị sàn trừ phí vận chuyển 2 chiều bất thường hay không | 🔴 Cảnh báo phí hoàn hàng sai quy định |

### 2.2. So khớp Phí Vận chuyển & Tiền Thu hộ COD (Carrier Matching)

1. **Kiểm tra trạng thái Giao hàng vs Nhận tiền**:
   - Trạng thái ĐVVC: `Giao hàng thành công`.
   - Trạng thái Tiền: Tiền COD chưa được ghi nhận trong bảng kê thanh toán sau quá `N` ngày (mặc định 7 ngày) -> **Cảnh báo Om tiền COD**.
2. **Kiểm tra Phí Vận chuyển Thực tế vs Tạm tính**:
   - `PhiShip_ThucTe > PhiShip_DuKien * 1.15` (Chênh lệch vượt cân nặng quá 15%) -> **Cảnh báo Phí vượt cân bất thường**.
3. **Kiểm tra Đơn chuyển hoàn (Return Orders)**:
   - Đơn giao không thành công phải có bản ghi hoàn hàng hợp lệ trong kho nội bộ trong vòng 14 ngày.

### 2.3. So khớp Phiên chuyển tiền về Tài khoản Ngân hàng

1. Gom nhóm toàn bộ bản ghi đơn hàng theo `Payout_ID` (Mã phiên rút tiền/thanh toán của sàn).
2. Tính `TongTien_Phien = SUM(TienThucNhan_TungDon)`.
3. So khớp với dòng tiền vào (Credit) trong Sao kê Ngân hàng có nội dung chuyển khoản chứa `Payout_ID` hoặc tên sàn.
4. Phát hiện các phiên sàn báo "Đã thanh toán" nhưng tài khoản ngân hàng chưa nhận được tiền.

---

## 3. Quy trình Xử lý Bằng DuckDB Engine

```sql
-- Ví dụ truy vấn SQL DuckDB phát hiện lệch tiền COD và phí sàn
CREATE OR REPLACE TABLE recon_mismatch_results AS
SELECT 
    s.order_id,
    s.channel_code,
    s.order_amount AS expected_amount,
    s.settlement_amount AS actual_amount,
    (s.order_amount - s.settlement_amount) AS discrepancy_amount,
    c.cod_amount AS carrier_cod_amount,
    CASE 
        WHEN s.settlement_amount IS NULL THEN 'MISSING_SETTLEMENT'
        WHEN ABS(s.order_amount - s.settlement_amount - s.total_fee) > 1000 THEN 'FEE_MISMATCH'
        WHEN c.cod_amount IS NOT NULL AND c.cod_amount != s.order_amount THEN 'COD_MISMATCH'
        ELSE 'MATCHED'
    END AS recon_status
FROM read_parquet('data/storage/orders/*.parquet') s
LEFT JOIN read_parquet('data/storage/carrier/*.parquet') c 
    ON s.order_id = c.order_id;
```
