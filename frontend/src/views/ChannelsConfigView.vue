<script setup lang="ts">
import { ref } from 'vue'
import { Plus, Settings, CheckCircle2, AlertCircle } from 'lucide-vue-next'

const channels = ref([
  {
    id: '1',
    code: 'shopee_official',
    name: 'Gian Hàng Shopee Mall',
    type: 'SHOPEE',
    status: 'ACTIVE',
    commissionRate: '4.0%',
    paymentFeeRate: '4.5%',
    serviceFeeRate: '6.0%',
  },
  {
    id: '2',
    code: 'tiktok_shop_main',
    name: 'TikTok Shop Flagship',
    type: 'TIKTOK',
    status: 'ACTIVE',
    commissionRate: '5.0%',
    paymentFeeRate: '3.5%',
    serviceFeeRate: '0.0%',
  },
  {
    id: '3',
    code: 'ghn_express',
    name: 'Giao Hàng Nhanh (GHN)',
    type: 'GHN',
    status: 'ACTIVE',
    commissionRate: 'N/A',
    paymentFeeRate: 'N/A',
    serviceFeeRate: 'N/A',
  },
])
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h2 class="text-xl font-bold text-white tracking-tight">Cấu hình Gian hàng & Biểu phí Sàn</h2>
        <p class="text-xs text-slate-400 mt-1">Thiết lập công thức tính phí để hệ thống tự động phát hiện sai lệch khi đối soát</p>
      </div>
      <button class="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-sm font-semibold flex items-center gap-2 transition-all">
        <Plus class="w-4 h-4" />
        <span>Thêm Kênh Mới</span>
      </button>
    </div>

    <!-- Channels Grid -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-5">
      <div
        v-for="ch in channels"
        :key="ch.id"
        class="glass-panel rounded-2xl p-5 space-y-4 hover:border-slate-600 transition-all"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-sm font-bold text-white">{{ ch.name }}</h3>
            <p class="text-xs text-slate-400 font-mono">{{ ch.code }}</p>
          </div>
          <span class="px-2 py-0.5 rounded-full text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
            Đang hoạt động
          </span>
        </div>

        <div class="space-y-2 pt-2 border-t border-slate-800 text-xs">
          <div class="flex justify-between text-slate-400">
            <span>Phí cố định (Commission):</span>
            <span class="font-mono text-slate-200 font-semibold">{{ ch.commissionRate }}</span>
          </div>
          <div class="flex justify-between text-slate-400">
            <span>Phí thanh toán (Payment):</span>
            <span class="font-mono text-slate-200 font-semibold">{{ ch.paymentFeeRate }}</span>
          </div>
          <div class="flex justify-between text-slate-400">
            <span>Phí dịch vụ (Freeship/Hoàn xu):</span>
            <span class="font-mono text-slate-200 font-semibold">{{ ch.serviceFeeRate }}</span>
          </div>
        </div>

        <button class="w-full py-2 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-xs font-semibold flex items-center justify-center gap-1.5 transition-all">
          <Settings class="w-3.5 h-3.5" />
          <span>Chỉnh sửa Biểu phí</span>
        </button>
      </div>
    </div>
  </div>
</template>
