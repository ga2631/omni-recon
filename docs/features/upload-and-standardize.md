# Feature: Upload & Standardize (Phase 1 Ingestion)

This document provides complete technical specifications for the **Upload & Standardize** subsystem in OmniRecon, enabling multi-channel e-commerce sellers in Vietnam (Shopee, TikTok Shop) to ingest financial settlement statements into a conformed Medallion architecture (Bronze & Silver tiers).

---

## 1. End-to-End System Flow

```
[ Vue 3 Client (UploadStatementView.vue + Pinia Store) ]
          │
          │ 1. Multipart POST (/api/v1/statements/upload)
          │    - Payload: shop_id, platform (SHOPEE|TIKTOK), report_type, file (.xlsx/.csv)
          ▼
[ Rust Axum Backend Server (statements.rs) ]
          │
          │ 2. Stream Buffer & SHA-256 Checksum Calculation
          │    - Magic byte validation (XLSX Zip / UTF-8 CSV)
          │    - Duplicate check against Bronze log records
          │
          │ 3. Bronze Layer Persistence
          │    - Written to storage/bronze/{merchant_id}/{shop_id}/{YYYY}/{MM}/{log_id}_{hash}.{ext}
          │    - Immutable, append-only raw auditing tier
          ▼
[ DuckDB + Calamine Analytical ETL Engine (shopee.rs / tiktok.rs / duckdb_runner.rs) ]
          │
          │ 4. Vectorized Parsing & Header Normalization
          │    - Shopee / TikTok column heuristic detection
          │    - Currency normalization (VND decimal conversion, negative parentheses)
          │    - Flexible ISO datetime parsing
          │
          │ 5. In-Memory DuckDB Staging & Financial Integrity Check
          │    - Staged into in-memory table `staging_settlement_records`
          │    - Formula assertion: Net = Gross - Discounts - Fees + Subsidies
          │    - Discrepant/balanced rows tagged
          ▼
[ PostgreSQL 16 Silver Conformed Layer ]
          │
          │ 6. High-Speed Upsert into Relational Schema
          │    - `upload_logs`: State set to COMPLETED with row metrics
          │    - `unified_orders`: Conformed order master records
          │    - `unified_transactions`: Financial breakdown with JSONB fee payload
          ▼
[ Vue 3 Client UI (Real-time Reactive Updates) ]
          │
          │ 7. Instant JSON Response & Render
          │    - Progress bar & Medallion tier badges (Bronze -> DuckDB -> Silver)
          │    - Sanity check counters (Gross, Net, Total Fees, Balanced rows)
          │    - 5-row Silver Preview Table with formatted VND currencies
          │    - Updated recent batch log history list
```

---

## 2. Database & Schema Changes

The following tables and indexes were introduced in `deploy/sql/01_init_schema.sql`:

### 2.1. Bronze Metadata Layer
- **`upload_logs`**:
  - `id` (UUID PRIMARY KEY)
  - `merchant_id` (UUID REFERENCES `merchants`)
  - `shop_id` (UUID REFERENCES `shops`)
  - `platform` (`SHOPEE`, `TIKTOK`, `LAZADA`, `TIKI`, `GHN`, etc.)
  - `report_type` (`INCOME_STATEMENT`, `SETTLEMENT_REPORT`, etc.)
  - `original_filename` (VARCHAR)
  - `file_path` (VARCHAR - Physical Bronze location)
  - `file_hash` (VARCHAR(64) - SHA-256 Checksum, unique per shop for idempotency)
  - `file_size_bytes` (BIGINT)
  - `total_rows`, `successful_rows`, `failed_rows` (INT)
  - `status` (`PENDING`, `VALIDATING`, `PROCESSING`, `COMPLETED`, `FAILED`)
  - `error_summary` (JSONB)
  - `created_at`, `processed_at` (TIMESTAMPTZ)

### 2.2. Silver Conformed Layer
- **`unified_orders`**:
  - Master order header table storing `platform_order_id`, `order_status`, `buyer_username`, `tracking_number`, `ordered_at`, `delivered_at`.
  - Includes `raw_attributes` (JSONB) with GIN indexing (`jsonb_path_ops`) for flexible platform attributes.
