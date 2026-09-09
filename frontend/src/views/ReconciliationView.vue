<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { RouterLink } from 'vue-router'
import { useReconciliationStore } from '../stores/reconciliation'
import { useFilterStore } from '../stores/filters'
import FilterBar from '../components/common/FilterBar.vue'
import StatusBadge from '../components/common/StatusBadge.vue'
import {
  Download,
  RefreshCw,
  FileText,
  UploadCloud,
  CheckCircle2,
  AlertTriangle,
  FileSpreadsheet,
  Store,
  Layers,
  Code,
  Eye,
  X,
  Copy,
  Check,
  Info,
  Calendar,
  Truck,
  DollarSign,
  TrendingDown,
  ExternalLink,
} from 'lucide-vue-next'
import type { ReconciliationRow } from '../types'

const reconStore = useReconciliationStore()
const filterStore = useFilterStore()

const selectedRow = ref<ReconciliationRow | null>(null)
const showRawModal = ref(false)
const copiedRaw = ref(false)
const viewMode = ref<'FULL' | 'COMPACT'>('FULL')

onMounted(() => {
  reconStore.fetchItems()
})

const filteredItems = computed(() => {
  return reconStore.items.filter((item) => {
    // Channel filter
    if (filterStore.selectedChannel !== 'ALL') {
      if (item.channel_code.toUpperCase() !== filterStore.selectedChannel.toUpperCase()) {
        return false
      }
    }
    // Status filter
    if (filterStore.selectedStatus !== 'ALL') {
      if (item.status !== filterStore.selectedStatus) {
        return false
      }
    }
    // Search query
    if (filterStore.searchQuery.trim()) {
      const q = filterStore.searchQuery.toLowerCase().trim()
      const matchOrder = item.order_id.toLowerCase().includes(q)
      const matchTracking = item.tracking_code?.toLowerCase().includes(q) || false
      const matchShop = item.shop_name?.toLowerCase().includes(q) || item.shop_code?.toLowerCase().includes(q) || false
      const matchFile = item.source_file?.toLowerCase().includes(q) || false
      const matchBuyer = item.buyer_username?.toLowerCase().includes(q) || false
      if (!matchOrder && !matchTracking && !matchShop && !matchFile && !matchBuyer) return false
    }
    return true
  })
})

const stats = computed(() => {
  const total = reconStore.items.length
  const matched = reconStore.items.filter((i) => i.status === 'MATCHED').length
  const discrepancies = total - matched
  const totalGross = reconStore.items.reduce((acc, i) => acc + (i.gross_amount || i.expected_amount || 0), 0)
  const totalNet = reconStore.items.reduce((acc, i) => acc + (i.net_settlement || i.actual_settlement || 0), 0)
  const totalFees = reconStore.items.reduce((acc, i) => acc + (i.total_fee || 0), 0)
  return { total, matched, discrepancies, totalGross, totalNet, totalFees }
})

function formatVND(amount: number | undefined | null) {
  if (amount === undefined || amount === null) return '0 ₫'
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(amount)
}

function formatDateTime(dateStr?: string) {
  if (!dateStr) return '---'
  try {
    const d = new Date(dateStr)
    return d.toLocaleString('vi-VN', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    })
  } catch {
    return dateStr
  }
}

function getPlatformBadge(platform: string) {
  const p = (platform || '').toUpperCase()
  if (p === 'SHOPEE') return { label: 'Shopee', bg: 'bg-orange-500/15 text-orange-400 border-orange-500/30' }
  if (p === 'TIKTOK') return { label: 'TikTok Shop', bg: 'bg-rose-500/15 text-rose-400 border-rose-500/30' }
  if (p === 'LAZADA') return { label: 'Lazada', bg: 'bg-blue-500/15 text-blue-400 border-blue-500/30' }
  if (p === 'GHN') return { label: 'GHN Express', bg: 'bg-amber-500/15 text-amber-400 border-amber-500/30' }
  if (p === 'GHTK') return { label: 'GHTK', bg: 'bg-emerald-500/15 text-emerald-400 border-emerald-500/30' }
  return { label: platform || 'Khác', bg: 'bg-slate-700/30 text-slate-300 border-slate-600/30' }
}

