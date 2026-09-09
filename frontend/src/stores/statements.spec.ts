import { describe, it, expect, beforeEach } from 'vitest'
import { setActivePinia, createPinia } from 'pinia'
import { useStatementsStore } from './statements'

describe('Statements Store', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('formats Vietnamese currency amounts correctly', () => {
    const store = useStatementsStore()
    expect(store.formatVND(150000)).toContain('150.000')
    expect(store.formatVND('2500000')).toContain('2.500.000')
    expect(store.formatVND(0)).toContain('0')
    expect(store.formatVND(undefined)).toBe('0 ₫')
  })

  it('initializes default batches and channels list', () => {
    const store = useStatementsStore()
    expect(store.batches.length).toBeGreaterThanOrEqual(2)
    expect(store.channels.length).toBeGreaterThanOrEqual(3)
    expect(store.isUploading).toBe(false)
    expect(store.uploadProgress).toBe(0)
  })

  it('provides a downloadSampleTemplate function that handles template generation', () => {
    const store = useStatementsStore()
    expect(typeof store.downloadSampleTemplate).toBe('function')
  })
})

