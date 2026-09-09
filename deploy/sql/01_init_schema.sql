-- ==============================================================================
-- OmniRecon Database Schema (PostgreSQL 16)
-- Medallion Architecture: Bronze Metadata & Conformed Silver Tables
-- ==============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";

-- 1. Merchants / Organizations (Multi-tenant Master)
CREATE TABLE IF NOT EXISTS merchants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    tax_id VARCHAR(50),
    currency VARCHAR(10) NOT NULL DEFAULT 'VND',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 2. Users & Authentication
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'ADMIN' CHECK (role IN ('ADMIN', 'ACCOUNTANT', 'VIEWER')),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_merchant_id ON users(merchant_id);

-- 3. Multi-channel Shops / Stores
CREATE TABLE IF NOT EXISTS shops (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    code VARCHAR(100) NOT NULL, -- e.g. 'shopee_store_01', 'ghn_main'
    name VARCHAR(255) NOT NULL,
    platform VARCHAR(50) NOT NULL CHECK (
        platform IN ('SHOPEE', 'TIKTOK', 'LAZADA', 'TIKI', 'GHN', 'GHTK', 'VIETTEL_POST', 'NINJA_VAN', 'BANK_STATEMENT', 'CUSTOM_OMS')
    ),
    shop_identifier VARCHAR(150), -- Remote platform shop/seller ID
    config_json JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_shops_merchant_code UNIQUE (merchant_id, code)
);

CREATE INDEX IF NOT EXISTS idx_shops_merchant_platform ON shops(merchant_id, platform);

-- 4. Bronze Layer Metadata: Upload Logs
CREATE TABLE IF NOT EXISTS upload_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    shop_id UUID REFERENCES shops(id) ON DELETE SET NULL,
    platform VARCHAR(50) NOT NULL CHECK (
        platform IN ('SHOPEE', 'TIKTOK', 'LAZADA', 'TIKI', 'GHN', 'GHTK', 'VIETTEL_POST', 'NINJA_VAN', 'BANK_STATEMENT', 'CUSTOM_OMS')
    ),
    report_type VARCHAR(50) NOT NULL DEFAULT 'INCOME_STATEMENT' CHECK (
        report_type IN ('INCOME_STATEMENT', 'SETTLEMENT_REPORT', 'LOGISTICS_REPORT', 'BANK_STATEMENT')
    ),
    original_filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(512) NOT NULL,
    file_hash VARCHAR(64) NOT NULL, -- SHA-256 Checksum for deduplication
    file_size_bytes BIGINT NOT NULL DEFAULT 0,
    total_rows INT DEFAULT 0,
    successful_rows INT DEFAULT 0,
    failed_rows INT DEFAULT 0,
    status VARCHAR(30) NOT NULL DEFAULT 'PENDING' CHECK (
        status IN ('PENDING', 'VALIDATING', 'PROCESSING', 'COMPLETED', 'FAILED')
    ),
    error_summary JSONB DEFAULT '{}'::jsonb,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ,
    CONSTRAINT uq_upload_logs_shop_file_hash UNIQUE (shop_id, file_hash)
);

CREATE INDEX IF NOT EXISTS idx_upload_logs_merchant_status ON upload_logs(merchant_id, status);
CREATE INDEX IF NOT EXISTS idx_upload_logs_created_at ON upload_logs(created_at DESC);

-- 5. Silver Layer: Unified Orders (Conformed Order Master)
CREATE TABLE IF NOT EXISTS unified_orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    shop_id UUID NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    upload_log_id UUID REFERENCES upload_logs(id) ON DELETE SET NULL,
    platform VARCHAR(50) NOT NULL,
    platform_order_id VARCHAR(150) NOT NULL,
    order_status VARCHAR(50) NOT NULL DEFAULT 'COMPLETED' CHECK (
        order_status IN ('PENDING', 'PROCESSING', 'DELIVERED', 'COMPLETED', 'CANCELLED', 'RETURNED')
    ),
    buyer_username VARCHAR(150),
    tracking_number VARCHAR(150),
    ordered_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    raw_attributes JSONB DEFAULT '{}'::jsonb, -- Dynamic EAV JSONB attributes
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_unified_orders_shop_platform_order UNIQUE (shop_id, platform_order_id)
);

CREATE INDEX IF NOT EXISTS idx_unified_orders_lookup ON unified_orders(shop_id, platform_order_id);
CREATE INDEX IF NOT EXISTS idx_unified_orders_tracking ON unified_orders(tracking_number);
CREATE INDEX IF NOT EXISTS idx_unified_orders_raw_attrs ON unified_orders USING GIN (raw_attributes jsonb_path_ops);

