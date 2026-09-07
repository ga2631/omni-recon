import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ReconciliationRow } from '../types'

export const useReconciliationStore = defineStore('reconciliation', () => {
  const items = ref<ReconciliationRow[]>([
    {
      order_id: '240830SHOPEE88912',
      channel_code: 'SHOPEE',
      tracking_code: 'GHN992817263',
      expected_amount: 450000,
      actual_settlement: 405000,
      total_fee: 45000,
      carrier_cod: 450000,
      status: 'MATCHED',
      discrepancy: 0,
    },
    {
      order_id: '240830TIKTOK77123',
      channel_code: 'TIKTOK',
      tracking_code: 'GHTK11200921',
      expected_amount: 620000,
      actual_settlement: 540000,
      total_fee: 80000,
      carrier_cod: 600000,
      status: 'COD_MISMATCH',
      discrepancy: -20000,
    },
    {
      order_id: '240830SHOPEE99182',
      channel_code: 'SHOPEE',
      tracking_code: 'GHN88712300',
      expected_amount: 1200000,
      actual_settlement: 980000,
      total_fee: 220000,
      carrier_cod: 1200000,
      status: 'FEE_MISMATCH',
      discrepancy: -55000,
    },
  ])

  const isLoading = ref(false)

  async function fetchItems() {
    isLoading.value = true
    try {
      const res = await fetch('/api/v1/reconciliation/items')
      if (res.ok) {
        const json = await res.json()
        if (json.data) items.value = json.data
      }
    } catch (e) {
      console.warn('Using local fallback data', e)
    } finally {
      isLoading.value = false
    }
  }

  return {
    items,
    isLoading,
    fetchItems,
  }
})
