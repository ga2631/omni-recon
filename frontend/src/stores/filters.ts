import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useFilterStore = defineStore('filters', () => {
  const selectedChannel = ref<string>('ALL')
  const selectedStatus = ref<string>('ALL')
  const searchQuery = ref<string>('')
  const dateRange = ref<{ start: string; end: string }>({
    start: '2026-08-01',
    end: '2026-08-31',
  })

  function resetFilters() {
    selectedChannel.value = 'ALL'
    selectedStatus.value = 'ALL'
    searchQuery.value = ''
  }

  return {
    selectedChannel,
    selectedStatus,
    searchQuery,
    dateRange,
    resetFilters,
  }
})
