<script setup lang="ts">
import { ref } from 'vue'
import { UploadCloud, CheckCircle2, FileSpreadsheet, AlertCircle } from 'lucide-vue-next'

const selectedChannel = ref('SHOPEE')
const isDragging = ref(false)
const uploadedFiles = ref<Array<{ name: string; size: string; status: string }>>([
  {
    name: 'Shopee_Income_Statement_August_2026.xlsx',
    size: '14.2 MB',
    status: 'Đã đối soát xong',
  },
  {
    name: 'TikTok_Settlement_Order_List_202608.xlsx',
    size: '8.7 MB',
    status: 'Đã đối soát xong',
  },
])

function handleDrop(e: DragEvent) {
  isDragging.value = false
  if (e.dataTransfer?.files) {
    // Add file handling logic
  }
}
</script>

<template>
  <div class="space-y-6 max-w-4xl mx-auto">
    <div>
      <h2 class="text-xl font-bold text-white tracking-tight">Tải lên Bảng kê Sao kê Mới</h2>
      <p class="text-xs text-slate-400 mt-1">Hỗ trợ file Excel (.xlsx), CSV xuất từ Shopee, Lazada, TikTok Shop, GHN, GHTK</p>
    </div>

    <!-- Upload Box -->
    <div class="glass-panel rounded-2xl p-6 space-y-6">
      <!-- Channel Selection -->
      <div class="space-y-2">
        <label class="text-xs font-semibold text-slate-300 uppercase tracking-wider">Chọn Kênh Bán / Đơn Vị Vận Chuyển</label>
        <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
          <button
            v-for="ch in [
              { id: 'SHOPEE', name: 'Shopee' },
              { id: 'TIKTOK', name: 'TikTok Shop' },
              { id: 'LAZADA', name: 'Lazada' },
              { id: 'CARRIER', name: 'ĐVVC (GHN/GHTK)' },
            ]"
            :key="ch.id"
            @click="selectedChannel = ch.id"
            class="p-3 rounded-xl border text-sm font-medium transition-all text-center"
            :class="selectedChannel === ch.id ? 'bg-emerald-500/10 border-emerald-500 text-emerald-400 font-bold' : 'bg-slate-800/60 border-slate-700 text-slate-400 hover:border-slate-600'"
          >
            {{ ch.name }}
          </button>
        </div>
      </div>

      <!-- Drag and drop zone -->
      <div
        @dragover.prevent="isDragging = true"
        @dragleave.prevent="isDragging = false"
        @drop.prevent="handleDrop"
        class="border-2 border-dashed rounded-xl p-10 flex flex-col items-center justify-center text-center transition-all"
        :class="isDragging ? 'border-emerald-500 bg-emerald-500/5' : 'border-slate-700 hover:border-slate-600 bg-slate-900/40'"
      >
        <div class="p-4 bg-slate-800 rounded-full text-emerald-400 mb-3">
          <UploadCloud class="w-8 h-8" />
        </div>
        <p class="text-sm font-semibold text-white">Kéo thả tệp bảng kê vào đây hoặc bấm để chọn tệp</p>
        <p class="text-xs text-slate-400 mt-1">Hỗ trợ .XLSX, .XLS, .CSV dung lượng tối đa 250MB</p>
        <input type="file" multiple class="hidden" id="fileUpload" />
        <label
          for="fileUpload"
          class="mt-4 px-4 py-2 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg text-xs font-semibold cursor-pointer transition-all"
        >
          Chọn tệp từ máy tính
        </label>
      </div>
    </div>

    <!-- Recent Batches -->
    <div class="glass-panel rounded-2xl p-6">
      <h3 class="text-sm font-bold text-white mb-4">Lịch sử Tải lên Gần đây</h3>
      <div class="space-y-3">
        <div
          v-for="file in uploadedFiles"
          :key="file.name"
          class="flex items-center justify-between p-3 bg-slate-800/50 rounded-xl border border-slate-700/60"
        >
          <div class="flex items-center gap-3">
            <FileSpreadsheet class="w-5 h-5 text-emerald-400" />
            <div>
              <p class="text-xs font-semibold text-white">{{ file.name }}</p>
              <p class="text-[11px] text-slate-400">{{ file.size }}</p>
            </div>
          </div>
          <span class="inline-flex items-center gap-1 text-xs text-emerald-400 font-medium">
            <CheckCircle2 class="w-3.5 h-3.5" /> {{ file.status }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
