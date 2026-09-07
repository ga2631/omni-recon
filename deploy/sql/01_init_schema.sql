-- ==============================================================================
-- OmniRecon Initial Database Schema (PostgreSQL 16)
-- ==============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Organizations & Workspaces
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    currency VARCHAR(10) DEFAULT 'VND',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 2. Users & Authentication
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'ADMIN' CHECK (role IN ('ADMIN', 'ACCOUNTANT', 'VIEWER')),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_org_id ON users(org_id);

-- 3. Sales & Shipping Channels
CREATE TABLE IF NOT EXISTS channels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    code VARCHAR(100) NOT NULL, -- e.g. 'shopee_store_01', 'ghn_main'
    name VARCHAR(255) NOT NULL,
    platform_type VARCHAR(50) NOT NULL CHECK (
        platform_type IN ('SHOPEE', 'TIKTOK', 'LAZADA', 'TIKI', 'GHN', 'GHTK', 'VIETTEL_POST', 'NINJA_VAN', 'BANK_STATEMENT', 'CUSTOM_OMS')
    ),
    config_json JSONB DEFAULT '{}'::jsonb,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_channel_org_code UNIQUE (org_id, code)
);

CREATE INDEX IF NOT EXISTS idx_channels_org_type ON channels(org_id, platform_type);

-- 4. Statement Batches (Uploaded Excel / CSV Files)
CREATE TABLE IF NOT EXISTS statement_batches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    channel_id UUID REFERENCES channels(id) ON DELETE SET NULL,
    filename VARCHAR(255) NOT NULL,
    file_path VARCHAR(512) NOT NULL,
    file_size_bytes BIGINT DEFAULT 0,
    total_rows INT DEFAULT 0,
    status VARCHAR(50) DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'PARSING', 'PROCESSED', 'FAILED')),
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ
);

CREATE INDEX IF NOT EXISTS idx_batches_org_status ON statement_batches(org_id, status);

-- 5. Reconciliation Jobs
CREATE TABLE IF NOT EXISTS reconciliation_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
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

CREATE INDEX IF NOT EXISTS idx_recon_jobs_org_date ON reconciliation_jobs(org_id, date_from, date_to);

-- 6. Discrepancy Alerts
CREATE TABLE IF NOT EXISTS discrepancy_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    job_id UUID REFERENCES reconciliation_jobs(id) ON DELETE SET NULL,
    channel_code VARCHAR(100) NOT NULL,
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

CREATE INDEX IF NOT EXISTS idx_alerts_org_status ON discrepancy_alerts(org_id, status);
CREATE INDEX IF NOT EXISTS idx_alerts_order_id ON discrepancy_alerts(order_id);

-- 7. Alert Rules & Thresholds
CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    rule_code VARCHAR(100) NOT NULL,
    name VARCHAR(255) NOT NULL,
    conditions_json JSONB NOT NULL DEFAULT '{}'::jsonb,
    severity VARCHAR(20) DEFAULT 'HIGH',
    is_enabled BOOLEAN DEFAULT TRUE,
    notification_channels JSONB DEFAULT '["IN_APP"]'::jsonb,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 8. Cashflow Metrics Aggregation (Daily snapshot)
CREATE TABLE IF NOT EXISTS cashflow_timeline (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    record_date DATE NOT NULL,
    channel_id UUID REFERENCES channels(id) ON DELETE SET NULL,
    gross_sales NUMERIC(18, 2) DEFAULT 0,
    net_settlement NUMERIC(18, 2) DEFAULT 0,
    total_fees NUMERIC(18, 2) DEFAULT 0,
    cod_pending NUMERIC(18, 2) DEFAULT 0,
    discrepancy_sum NUMERIC(18, 2) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_cashflow_daily UNIQUE (org_id, record_date, channel_id)
);

CREATE INDEX IF NOT EXISTS idx_cashflow_date ON cashflow_timeline(org_id, record_date);