function openRawModal(row: ReconciliationRow) {
  selectedRow.value = row
  showRawModal.value = true
  copiedRaw.value = false
}

function closeRawModal() {
  showRawModal.value = false
  selectedRow.value = null
}

function copyJson(data: any) {
  navigator.clipboard.writeText(JSON.stringify(data, null, 2))
  copiedRaw.value = true
  setTimeout(() => {
    copiedRaw.value = false
  }, 2000)
}

function exportToCSV() {
  if (filteredItems.value.length === 0) return

  const headers = [
    'Mã Đơn Hàng',
    'Kênh Bán',
    'Gian Hàng',
    'File Bảng Kê Nguồn',
    'Loại Báo Cáo Nguồn',
    'Mã Vận Đơn',
    'Người Mua',
    'Ngày Hoàn Thành',
    'Trạng Thái Đơn',
    'Doanh Thu Gốc',
    'Phí VC Người Mua',
    'Trợ Giá VC Sàn',
    'Phí VC Thực Tế',
    'Phí Thanh Toán',
    'Phí Cố Định',
    'Phí Dịch Vụ',
    'Phí Tiếp Thị & Khác',
    'Tổng Phí Sàn',
    'Thực Nhận (Net)',
    'Chênh Lệch Đối Soát',
    'Trạng Thái Đối Soát',
  ]

  const rows = filteredItems.value.map((item) => [
    `"${item.order_id}"`,
    `"${item.channel_code}"`,
    `"${item.shop_name || item.shop_code || ''}"`,
    `"${item.source_file || ''}"`,
    `"${item.source_report_type || ''}"`,
    `"${item.tracking_code || ''}"`,
    `"${item.buyer_username || ''}"`,
    `"${item.settled_at || ''}"`,
    `"${item.order_status}"`,
    item.gross_amount || item.expected_amount || 0,
    item.buyer_shipping_fee || 0,
    item.shipping_subsidy || 0,
    item.seller_shipping_fee || 0,
    item.payment_fee || 0,
    item.commission_fee || 0,
    item.service_fee || 0,
    (item.affiliate_commission_fee || 0) + (item.other_fees || 0),
    item.total_fee || 0,
    item.net_settlement || item.actual_settlement || 0,
    item.discrepancy || 0,
    `"${item.status}"`,
  ])

  const csvContent = '\uFEFF' + [headers.join(','), ...rows.map((e) => e.join(','))].join('\n')
  const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' })
  const url = URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.setAttribute('href', url)
  link.setAttribute('download', `Doi_Soat_Chi_Tiet_Don_Hang_${new Date().toISOString().slice(0, 10)}.csv`)
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}
</script>