-- 6. Silver Layer: Unified Transactions (Conformed Financial Line Items)
CREATE TABLE IF NOT EXISTS unified_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID REFERENCES unified_orders(id) ON DELETE CASCADE,
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    shop_id UUID NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    upload_log_id UUID NOT NULL REFERENCES upload_logs(id) ON DELETE CASCADE,
    platform VARCHAR(50) NOT NULL,
    platform_order_id VARCHAR(150) NOT NULL,
    payout_id VARCHAR(150), -- Settlement batch ID
    transaction_type VARCHAR(50) NOT NULL DEFAULT 'ORDER_SETTLEMENT' CHECK (
        transaction_type IN ('ORDER_SETTLEMENT', 'REFUND_ADJUSTMENT', 'SHIPPING_FEE_ADJUSTMENT', 'COMPENSATION', 'AFFILIATE_DEDUCTION', 'OTHER_FEE')
    ),
    
    -- Monetary columns (Currency VND, 2 decimal places)
    gross_amount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    seller_discount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    platform_voucher NUMERIC(18, 2) NOT NULL DEFAULT 0,
    buyer_shipping_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    seller_shipping_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    shipping_subsidy NUMERIC(18, 2) NOT NULL DEFAULT 0,
    commission_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    service_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    payment_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    affiliate_commission_fee NUMERIC(18, 2) NOT NULL DEFAULT 0,
    other_fees NUMERIC(18, 2) NOT NULL DEFAULT 0,
    net_settlement NUMERIC(18, 2) NOT NULL DEFAULT 0,
    
    settled_at TIMESTAMPTZ,
    raw_fee_breakdown JSONB DEFAULT '{}'::jsonb, -- Raw unrounded fees from platform
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_unified_tx_unique_entry UNIQUE (shop_id, platform_order_id, payout_id, transaction_type)
);

CREATE INDEX IF NOT EXISTS idx_unified_tx_shop_payout ON unified_transactions(shop_id, payout_id);
CREATE INDEX IF NOT EXISTS idx_unified_tx_settled_at ON unified_transactions(merchant_id, settled_at);
CREATE INDEX IF NOT EXISTS idx_unified_tx_raw_fee ON unified_transactions USING GIN (raw_fee_breakdown jsonb_path_ops);

-- 7. Reconciliation Jobs
CREATE TABLE IF NOT EXISTS reconciliation_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    date_from DATE NOT NULL,
    date_to DATE NOT NULL,
    status VARCHAR(50) DEFAULT 'QUEUED' CHECK (status IN ('QUEUED', 'RUNNING', 'COMPLETED', 'FAILED')),
    progress_percent INT DEFAULT 0,
    total_matched INT DEFAULT 0,
    total_discrepant INT DEFAULT 0,
    total_amount_expected NUMERIC(18, 2) DEFAULT 0,
    total_amount_settled NUMERIC(18, 2) DEFAULT 0,
    total_fee_deducted NUMERIC(18, 2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_recon_jobs_merchant_date ON reconciliation_jobs(merchant_id, date_from, date_to);

-- 8. Discrepancy Alerts
CREATE TABLE IF NOT EXISTS discrepancy_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    job_id UUID REFERENCES reconciliation_jobs(id) ON DELETE SET NULL,
    shop_code VARCHAR(100) NOT NULL,
    order_id VARCHAR(150) NOT NULL,
    tracking_code VARCHAR(150),
    alert_type VARCHAR(100) NOT NULL CHECK (
        alert_type IN ('COD_MISMATCH', 'FEE_MISMATCH', 'OVERDUE_PAYOUT', 'RETURN_LOST', 'ABNORMAL_SHIPPING', 'MISSING_SETTLEMENT')
    ),
    severity VARCHAR(20) DEFAULT 'HIGH' CHECK (severity IN ('LOW', 'MEDIUM', 'HIGH', 'CRITICAL')),
    expected_amount NUMERIC(18, 2) DEFAULT 0,
    actual_amount NUMERIC(18, 2) DEFAULT 0,
    discrepancy_amount NUMERIC(18, 2) DEFAULT 0,
    notes TEXT,
    status VARCHAR(50) DEFAULT 'OPEN' CHECK (status IN ('OPEN', 'INVESTIGATING', 'RESOLVED', 'IGNORED')),
    resolved_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    resolved_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_alerts_merchant_status ON discrepancy_alerts(merchant_id, status);
CREATE INDEX IF NOT EXISTS idx_alerts_order_id ON discrepancy_alerts(order_id);

-- 9. Alert Rules & Thresholds
CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    rule_code VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    conditions_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    severity VARCHAR(20) DEFAULT 'HIGH',
    is_enabled BOOLEAN DEFAULT TRUE,
    notification_channels JSONB DEFAULT '["IN_APP"]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 10. Cashflow Metrics Aggregation (Daily snapshot)
CREATE TABLE IF NOT EXISTS cashflow_timeline (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    merchant_id UUID NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    record_date DATE NOT NULL,
    shop_id UUID REFERENCES shops(id) ON DELETE SET NULL,
    gross_sales NUMERIC(18, 2) DEFAULT 0,
    net_settlement NUMERIC(18, 2) DEFAULT 0,
    total_fees NUMERIC(18, 2) DEFAULT 0,
    cod_pending NUMERIC(18, 2) DEFAULT 0,
    discrepancy_sum NUMERIC(18, 2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_cashflow_daily UNIQUE (merchant_id, record_date, shop_id)
);

CREATE INDEX IF NOT EXISTS idx_cashflow_date ON cashflow_timeline(merchant_id, record_date);

-- ==============================================================================
-- 11. Compatibility Views for Legacy References
-- ==============================================================================
CREATE OR REPLACE VIEW organizations AS 
    SELECT id, name, currency, created_at, updated_at FROM merchants;

CREATE OR REPLACE VIEW channels AS 
    SELECT id, merchant_id AS org_id, code, name, platform AS platform_type, config_json, is_active, created_at, updated_at FROM shops;

CREATE OR REPLACE VIEW statement_batches AS 
    SELECT id, merchant_id AS org_id, shop_id AS channel_id, original_filename AS filename, 
           file_path, file_size_bytes, total_rows, status, error_summary->>'message' AS error_message, 
           created_at, processed_at 
    FROM upload_logs;
