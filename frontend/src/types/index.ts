export type PlatformType = 
  | 'SHOPEE' 
  | 'TIKTOK' 
  | 'LAZADA' 
  | 'TIKI' 
  | 'GHN' 
  | 'GHTK' 
  | 'VIETTEL_POST' 
  | 'BANK';

export type ReconStatus = 
  | 'MATCHED' 
  | 'COD_MISMATCH' 
  | 'FEE_MISMATCH' 
  | 'MISSING_SETTLEMENT' 
  | 'OVERDUE_PAYOUT';

export type AlertSeverity = 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL';

export interface ChannelItem {
  id: string;
  code: string;
  name: string;
  platform_type: PlatformType;
  is_active: boolean;
}

export interface ReconciliationRow {
  order_id: string;
  channel_code: string;
  tracking_code?: string;
  expected_amount: number;
  actual_settlement: number;
  total_fee: number;
  carrier_cod?: number;
  status: ReconStatus;
  discrepancy: number;
}

export interface DiscrepancyAlertItem {
  id: string;
  order_id: string;
  channel_code: string;
  alert_type: string;
  severity: AlertSeverity;
  expected_amount: number;
  actual_amount: number;
  discrepancy_amount: number;
  status: 'OPEN' | 'INVESTIGATING' | 'RESOLVED';
  created_at: string;
  notes: string;
}

export interface DashboardMetrics {
  summary: {
    gross_revenue: number;
    net_settled: number;
    total_platform_fees: number;
    cod_pending: number;
    total_discrepancy_amount: number;
    discrepancy_count: number;
  };
  channel_breakdown: Array<{
    channel: string;
    revenue: number;
    fees: number;
    fee_rate: number;
  }>;
  cashflow_trend: Array<{
    date: string;
    expected: number;
    actual: number;
  }>;
}

export interface FinancialSanitySummary {
  total_rows: number;
  total_gross: number | string;
  total_net: number | string;
  total_fees: number | string;
  balanced_rows: number;
  discrepant_rows: number;
}

export interface StandardSettlementRecord {
  order_id: string;
  platform: string;
  payout_id?: string;
  transaction_type: string;
  order_status: string;
  buyer_username?: string;
  tracking_number?: string;
  gross_amount: number | string;
  seller_discount: number | string;
  platform_voucher: number | string;
  buyer_shipping_fee: number | string;
  seller_shipping_fee: number | string;
  shipping_subsidy: number | string;
  commission_fee: number | string;
  service_fee: number | string;
  payment_fee: number | string;
  affiliate_commission_fee: number | string;
  other_fees: number | string;
  net_settlement: number | string;
  ordered_at?: string;
  delivered_at?: string;
  settled_at?: string;
  raw_attributes?: Record<string, any>;
  raw_fee_breakdown?: Record<string, any>;
}

export interface UploadStatementResult {
  upload_log_id: string;
  merchant_id: string;
  shop_id?: string;
  platform: string;
  report_type: string;
  original_filename: string;
  file_path: string;
  file_hash: string;
  file_size_bytes: number;
  total_rows: number;
  successful_rows: number;
  failed_rows: number;
  status: string;
  sanity_check?: FinancialSanitySummary;
  sample_records: StandardSettlementRecord[];
}

export interface StatementBatchItem {
  id: string;
  filename: string;
  channel_code?: string;
  platform?: string;
  report_type?: string;
  total_rows: number;
  status: string;
  created_at: string;
}