- **`unified_transactions`**:
  - Line-item settlement table storing core financial dimensions: `gross_amount`, `seller_discount`, `platform_voucher`, `buyer_shipping_fee`, `seller_shipping_fee`, `shipping_subsidy`, `commission_fee`, `service_fee`, `payment_fee`, `affiliate_commission_fee`, `other_fees`, `net_settlement`, and `settled_at`.
  - Includes `raw_fee_breakdown` (JSONB) with GIN indexing to retain platform fee line items.

---

## 3. Technical Optimizations

1. **Streaming SHA-256 Hashing & Zero-copy Buffering**:
   - Computes the file's cryptographic hash in a single pass during multipart stream ingestion without reading the file into memory twice.
2. **DuckDB In-Memory Staging & Vectorized Verification**:
   - Executes multi-threaded SIMD SQL queries across in-memory columnar buffers to verify the financial integrity formula ($\text{Calculated Net} \approx \text{Net Settlement}$) in milliseconds, avoiding load on PostgreSQL.
3. **Dynamic Heuristic Header Matching**:
   - Shopee and TikTok Excel/CSV parsers scan the first 15 rows to automatically locate table headers, accommodating changing platform export templates without code alterations.
4. **Resilient Currency & Date Sanitization**:
   - Handles Vietnamese number conventions (dot as thousand separator, comma as decimal), accounting bracket notations `(15.000)`, and diverse date formats (`%d-%m-%Y %H:%M`, `%Y-%m-%d %H:%M:%S`).
5. **Idempotency Guarantee**:
   - Composite unique constraint `(shop_id, file_hash)` in `upload_logs` prevents accidental duplicate batch processing.
6. **Reactive Frontend State Management**:
   - Pinia `useStatementsStore` provides optimistic updates, live progress reporting, formatted Vietnamese currency rendering (`Intl.NumberFormat`), and error boundary handling.

---

## 4. Impacted Files

| File Path | Responsibility |
| :--- | :--- |
| `frontend/src/views/UploadStatementView.vue` | Interactive Vue 3 upload view with drag-and-drop, live progress, sanity cards, preview table, and recent batch logs. |
| `frontend/src/stores/statements.ts` | Pinia store managing statement upload lifecycle, channel bindings, batch history, and currency formatters. |
| `frontend/src/types/index.ts` | TypeScript definitions for `UploadStatementResult`, `FinancialSanitySummary`, `StandardSettlementRecord`, and `StatementBatchItem`. |
| `deploy/sql/01_init_schema.sql` | PostgreSQL 16 DDL for Bronze metadata (`upload_logs`) and Silver conformed schema (`unified_orders`, `unified_transactions`). |
| `deploy/sql/02_seed_data.sql` | Seed data for default merchant, demo shops, and initial alert rules. |
| `backend/common/src/models.rs` | Strongly-typed Rust structs and domain models for `Merchant`, `Shop`, `UploadLog`, `UnifiedOrder`, `UnifiedTransaction`. |
| `backend/engine/src/parsers/mod.rs` | Conformed DTOs (`StandardSettlementRecord`) and currency/datetime normalization logic. |
| `backend/engine/src/parsers/shopee.rs` | Excel (`.xlsx`) and CSV parser with column heuristic mapping for Shopee income statements. |
| `backend/engine/src/parsers/tiktok.rs` | Excel (`.xlsx`) and CSV parser with column heuristic mapping for TikTok Shop settlement statements. |
| `backend/engine/src/duckdb_runner.rs` | DuckDB in-memory batch staging, financial integrity checks, and OLAP aggregations. |
| `backend/engine/tests/parser_tests.rs` | Automated unit tests validating Shopee/TikTok statement parsing and DuckDB verification. |
| `backend/server/src/handlers/statements.rs` | Axum HTTP multipart upload handler, SHA-256 hashing, Bronze file persistence, and ETL trigger. |
| `backend/Cargo.toml` & crate configs | Workspace dependency configuration including `sha2`, `hex`, and `tempfile`. |
