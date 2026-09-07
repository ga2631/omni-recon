<script setup lang="ts">
import { useRoute } from 'vue-router'
import {
  LayoutDashboard,
  FileSpreadsheet,
  UploadCloud,
  AlertTriangle,
  Layers,
  Settings,
  BookOpen
} from 'lucide-vue-next'

const route = useRoute()

const navItems = [
  { name: 'Tổng quan Dòng tiền', path: '/', icon: LayoutDashboard },
  { name: 'Bảng Đối soát', path: '/reconciliation', icon: FileSpreadsheet },
  { name: 'Tải lên Bảng kê', path: '/upload', icon: UploadCloud },
  { name: 'Cảnh báo Lệch tiền', path: '/alerts', icon: AlertTriangle, badge: '2' },
  { name: 'Cấu hình Kênh bán', path: '/channels', icon: Layers },
]
</script>

<template>
  <aside class="w-64 bg-slate-900/60 backdrop-blur border-r border-slate-800 flex flex-col justify-between p-4 min-h-[calc(100vh-4rem)]">
    <!-- Main Nav -->
    <div class="space-y-1">
      <div class="px-3 py-2 text-[11px] font-bold uppercase tracking-wider text-slate-500">
        Phân tích & Vận hành
      </div>
      <router-link
        v-for="item in navItems"
        :key="item.path"
        :to="item.path"
        class="flex items-center justify-between px-3 py-2.5 rounded-lg text-sm font-medium transition-all group"
        :class="route.path === item.path ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/20' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'"
      >
        <div class="flex items-center gap-3">
          <component :is="item.icon" class="w-4 h-4 transition-colors group-hover:text-white" />
          <span>{{ item.name }}</span>
        </div>
        <span
          v-if="item.badge"
          class="px-1.5 py-0.5 text-[10px] font-bold rounded-full bg-rose-500/20 text-rose-400 border border-rose-500/30"
        >
          {{ item.badge }}
        </span>
      </router-link>
    </div>

    <!-- Footer link to Docs & Attribution -->
    <div class="pt-4 border-t border-slate-800 space-y-2">
      <a
        href="https://github.com/omni-recon/omni-recon"
        target="_blank"
        class="flex items-center gap-2 px-3 py-2 text-xs text-slate-400 hover:text-slate-200 rounded-lg hover:bg-slate-800/50 transition-all"
      >
        <BookOpen class="w-3.5 h-3.5" />
        <span>Tài liệu & GitHub Repo</span>
      </a>
      <div class="px-3 text-[10px] text-slate-500 font-mono">
        OmniRecon Open Source Core v0.1.0
      </div>
    </div>
  </aside>
</template>
