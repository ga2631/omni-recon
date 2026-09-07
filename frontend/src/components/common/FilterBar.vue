<script setup lang="ts">
import { useFilterStore } from '../../stores/filters'
import { Search, Filter, RotateCcw } from 'lucide-vue-next'

const filterStore = useFilterStore()
</script>

<template>
  <div class="glass-panel rounded-xl p-4 flex flex-wrap items-center justify-between gap-4">
    <!-- Search Box -->
    <div class="relative flex-1 min-w-[240px] max-w-md">
      <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-slate-400" />
      <input
        v-model="filterStore.searchQuery"
        type="text"
        placeholder="Tìm kiếm theo mã đơn hàng, mã vận đơn..."
        class="w-full pl-9 pr-4 py-2 bg-slate-800/80 border border-slate-700 rounded-lg text-sm text-slate-200 placeholder-slate-500 focus:outline-none focus:border-brand-500 focus:ring-1 focus:ring-brand-500 transition-all"
      />
    </div>

    <!-- Dropdown Filters -->
    <div class="flex items-center gap-3 flex-wrap">
      <!-- Channel Filter -->
      <div class="flex items-center gap-2">
        <label class="text-xs text-slate-400 font-medium">Kênh:</label>
        <select
          v-model="filterStore.selectedChannel"
          class="bg-slate-800 border border-slate-700 rounded-lg px-3 py-1.5 text-sm text-slate-200 focus:outline-none focus:border-brand-500"
        >
          <option value="ALL">Tất cả kênh</option>
          <option value="SHOPEE">Shopee</option>
          <option value="TIKTOK">TikTok Shop</option>
          <option value="LAZADA">Lazada</option>
          <option value="GHN">Giao Hàng Nhanh</option>
          <option value="GHTK">GHTK</option>
        </select>
      </div>

      <!-- Status Filter -->
      <div class="flex items-center gap-2">
        <label class="text-xs text-slate-400 font-medium">Trạng thái:</label>
        <select
          v-model="filterStore.selectedStatus"
          class="bg-slate-800 border border-slate-700 rounded-lg px-3 py-1.5 text-sm text-slate-200 focus:outline-none focus:border-brand-500"
        >
          <option value="ALL">Tất cả trạng thái</option>
          <option value="MATCHED">Khớp chuẩn (100%)</option>
          <option value="COD_MISMATCH">Lệch tiền COD</option>
          <option value="FEE_MISMATCH">Lệch biểu phí sàn</option>
          <option value="MISSING_SETTLEMENT">Chưa thanh toán</option>
        </select>
      </div>

      <!-- Reset Filter Button -->
      <button
        @click="filterStore.resetFilters"
        class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-lg text-sm border border-slate-700 transition-all"
        title="Đặt lại bộ lọc"
      >
        <RotateCcw class="w-3.5 h-3.5" />
        <span>Đặt lại</span>
      </button>
    </div>
  </div>
</template>
