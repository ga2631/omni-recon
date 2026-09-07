<script setup lang="ts">
import { onMounted } from 'vue'
import { useReconciliationStore } from '../stores/reconciliation'
import FilterBar from '../components/common/FilterBar.vue'
import StatusBadge from '../components/common/StatusBadge.vue'
import { Download, RefreshCw, FileText } from 'lucide-vue-next'

const reconStore = useReconciliationStore()

onMounted(() => {
  reconStore.fetchItems()
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
            <tr
              v-for="item in reconStore.items"
              :key="item.order_id"
              class="hover:bg-slate-800/30 transition-colors"
            >
              <td class="px-5 py-3.5 font-sans font-medium text-white flex items-center gap-1.5">
                <FileText class="w-3.5 h-3.5 text-slate-500" />
                {{ item.order_id }}
              </td>
              <td class="px-5 py-3.5 font-sans">{{ item.channel_code }}</td>
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
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
