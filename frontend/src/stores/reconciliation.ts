import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ReconciliationRow } from '../types'
import { apiFetch } from '../utils/api'

export const useReconciliationStore = defineStore('reconciliation', () => {
  const items = ref<ReconciliationRow[]>([])
  const isLoading = ref(false)

  async function fetchItems() {
    isLoading.value = true
    try {
      const res = await apiFetch('/api/v1/reconciliation/items')
      if (res.ok) {
        const json = await res.json()
        if (json.data) items.value = json.data
      }
    } catch (e) {
      console.error('Failed to fetch reconciliation items', e)
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
