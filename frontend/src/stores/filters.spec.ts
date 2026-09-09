import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useFilterStore } from './filters'

describe('Filters Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes with default values', () => {
    const store = useFilterStore()
    expect(store.selectedChannel).toBe('ALL')
    expect(store.selectedStatus).toBe('ALL')
    expect(store.searchQuery).toBe('')
    expect(store.dateRange.start).toBe('2026-08-01')
    expect(store.dateRange.end).toBe('2026-08-31')
  })

  it('resets filters properly', () => {
    const store = useFilterStore()
    store.selectedChannel = 'SHOPEE'
    store.selectedStatus = 'COD_MISMATCH'
    store.searchQuery = 'ORD123456'

    store.resetFilters()

    expect(store.selectedChannel).toBe('ALL')
    expect(store.selectedStatus).toBe('ALL')
    expect(store.searchQuery).toBe('')
  })
})
