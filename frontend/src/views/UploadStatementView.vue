<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import {
  UploadCloud,
  CheckCircle2,
  FileSpreadsheet,
  AlertTriangle,
  Layers,
  ShieldCheck,
  RefreshCw,
  Coins,
  Store,
  FileCheck,
  ArrowRight,
} from 'lucide-vue-next'
import { useStatementsStore } from '../stores/statements'

const store = useStatementsStore()

const selectedPlatform = ref<'SHOPEE' | 'TIKTOK' | 'LAZADA' | 'GHN'>('SHOPEE')
const selectedShopId = ref<string>('')
const reportType = ref<'INCOME_STATEMENT' | 'SETTLEMENT_REPORT' | 'LOGISTICS_REPORT'>('INCOME_STATEMENT')
const isDragging = ref(false)
const fileInputRef = ref<HTMLInputElement | null>(null)

onMounted(() => {
  store.fetchChannels()
  store.fetchBatches()
})

const platformShops = computed(() => {
  return store.channels.filter((ch) => ch.platform_type === selectedPlatform.value)
})

function onPlatformChange(platform: 'SHOPEE' | 'TIKTOK' | 'LAZADA' | 'GHN') {
  selectedPlatform.value = platform
  if (platform === 'SHOPEE' || platform === 'LAZADA') {
    reportType.value = 'INCOME_STATEMENT'
  } else if (platform === 'TIKTOK') {
    reportType.value = 'SETTLEMENT_REPORT'
  } else {
    reportType.value = 'LOGISTICS_REPORT'
  }
  const match = store.channels.find((ch) => ch.platform_type === platform)
  selectedShopId.value = match ? match.id : ''
}

async function handleFileSelected(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    const file = target.files[0]
    await processUpload(file)
    // reset input
    target.value = ''
  }
}

async function handleDrop(e: DragEvent) {
  isDragging.value = false
  if (e.dataTransfer?.files && e.dataTransfer.files.length > 0) {
    const file = e.dataTransfer.files[0]
    await processUpload(file)
  }
}

async function processUpload(file: File) {
  await store.uploadStatement(
    file,
    selectedPlatform.value,
    reportType.value,
    selectedShopId.value || undefined,
  )
}
</script>