<template>
  <div class="space-y-6 max-w-[1600px] mx-auto pb-16">
    <!-- Header -->
    <div class="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight flex items-center gap-2.5">
          <FileSpreadsheet class="w-7 h-7 text-emerald-400" />
          Bảng Đối soát Chi tiết Đơn hàng
        </h2>
        <p class="text-xs text-slate-400 mt-1">
          So khớp toàn diện giữa Dữ liệu Bảng kê Chuẩn hóa (Silver Layer), Đơn nội bộ & Nguồn gốc File Tải lên
        </p>
      </div>

      <div class="flex items-center gap-3 flex-wrap">
        <!-- View Mode Switcher -->
        <div class="bg-slate-900 p-1 rounded-xl border border-slate-800 flex items-center gap-1 text-xs">
          <button
            @click="viewMode = 'FULL'"
            class="px-3 py-1.5 rounded-lg font-semibold transition-all"
            :class="viewMode === 'FULL' ? 'bg-emerald-500 text-slate-950 font-bold shadow-md' : 'text-slate-400 hover:text-white'"
          >
            Toàn Bộ Cột Chuẩn Hóa
          </button>
          <button
            @click="viewMode = 'COMPACT'"
            class="px-3 py-1.5 rounded-lg font-semibold transition-all"
            :class="viewMode === 'COMPACT' ? 'bg-emerald-500 text-slate-950 font-bold shadow-md' : 'text-slate-400 hover:text-white'"
          >
            Thu Gọn
          </button>
        </div>

        <button
          @click="reconStore.fetchItems"
          class="p-2.5 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 rounded-xl text-xs transition-all flex items-center gap-1.5"
          title="Làm mới dữ liệu"
        >
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': reconStore.isLoading }" />
        </button>

        <button
          @click="exportToCSV"
          :disabled="filteredItems.length === 0"
          class="px-4 py-2 bg-slate-800 hover:bg-slate-700 disabled:opacity-50 text-emerald-400 border border-emerald-500/30 hover:border-emerald-500/60 rounded-xl text-xs font-bold transition-all shadow-md flex items-center gap-2"
        >
          <Download class="w-4 h-4" />
          <span>Xuất Báo Cáo Excel (.CSV)</span>
        </button>

        <RouterLink
          to="/upload"
          class="px-4 py-2 bg-emerald-500 hover:bg-emerald-400 text-slate-950 rounded-xl text-xs font-bold transition-all shadow-md shadow-emerald-500/20 flex items-center gap-2"
        >
          <UploadCloud class="w-4 h-4" />
          <span>Tải Lên Bảng Kê</span>
        </RouterLink>
      </div>
    </div>

    <!-- Quick Stats Cards -->
    <div v-if="reconStore.items.length > 0" class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3.5">
      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-slate-400 flex items-center gap-1.5">
          <FileText class="w-3.5 h-3.5 text-slate-400" /> Tổng Số Đơn
        </div>
        <div class="text-lg font-bold text-white mt-1">{{ stats.total.toLocaleString() }} đơn</div>
      </div>

      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-emerald-400 flex items-center gap-1.5">
          <CheckCircle2 class="w-3.5 h-3.5 text-emerald-400" /> Khớp Chuẩn (100%)
        </div>
        <div class="text-lg font-bold text-emerald-400 mt-1">{{ stats.matched.toLocaleString() }} đơn</div>
      </div>

      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-amber-400 flex items-center gap-1.5">
          <AlertTriangle class="w-3.5 h-3.5 text-amber-400" /> Lệch Phí / COD
        </div>
        <div class="text-lg font-bold text-amber-400 mt-1">{{ stats.discrepancies.toLocaleString() }} đơn</div>
      </div>

      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-slate-300 flex items-center gap-1.5">
          <DollarSign class="w-3.5 h-3.5 text-emerald-400" /> Tổng Doanh Thu Gốc
        </div>
        <div class="text-base font-bold text-white mt-1 truncate">{{ formatVND(stats.totalGross) }}</div>
      </div>

      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-rose-400 flex items-center gap-1.5">
          <TrendingDown class="w-3.5 h-3.5 text-rose-400" /> Tổng Phí Sàn Cấn Trừ
        </div>
        <div class="text-base font-bold text-rose-400 mt-1 truncate">{{ formatVND(stats.totalFees) }}</div>
      </div>

      <div class="glass-panel p-4 rounded-2xl border border-slate-800 bg-slate-900/60 shadow-lg">
        <div class="text-[11px] font-semibold text-cyan-400 flex items-center gap-1.5">
          <CheckCircle2 class="w-3.5 h-3.5 text-cyan-400" /> Tiền Thực Nhận (Net)
        </div>
        <div class="text-base font-bold text-cyan-300 mt-1 truncate">{{ formatVND(stats.totalNet) }}</div>
      </div>
    </div>

    <!-- Quick Stats Cards (when data exists) -->
    <div v-if="reconStore.items.length > 0" class="grid grid-cols-2 sm:grid-cols-4 gap-4">
      <div class="glass-panel p-3.5 rounded-xl">
        <div class="text-[11px] font-medium text-slate-400">Tổng số đơn</div>
        <div class="text-lg font-bold text-white mt-1">{{ stats.total }} đơn</div>
      </div>
      <div class="glass-panel p-3.5 rounded-xl">
        <div class="text-[11px] font-medium text-emerald-400 flex items-center gap-1">
          <CheckCircle2 class="w-3.5 h-3.5" />
          Khớp chuẩn
        </div>
        <div class="text-lg font-bold text-emerald-400 mt-1">{{ stats.matched }} đơn</div>
      </div>
      <div class="glass-panel p-3.5 rounded-xl">
        <div class="text-[11px] font-medium text-amber-400 flex items-center gap-1">
          <AlertTriangle class="w-3.5 h-3.5" />
          Lệch phí / COD
        </div>
        <div class="text-lg font-bold text-amber-400 mt-1">{{ stats.discrepancies }} đơn</div>
      </div>
      <div class="glass-panel p-3.5 rounded-xl">
        <div class="text-[11px] font-medium text-slate-400">Thực nhận về</div>
        <div class="text-lg font-bold text-brand-400 mt-1">{{ formatVND(stats.totalNet) }}</div>
      </div>
    </div>

    <!-- Reactive Filter Bar -->
    <FilterBar />

    <!-- Reconciliation Table -->
    <div class="glass-panel rounded-2xl border border-slate-800/80 shadow-2xl overflow-hidden">
      <!-- Top Table Info Bar -->
      <div class="p-4 bg-slate-900/80 border-b border-slate-800 flex flex-col sm:flex-row sm:items-center justify-between gap-3 text-xs text-slate-400">
        <div class="flex items-center gap-2">
          <span class="font-bold text-white flex items-center gap-1.5">
            <Store class="w-4 h-4 text-emerald-400" />
            Danh sách đối soát đơn hàng:
          </span>
          <span class="bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded-md font-mono font-bold">
            {{ filteredItems.length }} / {{ reconStore.items.length }} đơn
          </span>
        </div>

        <div class="flex items-center gap-4 flex-wrap text-[11px]">
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-emerald-400"></span> Khớp chuẩn
          </span>
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-rose-400"></span> Lệch phí sàn
          </span>
          <span class="flex items-center gap-1">
            <span class="w-2 h-2 rounded-full bg-amber-400"></span> Lệch COD
          </span>
          <span class="text-slate-500">|</span>
          <span class="text-slate-400 italic">Đơn vị tiền tệ: VND</span>
        </div>
      </div>

      <!-- Main Responsive Table Container -->
      <div class="overflow-x-auto">
        <table class="w-full text-left text-xs text-slate-300 whitespace-nowrap">
          <thead class="bg-slate-800/90 text-[11px] text-slate-400 uppercase font-bold border-b border-slate-700/60 sticky top-0 z-10 backdrop-blur-md">
            <tr>
              <!-- Nguồn dữ liệu & Định danh -->
              <th class="p-3.5 text-center">STT</th>
              <th class="p-3.5">Nguồn & Gian Hàng</th>
              <th class="p-3.5">Mã Đơn Hàng</th>
              <th class="p-3.5">Mã Vận Đơn</th>
              <th class="p-3.5">Ngày Hoàn Thành</th>
              <th class="p-3.5 text-center">Trạng Thái Đơn</th>

              <!-- Dòng tiền Chuẩn Hóa -->
              <th class="p-3.5 text-right bg-slate-900/40">Doanh Thu Gốc</th>
              <template v-if="viewMode === 'FULL'">
                <th class="p-3.5 text-right">Phí VC N.Mua</th>
                <th class="p-3.5 text-right">Trợ Giá VC</th>
                <th class="p-3.5 text-right">Phí VC Thực Tế</th>
                <th class="p-3.5 text-right">Phí Thanh Toán</th>
                <th class="p-3.5 text-right">Phí Cố Định</th>
                <th class="p-3.5 text-right">Phí Dịch Vụ</th>
                <th class="p-3.5 text-right">Phí Khác</th>
              </template>
              <th class="p-3.5 text-right text-rose-300 bg-rose-500/5">Tổng Phí Sàn</th>
              <th class="p-3.5 text-right text-cyan-300 bg-cyan-500/5 font-bold">Thực Nhận (Net)</th>

              <!-- Đối Soát & Chênh Lệch -->
              <th class="p-3.5 text-right font-bold">Chênh Lệch</th>
              <th class="p-3.5 text-center">Trạng Thái Đối Soát</th>

              <!-- Nguồn Gốc Chi Tiết -->
              <th class="p-3.5 text-center">Dữ Liệu Gốc</th>
            </tr>
          </thead>

          <tbody class="divide-y divide-slate-800/60 font-mono text-xs">
            <template v-if="filteredItems.length > 0">
              <tr
                v-for="(item, index) in filteredItems"
                :key="item.order_id"
                class="hover:bg-slate-800/40 transition-colors group"
              >
                <!-- STT -->
                <td class="p-3 text-center text-slate-500 font-sans text-[11px]">{{ index + 1 }}</td>

                <!-- Nguồn & Gian Hàng (Data Origin / Lineage) -->
                <td class="p-3 font-sans">
                  <div class="flex flex-col gap-1">
                    <div class="flex items-center gap-1.5">
                      <span
                        class="px-2 py-0.5 rounded-md text-[10px] font-bold border"
                        :class="getPlatformBadge(item.channel_code).bg"
                      >
                        {{ getPlatformBadge(item.channel_code).label }}
                      </span>
                      <span class="font-semibold text-white text-xs truncate max-w-[140px]" :title="item.shop_name || item.shop_code">
                        {{ item.shop_name || item.shop_code || '---' }}
                      </span>
                    </div>

                    <!-- Source File Lineage Info -->
                    <div class="flex items-center gap-1 text-[10px] text-slate-400" :title="`Nguồn: ${item.source_file || 'File bảng kê'} (${item.source_report_type || 'INCOME'})`">
                      <FileSpreadsheet class="w-3 h-3 text-emerald-400 flex-shrink-0" />
                      <span class="truncate max-w-[170px] text-slate-400 font-mono">
                        {{ item.source_file || 'Bảng kê sao kê' }}
                      </span>
                    </div>
                  </div>
                </td>

                <!-- Mã Đơn Hàng -->
                <td class="p-3 font-sans">
                  <div class="flex flex-col">
                    <span class="font-semibold text-white font-mono flex items-center gap-1 text-xs">
                      <FileText class="w-3.5 h-3.5 text-slate-500 flex-shrink-0" />
                      {{ item.order_id }}
                    </span>
                    <span v-if="item.buyer_username" class="text-[10px] text-slate-400 mt-0.5">
                      KH: {{ item.buyer_username }}
                    </span>
                  </div>
                </td>

                <!-- Mã Vận Đơn -->
                <td class="p-3 font-sans text-[11px]">
                  <div v-if="item.tracking_code" class="flex items-center gap-1 text-slate-300 font-mono">
                    <Truck class="w-3.5 h-3.5 text-amber-400 flex-shrink-0" />
                    <span>{{ item.tracking_code }}</span>
                  </div>
                  <span v-else class="text-slate-600 font-mono">---</span>
                </td>

                <!-- Ngày Hoàn Thành / Quyết Toán -->
                <td class="p-3 font-sans text-slate-400 text-[11px]">
                  <div class="flex items-center gap-1">
                    <Calendar class="w-3.5 h-3.5 text-slate-500 flex-shrink-0" />
                    <span>{{ formatDateTime(item.settled_at) }}</span>
                  </div>
                </td>

                <!-- Trạng Thái Đơn Sàn -->
                <td class="p-3 text-center font-sans">
                  <span
                    class="px-2 py-0.5 rounded text-[10px] font-bold border inline-block"
                    :class="
                      (item.order_status || '').toUpperCase().includes('RETURN') || item.order_status === 'RETURNED'
                        ? 'bg-rose-500/15 text-rose-300 border-rose-500/30'
                        : (item.order_status || '').toUpperCase().includes('CANCEL')
                        ? 'bg-amber-500/15 text-amber-300 border-amber-500/30'
                        : 'bg-emerald-500/15 text-emerald-300 border-emerald-500/30'
                    "
                  >
                    {{ item.order_status || 'COMPLETED' }}
                  </span>
                </td>

                <!-- Doanh Thu Gốc -->
                <td class="p-3 text-right font-semibold text-emerald-400 bg-slate-900/30">
                  {{ formatVND(item.gross_amount || item.expected_amount) }}
                </td>

                <!-- Full Standardized Fee Breakdown Columns -->
                <template v-if="viewMode === 'FULL'">
                  <!-- Phí VC Người Mua -->
                  <td class="p-3 text-right text-slate-300">
                    {{ formatVND(item.buyer_shipping_fee) }}
                  </td>

                  <!-- Trợ Giá VC Sàn -->
                  <td class="p-3 text-right text-teal-400">
                    {{ formatVND(item.shipping_subsidy) }}
                  </td>

                  <!-- Phí VC Thực Tế (Người Bán Chịu) -->
                  <td class="p-3 text-right text-amber-300">
                    {{ formatVND(item.seller_shipping_fee) }}
                  </td>

                  <!-- Phí Thanh Toán -->
                  <td class="p-3 text-right text-rose-400">
                    {{ formatVND(item.payment_fee) }}
                  </td>

                  <!-- Phí Cố Định / Hoa Hồng -->
                  <td class="p-3 text-right text-rose-400">
                    {{ formatVND(item.commission_fee) }}
                  </td>

                  <!-- Phí Dịch Vụ -->
                  <td class="p-3 text-right text-rose-400">
                    {{ formatVND(item.service_fee) }}
                  </td>

                  <!-- Phí Khác (Affiliate + Other) -->
                  <td class="p-3 text-right text-slate-400">
                    {{ formatVND((item.affiliate_commission_fee || 0) + (item.other_fees || 0)) }}
                  </td>
                </template>

                <!-- Tổng Phí Sàn Cấn Trừ -->
                <td class="p-3 text-right font-bold text-rose-400 bg-rose-500/5">
                  {{ formatVND(item.total_fee) }}
                </td>

                <!-- Thực Nhận Về (Net) -->
                <td class="p-3 text-right font-bold text-cyan-300 bg-cyan-500/5">
                  {{ formatVND(item.net_settlement || item.actual_settlement) }}
                </td>

                <!-- Chênh Lệch Đối Soát -->
                <td
                  class="p-3 text-right font-bold"
                  :class="item.discrepancy < 0 ? 'text-rose-400 font-bold' : item.discrepancy > 0 ? 'text-emerald-400 font-bold' : 'text-slate-400'"
                >
                  {{ formatVND(item.discrepancy) }}
                </td>

                <!-- Trạng Thái Đối Soát -->
                <td class="p-3 text-center font-sans">
                  <StatusBadge :status="item.status" />
                </td>

                <!-- Chi Tiết Thuộc Tính Gốc (Raw Lineage Modal Trigger) -->
                <td class="p-3 text-center font-sans">
                  <button
                    @click="openRawModal(item)"
                    class="px-2.5 py-1 bg-slate-800 hover:bg-emerald-500/20 hover:text-emerald-300 text-slate-300 border border-slate-700 hover:border-emerald-500/40 rounded-lg text-[11px] font-semibold transition-all inline-flex items-center gap-1.5 shadow-sm"
                    title="Xem toàn bộ dữ liệu gốc & xuất xứ tệp bảng kê"
                  >
                    <Eye class="w-3.5 h-3.5" />
                    <span>Chi Tiết Gốc</span>
                  </button>
                </td>
              </tr>
            </template>

            <!-- Empty State -->
            <tr v-else>
              <td :colspan="viewMode === 'FULL' ? 18 : 11" class="px-6 py-16 text-center text-slate-400 font-sans">
                <div class="flex flex-col items-center justify-center gap-3">
                  <div class="w-14 h-14 rounded-2xl bg-slate-800/80 border border-slate-700 flex items-center justify-center text-slate-400 shadow-lg">
                    <FileSpreadsheet class="w-7 h-7 text-emerald-400" />
                  </div>
                  <div class="text-base font-bold text-white">
                    {{ reconStore.items.length === 0 ? 'Chưa có dữ liệu đối soát đơn hàng' : 'Không tìm thấy đơn hàng khớp với bộ lọc' }}
                  </div>
                  <p class="text-xs text-slate-400 max-w-md">
                    {{ reconStore.items.length === 0
                      ? 'Vui lòng tải lên bảng kê đối soát từ Shopee, TikTok Shop, Lazada hoặc ĐVVC để hệ thống tự động chuẩn hóa và đối soát.'
                      : 'Hãy thử thay đổi từ khóa tìm kiếm (mã đơn, mã vận đơn, tên file nguồn) hoặc chọn lại các điều kiện lọc.' }}
                  </p>
                  <RouterLink
                    v-if="reconStore.items.length === 0"
                    to="/upload"
                    class="mt-3 inline-flex items-center gap-2 px-5 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-slate-950 text-xs font-bold rounded-xl shadow-lg shadow-emerald-500/20 transition-all"
                  >
                    <UploadCloud class="w-4 h-4" />
                    <span>Tải lên Bảng Kê Ngay</span>
                  </RouterLink>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Raw Lineage & Full Attributes Detail Modal -->
    <div
      v-if="showRawModal && selectedRow"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-slate-950/80 backdrop-blur-sm"
      @click.self="closeRawModal"
    >
      <div class="glass-panel border-slate-700 bg-slate-900 rounded-2xl max-w-3xl w-full p-6 shadow-2xl space-y-5 max-h-[90vh] overflow-y-auto">
        <!-- Modal Header -->
        <div class="flex items-center justify-between border-b border-slate-800 pb-4">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-emerald-500/20 text-emerald-400 rounded-xl">
              <Code class="w-5 h-5" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white flex items-center gap-2">
                Chi Tiết Gốc & Xuất Xứ Dữ Liệu (Raw Lineage)
              </h3>
              <p class="text-xs text-slate-400">
                Mã đơn hàng: <span class="font-mono font-bold text-emerald-400">{{ selectedRow.order_id }}</span>
              </p>
            </div>
          </div>

          <button
            @click="closeRawModal"
            class="p-1.5 rounded-xl text-slate-400 hover:text-white hover:bg-slate-800 transition-all"
          >
            <X class="w-5 h-5" />
          </button>
        </div>

        <!-- 1. Data Origin & Metadata Card -->
        <div class="p-4 bg-slate-950/70 border border-slate-800 rounded-xl space-y-2.5 text-xs">
          <div class="font-bold text-slate-200 uppercase tracking-wider text-[11px] flex items-center gap-1.5 text-emerald-400">
            <Store class="w-3.5 h-3.5" /> Thông Tin Nguồn Gốc Dữ Liệu (Data Provenance)
          </div>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-slate-300">
            <div class="flex items-center justify-between bg-slate-900 p-2.5 rounded-lg border border-slate-800/80">
              <span class="text-slate-400">Kênh Bán / Sàn:</span>
              <span class="font-bold text-white">{{ selectedRow.channel_code }}</span>
            </div>
            <div class="flex items-center justify-between bg-slate-900 p-2.5 rounded-lg border border-slate-800/80">
              <span class="text-slate-400">Gian Hàng:</span>
              <span class="font-semibold text-white">{{ selectedRow.shop_name || selectedRow.shop_code || '---' }}</span>
            </div>
            <div class="flex items-center justify-between bg-slate-900 p-2.5 rounded-lg border border-slate-800/80">
              <span class="text-slate-400">Tệp Bảng Kê Gốc:</span>
              <span class="font-mono font-semibold text-emerald-300 truncate max-w-[180px]" :title="selectedRow.source_file">
                {{ selectedRow.source_file || 'Bảng kê sao kê' }}
              </span>
            </div>
            <div class="flex items-center justify-between bg-slate-900 p-2.5 rounded-lg border border-slate-800/80">
              <span class="text-slate-400">Loại Báo Cáo Nguồn:</span>
              <span class="font-semibold text-cyan-300">{{ selectedRow.source_report_type || 'INCOME_STATEMENT' }}</span>
            </div>
            <div v-if="selectedRow.upload_log_id" class="flex items-center justify-between bg-slate-900 p-2.5 rounded-lg border border-slate-800/80 sm:col-span-2">
              <span class="text-slate-400">Mã Lô Bronze Log:</span>
              <span class="font-mono text-slate-300 text-[11px]">{{ selectedRow.upload_log_id }}</span>
            </div>
          </div>
        </div>

        <!-- 2. Financial Summary Card -->
        <div class="p-4 bg-slate-950/70 border border-slate-800 rounded-xl space-y-2.5 text-xs">
          <div class="font-bold text-slate-200 uppercase tracking-wider text-[11px] flex items-center gap-1.5 text-cyan-400">
            <DollarSign class="w-3.5 h-3.5" /> Bóc Tách Các Khoản Dòng Tiền & Biểu Phí Chuẩn Hóa
          </div>
          <div class="grid grid-cols-2 sm:grid-cols-4 gap-2.5 text-center font-mono">
            <div class="bg-slate-900 p-2.5 rounded-lg border border-slate-800">
              <p class="text-[10px] text-slate-400 font-sans font-semibold">Doanh Thu Gốc</p>
              <p class="text-xs font-bold text-emerald-400 mt-0.5">{{ formatVND(selectedRow.gross_amount || selectedRow.expected_amount) }}</p>
            </div>
            <div class="bg-slate-900 p-2.5 rounded-lg border border-slate-800">
              <p class="text-[10px] text-slate-400 font-sans font-semibold">Tổng Phí Sàn</p>
              <p class="text-xs font-bold text-rose-400 mt-0.5">{{ formatVND(selectedRow.total_fee) }}</p>
            </div>
            <div class="bg-slate-900 p-2.5 rounded-lg border border-slate-800">
              <p class="text-[10px] text-slate-400 font-sans font-semibold">Thực Nhận (Net)</p>
              <p class="text-xs font-bold text-cyan-300 mt-0.5">{{ formatVND(selectedRow.net_settlement || selectedRow.actual_settlement) }}</p>
            </div>
            <div class="bg-slate-900 p-2.5 rounded-lg border border-slate-800">
              <p class="text-[10px] text-slate-400 font-sans font-semibold">Chênh Lệch</p>
              <p class="text-xs font-bold mt-0.5" :class="selectedRow.discrepancy < 0 ? 'text-rose-400' : 'text-emerald-400'">
                {{ formatVND(selectedRow.discrepancy) }}
              </p>
            </div>
          </div>
        </div>

        <!-- 3. Raw Attributes JSON Viewer -->
        <div class="space-y-2 text-xs">
          <div class="flex items-center justify-between">
            <p class="font-semibold text-slate-300 flex items-center gap-1.5">
              <Info class="w-3.5 h-3.5 text-cyan-400" />
              Toàn bộ thuộc tính gốc trích xuất từ file (raw_attributes):
            </p>
            <button
              @click="copyJson(selectedRow.raw_attributes || {})"
              class="px-2.5 py-1 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-[11px] font-semibold transition-all border border-slate-700 flex items-center gap-1"
            >
              <Check v-if="copiedRaw" class="w-3 h-3 text-emerald-400" />
              <Copy v-else class="w-3 h-3" />
              <span>{{ copiedRaw ? 'Đã sao chép' : 'Sao chép JSON' }}</span>
            </button>
          </div>

          <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 font-mono text-emerald-300 overflow-x-auto text-[11px] max-h-56 leading-relaxed shadow-inner">
            <pre>{{ JSON.stringify(selectedRow.raw_attributes || {}, null, 2) }}</pre>
          </div>
        </div>

        <!-- 4. Raw Fee Breakdown JSON Viewer -->
        <div class="space-y-2 text-xs">
          <p class="font-semibold text-slate-300 flex items-center gap-1.5">
            <Layers class="w-3.5 h-3.5 text-amber-400" />
            Chi tiết bóc tách biểu phí nguyên bản (raw_fee_breakdown):
          </p>
          <div class="bg-slate-950 p-4 rounded-xl border border-slate-800 font-mono text-cyan-300 overflow-x-auto text-[11px] max-h-40 leading-relaxed shadow-inner">
            <pre>{{ JSON.stringify(selectedRow.raw_fee_breakdown || {}, null, 2) }}</pre>
          </div>
        </div>

        <!-- Modal Footer -->
        <div class="flex justify-end pt-2 border-t border-slate-800">
          <button
            @click="closeRawModal"
            class="px-5 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 rounded-xl text-xs font-bold transition-all border border-slate-700"
          >
            Đóng
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

