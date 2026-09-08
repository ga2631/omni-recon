-- ==============================================================================
-- OmniRecon Seed Data for Local Evaluation
-- ==============================================================================

-- 1. Create Default Merchant / Organization
INSERT INTO merchants (id, name, currency)
VALUES ('00000000-0000-0000-0000-000000000001', 'OmniRecon Demo Store', 'VND')
ON CONFLICT (id) DO NOTHING;

-- 2. Create Default Admin User (Password: Admin@123456)
-- Bcrypt hash for 'Admin@123456': $2a$12$e6LpIu5hLhWwR8b4h4M6pOp4qZ9P.oY8D.3wFf5s9c9M2pP9bQpOu
INSERT INTO users (id, merchant_id, email, password_hash, full_name, role, is_active)
VALUES (
    '00000000-0000-0000-0000-000000000002',
    '00000000-0000-0000-0000-000000000001',
    'admin@omnirecon.local',
    '$2a$12$e6LpIu5hLhWwR8b4h4M6pOp4qZ9P.oY8D.3wFf5s9c9M2pP9bQpOu',
    'System Administrator',
    'ADMIN',
    TRUE
)
ON CONFLICT (email) DO NOTHING;

-- 3. Create Sample Shops / Channels
INSERT INTO shops (id, merchant_id, code, name, platform, shop_identifier, config_json)
VALUES 
(
    '00000000-0000-0000-0000-000000000010',
    '00000000-0000-0000-0000-000000000001',
    'shopee_official',
    'Gian Hàng Shopee Mall',
    'SHOPEE',
    'vn_shopee_1001',
    '{"commission_rate": 0.04, "payment_fee_rate": 0.045, "service_fee_rate": 0.06}'::jsonb
),
(
    '00000000-0000-0000-0000-000000000011',
    '00000000-0000-0000-0000-000000000001',
    'tiktok_shop_main',
    'TikTok Shop Flagship',
    'TIKTOK',
    'vn_tiktok_2002',
    '{"commission_rate": 0.05, "payment_fee_rate": 0.035}'::jsonb
),
(
    '00000000-0000-0000-0000-000000000012',
    '00000000-0000-0000-0000-000000000001',
    'ghn_express',
    'Giao Hàng Nhanh (GHN)',
    'GHN',
    'ghn_partner_3003',
    '{"max_cod_hold_days": 7}'::jsonb
)
ON CONFLICT (merchant_id, code) DO NOTHING;

-- 4. Create Sample Alert Rules
INSERT INTO alert_rules (id, merchant_id, rule_code, name, conditions_json, severity, is_enabled, notification_channels)
VALUES
(
    '00000000-0000-0000-0000-000000000020',
    '00000000-0000-0000-0000-000000000001',
    'COD_MISMATCH_THRESHOLD',
    'Cảnh báo Lệch tiền COD trên 10.000đ',
    '{"threshold_amount": 10000, "compare_operator": "GREATER_THAN"}'::jsonb,
    'CRITICAL',
    TRUE,
    '["IN_APP", "TELEGRAM"]'::jsonb
),
(
    '00000000-0000-0000-0000-000000000021',
    '00000000-0000-0000-0000-000000000001',
    'OVERDUE_PAYOUT_THRESHOLD',
    'Cảnh báo Sàn giữ tiền quá 7 ngày kể từ khi giao thành công',
    '{"max_hold_days": 7}'::jsonb,
    'HIGH',
    TRUE,
    '["IN_APP"]'::jsonb
)
ON CONFLICT DO NOTHING;
