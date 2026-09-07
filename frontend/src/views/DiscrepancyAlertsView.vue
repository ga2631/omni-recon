<script setup lang="ts">
import { useAlertStore } from '../stores/alerts'
import StatusBadge from '../components/common/StatusBadge.vue'
import { AlertTriangle, CheckCircle, ExternalLink, MessageSquare } from 'lucide-vue-next'

const alertStore = useAlertStore()

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
    </div>

    <!-- Alert Cards List -->
    <div class="space-y-4">
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
  </div>
</template>
