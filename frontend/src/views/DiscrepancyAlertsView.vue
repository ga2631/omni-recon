<script setup lang="ts">
import { onMounted } from 'vue'
import { useAlertStore } from '../stores/alerts'
import StatusBadge from '../components/common/StatusBadge.vue'
import { AlertTriangle, CheckCircle, RefreshCw, ShieldCheck } from 'lucide-vue-next'

const alertStore = useAlertStore()

onMounted(() => {
  alertStore.fetchAlerts()
})

function formatVND(amount: number) {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(amount)
}
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-bold text-white tracking-tight">Trung tâm Cảnh báo Thất thoát Dòng tiền</h2>
        <p class="text-xs text-slate-400 mt-1">Danh sách các đơn hàng có chênh lệch tiền thu hộ COD hoặc phí sàn bất thường</p>
      </div>
      <button
        @click="alertStore.fetchAlerts"
        class="p-2 bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 rounded-lg text-sm transition-all flex items-center gap-1.5"
        title="Làm mới"
      >
        <RefreshCw class="w-4 h-4" :class="{ 'animate-spin': alertStore.isLoading }" />
      </button>
    </div>

    <!-- Alert Cards List -->
    <div v-if="alertStore.alerts.length > 0" class="space-y-4">
      <div
        v-for="alert in alertStore.alerts"
        :key="alert.id"
        class="glass-panel rounded-xl p-5 border-l-4"
        :class="alert.severity === 'CRITICAL' ? 'border-l-rose-500' : 'border-l-amber-500'"
      >
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
          <div class="space-y-1">
            <div class="flex items-center gap-2">
              <span class="text-sm font-bold text-white font-mono">{{ alert.order_id }}</span>
              <span class="text-xs px-2 py-0.5 rounded bg-slate-800 text-slate-300 font-semibold">
                {{ alert.channel_code }}
              </span>
              <StatusBadge :severity="alert.severity" />
            </div>
            <p class="text-xs text-slate-300">{{ alert.notes }}</p>
            <div class="flex items-center gap-4 text-xs font-mono pt-1 text-slate-400">
              <span>Kỳ vọng: <b class="text-slate-200">{{ formatVND(alert.expected_amount) }}</b></span>
              <span>Thực tế: <b class="text-slate-200">{{ formatVND(alert.actual_amount) }}</b></span>
              <span>Lệch: <b class="text-rose-400 font-bold">{{ formatVND(alert.discrepancy_amount) }}</b></span>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <button
              v-if="alert.status === 'OPEN'"
              @click="alertStore.resolveAlert(alert.id)"
              class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-xs font-semibold flex items-center gap-1.5 transition-all"
            >
              <CheckCircle class="w-3.5 h-3.5" />
              <span>Đã Xử Lý / Khiếu Nại Xong</span>
            </button>
            <span
              v-else
              class="px-3 py-1.5 bg-slate-800 text-slate-400 rounded-lg text-xs font-semibold flex items-center gap-1"
            >
              <CheckCircle class="w-3.5 h-3.5 text-emerald-400" /> Đã đóng
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="glass-panel rounded-xl p-12 text-center text-slate-400">
      <div class="flex flex-col items-center justify-center gap-3">
        <div class="w-12 h-12 rounded-full bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400">
          <ShieldCheck class="w-6 h-6" />
        </div>
        <div class="text-sm font-medium text-slate-200">Không có cảnh báo thất thoát dòng tiền</div>
        <p class="text-xs text-slate-400 max-w-sm">
          Tất cả dữ liệu đơn hàng đối soát đều khớp chuẩn hoặc các chênh lệch trước đó đã được giải quyết xong.
        </p>
      </div>
    </div>
  </div>
</template>
