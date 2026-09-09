import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { StatementBatchItem, ChannelItem, UploadStatementResult } from '../types'
import { apiFetch } from '../utils/api'

export const useStatementsStore = defineStore('statements', () => {
  const batches = ref<StatementBatchItem[]>([])

  const channels = ref<ChannelItem[]>([
    {
      id: '00000000-0000-0000-0000-000000000010',
      code: 'shopee_official',
      name: 'Gian Hàng Shopee Mall',
      platform_type: 'SHOPEE',
      is_active: true,
    },
    {
      id: '00000000-0000-0000-0000-000000000011',
      code: 'tiktok_shop_main',
      name: 'TikTok Shop Flagship',
      platform_type: 'TIKTOK',
      is_active: true,
    },
    {
      id: '00000000-0000-0000-0000-000000000012',
      code: 'ghn_express',
      name: 'Giao Hàng Nhanh (GHN)',
      platform_type: 'GHN',
      is_active: true,
    },
  ])

  const isUploading = ref(false)
  const uploadProgress = ref(0)
  const lastUploadResult = ref<UploadStatementResult | null>(null)
  const errorMessage = ref<string | null>(null)
  const successMessage = ref<string | null>(null)

  async function fetchBatches() {
    try {
      const res = await apiFetch('/api/v1/statements/batches')
      if (res.ok) {
        const json = await res.json()
        if (json.data) {
          batches.value = json.data
        }
      }
    } catch (e) {
      console.warn('Using local fallback batches', e)
    }
  }

  async function fetchChannels() {
    try {
      const res = await apiFetch('/api/v1/channels')
      if (res.ok) {
        const json = await res.json()
        if (json.data) {
          channels.value = json.data
        }
      }
    } catch (e) {
      console.warn('Using local fallback channels', e)
    }
  }

  async function uploadStatement(
    file: File,
    platform: string,
    reportType: string = 'INCOME_STATEMENT',
    shopId?: string,
  ): Promise<boolean> {
    isUploading.value = true
    uploadProgress.value = 20
    errorMessage.value = null
    successMessage.value = null
    lastUploadResult.value = null

    try {
      const formData = new FormData()
      formData.append('file', file)
      formData.append('platform', platform)
      formData.append('report_type', reportType)
      if (shopId) {
        formData.append('shop_id', shopId)
      }

      uploadProgress.value = 60

      const res = await apiFetch('/api/v1/statements/upload', {
        method: 'POST',
        body: formData,
      })

      uploadProgress.value = 90

      const json = await res.json()

      if (!res.ok || !json.success) {
        throw new Error(json.error || json.message || 'Lỗi xử lý file tải lên')
      }

      lastUploadResult.value = json.data
      successMessage.value = json.message || 'Tải lên & Chuẩn hóa thành công!'
      uploadProgress.value = 100

      // Add to local batches list immediately
      batches.value.unshift({
        id: json.data.upload_log_id,
        filename: json.data.original_filename,
        channel_code: platform.toLowerCase(),
        platform,
        report_type: reportType,
        total_rows: json.data.total_rows,
        status: json.data.status,
        created_at: new Date().toISOString(),
      })

      return true
    } catch (err: any) {
      errorMessage.value = err.message || 'Không thể kết nối đến máy chủ API'
      return false
    } finally {
      isUploading.value = false
    }
  }

  function downloadSampleTemplate(platform: string = 'SHOPEE') {
    const csvContent =
      '\uFEFFMã đơn hàng,Ngày hoàn thành,Trạng thái đơn hàng,Tổng tiền hàng,Phí vận chuyển người mua trả,Trợ giá phí vận chuyển của Shopee,Phí vận chuyển thực tế,Phí thanh toán,Phí cố định,Phí Dịch Vụ,Số tiền chuyển cho Người bán\n' +
      '260908NORMAL01,2026-09-05 14:00,Hoàn thành,200000,15000,0,15000,8000,8000,10000,174000\n' +
      '260908SHIPPING,2026-09-06 10:30,Hoàn thành,300000,20000,0,45000,12000,12000,15000,236000\n' +
      '260908RETURN03,2026-09-07 09:15,Trả hàng hoàn tiền,150000,25000,15000,40000,0,0,0,0\n' +
      '260908HIDDEN04,2026-09-08 16:45,Hoàn thành,100000,15000,0,15000,4000,4000,5000,70000\n'

    const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
    const url = URL.createObjectURL(blob)
    const link = document.createElement('a')
    link.href = url
    const filename = platform === 'TIKTOK' ? 'mau_bang_ke_tiktok.csv' : 'mau_bang_ke_shopee.csv'
    link.setAttribute('download', filename)
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    URL.revokeObjectURL(url)
  }

  function formatVND(val: number | string | undefined): string {
    if (val === undefined || val === null) return '0 ₫'
    const num = typeof val === 'string' ? parseFloat(val) : val
    if (isNaN(num)) return '0 ₫'
    return new Intl.NumberFormat('vi-VN', {
      style: 'currency',
      currency: 'VND',
      maximumFractionDigits: 0,
    }).format(num)
  }

  return {
    batches,
    channels,
    isUploading,
    uploadProgress,
    lastUploadResult,
    errorMessage,
    successMessage,
    fetchBatches,
    fetchChannels,
    uploadStatement,
    downloadSampleTemplate,
    formatVND,
  }
})

