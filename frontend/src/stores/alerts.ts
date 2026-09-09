import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { DiscrepancyAlertItem } from '../types'
import { apiFetch } from '../utils/api'

export const useAlertStore = defineStore('alerts', () => {
  const alerts = ref<DiscrepancyAlertItem[]>([])
  const isLoading = ref(false)

  async function fetchAlerts() {
    isLoading.value = true
    try {
      const res = await apiFetch('/api/v1/alerts')
      if (res.ok) {
        const json = await res.json()
        if (json.data) alerts.value = json.data
      }
    } catch (e) {
      console.error('Failed to fetch alerts', e)
    } finally {
      isLoading.value = false
    }
  }

  function resolveAlert(id: string) {
    const alert = alerts.value.find((a) => a.id === id)
    if (alert) {
      alert.status = 'RESOLVED'
    }
  }

  return {
    alerts,
    isLoading,
    fetchAlerts,
    resolveAlert,
  }
})

