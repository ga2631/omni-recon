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
