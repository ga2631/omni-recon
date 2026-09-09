<script setup lang="ts">
import { onMounted, computed } from 'vue'
import { RouterLink } from 'vue-router'
import { useReconciliationStore } from '../stores/reconciliation'
import { useFilterStore } from '../stores/filters'
import FilterBar from '../components/common/FilterBar.vue'
import StatusBadge from '../components/common/StatusBadge.vue'
import { Download, RefreshCw, FileText, UploadCloud, CheckCircle2, AlertTriangle } from 'lucide-vue-next'

const reconStore = useReconciliationStore()
const filterStore = useFilterStore()

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
      if (!matchOrder && !matchTracking) return false
    }
    return true
  })
})

const stats = computed(() => {
  const total = reconStore.items.length
  const matched = reconStore.items.filter((i) => i.status === 'MATCHED').length
  const discrepancies = total - matched
  const totalGross = reconStore.items.reduce((acc, i) => acc + (i.expected_amount || 0), 0)
  const totalNet = reconStore.items.reduce((acc, i) => acc + (i.actual_settlement || 0), 0)
  const totalFees = reconStore.items.reduce((acc, i) => acc + (i.total_fee || 0), 0)
  return { total, matched, discrepancies, totalGross, totalNet, totalFees }
})

function formatVND(amount: number) {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(amount)
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-bold text-white tracking-tight">Bảng Đối soát Chi tiết Đơn hàng</h2>
        <p class="text-xs text-slate-400 mt-1">So khớp 3 chiều giữa Đơn nội bộ, Sao kê sàn và Bảng kê COD vận chuyển</p>
      </div>
      <div class="flex items-center gap-3">
        <button
          @click="reconStore.fetchItems"
          class="p-2 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 rounded-lg text-sm transition-all flex items-center gap-1.5"
          title="Làm mới"
        >
          <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': reconStore.isLoading }" />
        </button>
        <button
          class="px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg text-sm font-semibold transition-all flex items-center gap-2"
        >
          <Download class="w-4 h-4" />
          <span>Xuất Báo Cáo Excel</span>
        </button>
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
    <div class="glass-panel rounded-xl overflow-hidden">
      <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-slate-300">
          <thead class="bg-slate-800/80 text-xs font-semibold uppercase text-slate-400">
            <tr>
              <th class="px-5 py-3.5">Mã Đơn Hàng</th>
              <th class="px-5 py-3.5">Kênh Bán</th>
              <th class="px-5 py-3.5">Mã Vận Đơn</th>
              <th class="px-5 py-3.5 text-right">Doanh thu dự kiến</th>
              <th class="px-5 py-3.5 text-right">Thực nhận về</th>
              <th class="px-5 py-3.5 text-right">Tổng phí sàn</th>
              <th class="px-5 py-3.5 text-right">Chênh lệch</th>
              <th class="px-5 py-3.5 text-center">Trạng thái</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800 font-mono text-xs">
            <template v-if="filteredItems.length > 0">
              <tr
                v-for="item in filteredItems"
                :key="item.order_id"
                class="hover:bg-slate-800/30 transition-colors"
              >
                <td class="px-5 py-3.5 font-sans font-medium text-white flex items-center gap-1.5">
                  <FileText class="w-3.5 h-3.5 text-slate-500" />
                  {{ item.order_id }}
                </td>
                <td class="px-5 py-3.5 font-sans font-semibold text-slate-200">{{ item.channel_code }}</td>
                <td class="px-5 py-3.5 text-slate-400">{{ item.tracking_code || '---' }}</td>
                <td class="px-5 py-3.5 text-right text-slate-200">{{ formatVND(item.expected_amount) }}</td>
                <td class="px-5 py-3.5 text-right font-semibold text-emerald-400">{{ formatVND(item.actual_settlement) }}</td>
                <td class="px-5 py-3.5 text-right text-rose-400">{{ formatVND(item.total_fee) }}</td>
                <td class="px-5 py-3.5 text-right font-bold" :class="item.discrepancy < 0 ? 'text-rose-400' : 'text-slate-400'">
                  {{ formatVND(item.discrepancy) }}
                </td>
                <td class="px-5 py-3.5 text-center font-sans">
                  <StatusBadge :status="item.status" />
                </td>
              </tr>
            </template>
            <tr v-else>
              <td colspan="8" class="px-6 py-12 text-center text-slate-400 font-sans">
                <div class="flex flex-col items-center justify-center gap-3">
                  <div class="w-12 h-12 rounded-full bg-slate-800 border border-slate-700 flex items-center justify-center text-slate-500">
                    <FileText class="w-6 h-6" />
                  </div>
                  <div class="text-sm font-medium text-slate-300">
                    {{ reconStore.items.length === 0 ? 'Chưa có dữ liệu đối soát đơn hàng' : 'Không tìm thấy đơn hàng khớp với bộ lọc' }}
                  </div>
                  <p class="text-xs text-slate-500 max-w-sm">
                    {{ reconStore.items.length === 0 
                      ? 'Vui lòng tải lên bảng kê đối soát từ Shopee, TikTok Shop hoặc Lazada để hệ thống tự động chuẩn hóa và đối soát.' 
                      : 'Hãy thử thay đổi từ khóa tìm kiếm hoặc điều chỉnh lại các điều kiện lọc phía trên.' }}
                  </p>
                  <RouterLink
                    v-if="reconStore.items.length === 0"
                    to="/upload"
                    class="mt-2 inline-flex items-center gap-2 px-4 py-2 bg-brand-600 hover:bg-brand-500 text-white text-xs font-semibold rounded-lg shadow-sm transition-all"
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
  </div>
</template>
