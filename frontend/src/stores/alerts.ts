import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { DiscrepancyAlertItem } from '../types'

export const useAlertStore = defineStore('alerts', () => {
  const alerts = ref<DiscrepancyAlertItem[]>([
    {
      id: '00000000-0000-0000-0000-000000000071',
      order_id: '240830TIKTOK77123',
      channel_code: 'TIKTOK',
      alert_type: 'COD_MISMATCH',
      severity: 'CRITICAL',
      expected_amount: 620000,
      actual_amount: 600000,
      discrepancy_amount: -20000,
      status: 'OPEN',
      created_at: '2026-08-31T11:00:00Z',
      notes: 'ĐVVC GHTK thu hộ 600k nhưng giá trị đơn là 620k (lệch 20.000đ)',
    },
    {
      id: '00000000-0000-0000-0000-000000000072',
      order_id: '240830SHOPEE99182',
      channel_code: 'SHOPEE',
      alert_type: 'FEE_MISMATCH',
      severity: 'HIGH',
      expected_amount: 165000,
      actual_amount: 220000,
      discrepancy_amount: -55000,
      status: 'OPEN',
      created_at: '2026-08-31T11:05:00Z',
      notes: 'Phí sàn trừ 18.3% doanh thu (vượt mức cam kết 13.75%)',
    },
  ])

  function resolveAlert(id: string) {
    const alert = alerts.value.find((a) => a.id === id)
    if (alert) {
      alert.status = 'RESOLVED'
    }
  }

  return {
    alerts,
    resolveAlert,
  }
})