<template>
  <div class="space-y-8 max-w-5xl mx-auto pb-12">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <h2 class="text-2xl font-bold text-white tracking-tight flex items-center gap-2">
          <UploadCloud class="w-7 h-7 text-emerald-400" />
          Tải lên & Chuẩn hóa Bảng kê Sao kê
        </h2>
        <p class="text-xs text-slate-400 mt-1">
          Hệ thống Medallion Architecture (Bronze Raw Lưu trữ & DuckDB Chuẩn hóa dữ liệu sang Silver Relational)
        </p>
      </div>

      <!-- Quick Platform Filter -->
      <div class="flex items-center gap-2 bg-slate-900/80 p-1.5 rounded-xl border border-slate-800">
        <span class="text-xs font-semibold text-slate-400 px-2 flex items-center gap-1">
          <Store class="w-3.5 h-3.5 text-emerald-400" /> Kênh:
        </span>
        <button
          v-for="p in [
            { id: 'SHOPEE', name: 'Shopee' },
            { id: 'TIKTOK', name: 'TikTok Shop' },
            { id: 'LAZADA', name: 'Lazada' },
            { id: 'GHN', name: 'ĐVVC' },
          ]"
          :key="p.id"
          @click="onPlatformChange(p.id as any)"
          class="px-3 py-1.5 rounded-lg text-xs font-semibold transition-all"
          :class="
            selectedPlatform === p.id
              ? 'bg-emerald-500 text-slate-950 shadow-md shadow-emerald-500/20 font-bold'
              : 'text-slate-400 hover:text-white hover:bg-slate-800'
          "
        >
          {{ p.name }}
        </button>
      </div>
    </div>

    <!-- Upload Box & Channel Settings -->
    <div class="glass-panel rounded-2xl p-6 sm:p-8 space-y-6 shadow-xl relative overflow-hidden">
      <!-- Top Decorative Glow -->
      <div class="absolute -top-24 -right-24 w-48 h-48 bg-emerald-500/10 rounded-full blur-3xl pointer-events-none"></div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Shop Assignment -->
        <div class="space-y-1.5">
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
            <Store class="w-3.5 h-3.5 text-emerald-400" /> Gian Hàng / Cửa Hàng Liên Kết
          </label>
          <select
            v-model="selectedShopId"
            class="w-full bg-slate-900/90 border border-slate-700 text-slate-200 text-xs rounded-xl px-3.5 py-2.5 focus:outline-none focus:border-emerald-500 transition-all font-medium"
          >
            <option value="">-- Tự động khớp theo metadata --</option>
            <option v-for="shop in platformShops" :key="shop.id" :value="shop.id">
              {{ shop.name }} ({{ shop.code }})
            </option>
          </select>
        </div>

        <!-- Report Type -->
        <div class="space-y-1.5">
          <label class="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
            <Layers class="w-3.5 h-3.5 text-emerald-400" /> Loại Bảng kê Báo cáo
          </label>
          <select
            v-model="reportType"
            class="w-full bg-slate-900/90 border border-slate-700 text-slate-200 text-xs rounded-xl px-3.5 py-2.5 focus:outline-none focus:border-emerald-500 transition-all font-medium"
          >
            <option value="INCOME_STATEMENT">Báo cáo Thu nhập / Income Statement (.xlsx, .csv)</option>
            <option value="SETTLEMENT_REPORT">Báo cáo Quyết toán / Settlement Report (.xlsx)</option>
            <option value="LOGISTICS_REPORT">Bảng kê Cước & COD Đơn vị Vận chuyển (.xlsx, .csv)</option>
          </select>
        </div>
      </div>

      <!-- Drag & Drop Upload Zone -->
      <div
        @dragover.prevent="isDragging = true"
        @dragleave.prevent="isDragging = false"
        @drop.prevent="handleDrop"
        class="border-2 border-dashed rounded-2xl p-10 sm:p-12 flex flex-col items-center justify-center text-center transition-all cursor-pointer relative"
        :class="[
          isDragging
            ? 'border-emerald-400 bg-emerald-500/10 scale-[0.99]'
            : 'border-slate-700 hover:border-emerald-500/60 bg-slate-900/50 hover:bg-slate-900/80',
          store.isUploading ? 'pointer-events-none opacity-60' : '',
        ]"
        @click="fileInputRef?.click()"
      >
        <div class="p-4 bg-emerald-500/10 border border-emerald-500/20 rounded-2xl text-emerald-400 mb-4 shadow-lg">
          <UploadCloud class="w-10 h-10" />
        </div>
        <p class="text-base font-bold text-white tracking-tight">Kéo & thả tệp bảng kê vào đây hoặc bấm để tải lên</p>
        <p class="text-xs text-slate-400 mt-1.5">
          Hỗ trợ định dạng Excel (.xlsx, .xls) và CSV. Dung lượng tối đa <span class="text-emerald-400 font-semibold">250MB</span>
        </p>

        <input
          ref="fileInputRef"
          type="file"
          accept=".xlsx,.xls,.csv"
          class="hidden"
          @change="handleFileSelected"
        />

        <button
          type="button"
          class="mt-5 px-5 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-slate-950 rounded-xl text-xs font-bold shadow-md shadow-emerald-500/20 transition-all flex items-center gap-2"
        >
          <FileSpreadsheet class="w-4 h-4" /> Chọn tệp từ máy tính
        </button>
      </div>

      <!-- Uploading / Processing Progress -->
      <div v-if="store.isUploading" class="p-4 bg-slate-900/90 border border-emerald-500/30 rounded-xl space-y-3">
        <div class="flex items-center justify-between text-xs">
          <span class="font-bold text-emerald-400 flex items-center gap-2">
            <RefreshCw class="w-4 h-4 animate-spin text-emerald-400" />
            Đang tải lên Bronze Layer & DuckDB chuẩn hóa...
          </span>
          <span class="text-slate-400 font-bold">{{ store.uploadProgress }}%</span>
        </div>
        <div class="w-full bg-slate-800 rounded-full h-2 overflow-hidden">
          <div
            class="bg-gradient-to-r from-emerald-500 to-teal-400 h-2 rounded-full transition-all duration-300"
            :style="{ width: store.uploadProgress + '%' }"
          ></div>
        </div>
      </div>

      <!-- Error Banner -->
      <div
        v-if="store.errorMessage"
        class="p-4 bg-rose-500/10 border border-rose-500/30 rounded-xl flex items-start gap-3 text-xs text-rose-300"
      >
        <AlertTriangle class="w-5 h-5 text-rose-400 flex-shrink-0 mt-0.5" />
        <div>
          <p class="font-bold text-rose-200">Xử lý tệp thất bại</p>
          <p class="mt-0.5">{{ store.errorMessage }}</p>
        </div>
      </div>
    </div>

    <!-- Upload Result & Sanity Check Card -->
    <div v-if="store.lastUploadResult" class="space-y-6">
      <div class="glass-panel rounded-2xl p-6 border-emerald-500/30 shadow-2xl relative overflow-hidden space-y-6">
        <!-- Header status -->
        <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 border-b border-slate-800 pb-4">
          <div class="flex items-center gap-3">
            <div class="p-2.5 bg-emerald-500/20 text-emerald-400 rounded-xl">
              <CheckCircle2 class="w-6 h-6" />
            </div>
            <div>
              <h3 class="text-base font-bold text-white">Chuẩn Hóa Thành Công (Silver Conformed)</h3>
              <p class="text-xs text-slate-400">
                Tệp: <span class="text-emerald-300 font-semibold">{{ store.lastUploadResult.original_filename }}</span>
                ({{ (store.lastUploadResult.file_size_bytes / (1024 * 1024)).toFixed(2) }} MB)
              </p>
            </div>
          </div>

          <div class="flex items-center gap-2">
            <span class="px-3 py-1 bg-amber-500/10 border border-amber-500/20 text-amber-300 text-[11px] font-bold rounded-lg flex items-center gap-1">
              🥉 Bronze Saved
            </span>
            <span class="px-3 py-1 bg-cyan-500/10 border border-cyan-500/20 text-cyan-300 text-[11px] font-bold rounded-lg flex items-center gap-1">
              ⚡ DuckDB Verified
            </span>
            <span class="px-3 py-1 bg-emerald-500/10 border border-emerald-500/20 text-emerald-300 text-[11px] font-bold rounded-lg flex items-center gap-1">
              🥈 Silver Standardized
            </span>
          </div>
        </div>

        <!-- Sanity Metrics Counters -->
        <div
          v-if="store.lastUploadResult.sanity_check"
          class="grid grid-cols-2 sm:grid-cols-4 gap-4"
        >
          <div class="bg-slate-900/70 p-4 rounded-xl border border-slate-800">
            <p class="text-[11px] font-semibold text-slate-400">Tổng Số Bản Ghi</p>
            <p class="text-lg font-bold text-white mt-1">
              {{ store.lastUploadResult.sanity_check.total_rows.toLocaleString() }}
            </p>
          </div>

          <div class="bg-slate-900/70 p-4 rounded-xl border border-slate-800">
            <p class="text-[11px] font-semibold text-slate-400">Tổng Doanh Thu Gốc</p>
            <p class="text-lg font-bold text-emerald-400 mt-1">
              {{ store.formatVND(store.lastUploadResult.sanity_check.total_gross) }}
            </p>
          </div>

          <div class="bg-slate-900/70 p-4 rounded-xl border border-slate-800">
            <p class="text-[11px] font-semibold text-slate-400">Tiền Thực Nhận (Net)</p>
            <p class="text-lg font-bold text-cyan-400 mt-1">
              {{ store.formatVND(store.lastUploadResult.sanity_check.total_net) }}
            </p>
          </div>

          <div class="bg-slate-900/70 p-4 rounded-xl border border-slate-800">
            <p class="text-[11px] font-semibold text-slate-400">Tổng Phí Sàn Cấn Trừ</p>
            <p class="text-lg font-bold text-amber-400 mt-1">
              {{ store.formatVND(store.lastUploadResult.sanity_check.total_fees) }}
            </p>
          </div>
        </div>

        <!-- Sample Records Preview -->
        <div v-if="store.lastUploadResult.sample_records && store.lastUploadResult.sample_records.length > 0" class="space-y-3">
          <div class="flex items-center justify-between">
            <h4 class="text-xs font-bold text-slate-300 uppercase tracking-wider flex items-center gap-1.5">
              <FileCheck class="w-4 h-4 text-emerald-400" /> Bản xem trước 5 dòng đầu tiên đã chuẩn hóa (Silver Preview)
            </h4>
            <span class="text-[11px] text-slate-400">
              Định dạng chuẩn: Unified Schema (VND)
            </span>
          </div>

          <div class="overflow-x-auto rounded-xl border border-slate-800 bg-slate-900/90">
            <table class="w-full text-left text-xs text-slate-300">
              <thead class="bg-slate-800/80 text-[11px] text-slate-400 uppercase font-bold border-b border-slate-700/60">
                <tr>
                  <th class="p-3">Mã Đơn Hàng</th>
                  <th class="p-3">Mã Quyết Toán</th>
                  <th class="p-3 text-right">Doanh Thu Gốc</th>
                  <th class="p-3 text-right">Phí Cố Định</th>
                  <th class="p-3 text-right">Phí Dịch Vụ</th>
                  <th class="p-3 text-right">Phí TT</th>
                  <th class="p-3 text-right">Thực Nhận</th>
                  <th class="p-3 text-center">Thời Gian</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-800/70 font-mono">
                <tr v-for="rec in store.lastUploadResult.sample_records" :key="rec.order_id" class="hover:bg-slate-800/40 transition-all">
                  <td class="p-3 font-semibold text-white">{{ rec.order_id }}</td>
                  <td class="p-3 text-slate-400">{{ rec.payout_id || '-' }}</td>
                  <td class="p-3 text-right font-medium text-emerald-400">{{ store.formatVND(rec.gross_amount) }}</td>
                  <td class="p-3 text-right text-rose-400">{{ store.formatVND(rec.commission_fee) }}</td>
                  <td class="p-3 text-right text-rose-400">{{ store.formatVND(rec.service_fee) }}</td>
                  <td class="p-3 text-right text-rose-400">{{ store.formatVND(rec.payment_fee) }}</td>
                  <td class="p-3 text-right font-bold text-cyan-300">{{ store.formatVND(rec.net_settlement) }}</td>
                  <td class="p-3 text-center text-slate-400 text-[11px]">{{ rec.settled_at || 'Chờ về' }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Upload History -->
    <div class="glass-panel rounded-2xl p-6 sm:p-8 space-y-4 shadow-xl">
      <div class="flex items-center justify-between border-b border-slate-800 pb-4">
        <h3 class="text-base font-bold text-white flex items-center gap-2">
          <FileSpreadsheet class="w-5 h-5 text-emerald-400" />
          Lịch sử Tải lên & Chuẩn hóa Gần đây (Bronze Logs)
        </h3>
        <button
          @click="store.fetchBatches"
          class="text-xs text-slate-400 hover:text-emerald-400 flex items-center gap-1.5 transition-all font-semibold"
        >
          <RefreshCw class="w-3.5 h-3.5" /> Làm mới
        </button>
      </div>

      <div class="divide-y divide-slate-800/60">
        <div
          v-for="batch in store.batches"
          :key="batch.id"
          class="py-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-3 hover:bg-slate-800/30 px-3 rounded-xl transition-all"
        >
          <div class="flex items-center gap-3.5">
            <div class="p-2 bg-slate-800 rounded-xl text-emerald-400 border border-slate-700/60">
              <FileSpreadsheet class="w-5 h-5" />
            </div>
            <div>
              <p class="text-xs font-bold text-white">{{ batch.filename }}</p>
              <div class="flex items-center gap-3 text-[11px] text-slate-400 mt-0.5">
                <span class="text-emerald-400 font-semibold">{{ batch.platform || 'SHOPEE' }}</span>
                <span>•</span>
                <span>{{ batch.total_rows.toLocaleString() }} dòng</span>
                <span>•</span>
                <span>{{ new Date(batch.created_at).toLocaleString('vi-VN') }}</span>
              </div>
            </div>
          </div>

          <div class="flex items-center gap-3">
            <span
              class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-bold border"
              :class="
                batch.status === 'COMPLETED'
                  ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20'
                  : 'bg-amber-500/10 text-amber-400 border-amber-500/20'
              "
            >
              <CheckCircle2 class="w-3.5 h-3.5" />
              {{ batch.status === 'COMPLETED' ? 'Đã chuẩn hóa' : batch.status }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
