<script setup lang="ts">
import { useAuthStore } from '../../stores/auth'
import { Bell, User as UserIcon, ShieldCheck } from 'lucide-vue-next'

const authStore = useAuthStore()
</script>

<template>
  <header class="h-16 bg-slate-900/80 backdrop-blur border-b border-slate-800 flex items-center justify-between px-6 sticky top-0 z-30">
    <!-- Brand Title -->
    <div class="flex items-center gap-3">
      <div class="w-8 h-8 rounded-lg bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400 font-black">
        Ω
      </div>
      <div>
        <h1 class="text-base font-bold text-white tracking-tight flex items-center gap-2">
          OmniRecon
          <span class="text-[10px] bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-1.5 py-0.5 rounded font-mono">
            AGPL-3.0
          </span>
        </h1>
        <p class="text-[11px] text-slate-400 hidden sm:block">Đối soát & Cảnh báo Dòng tiền Đa kênh</p>
      </div>
    </div>

    <!-- Actions & User profile -->
    <div class="flex items-center gap-4">
      <router-link
        to="/alerts"
        class="relative p-2 text-slate-400 hover:text-white bg-slate-800/60 hover:bg-slate-800 border border-slate-700/60 rounded-lg transition-all"
        title="Trung tâm cảnh báo"
      >
        <Bell class="w-4 h-4" />
        <span class="absolute top-1 right-1 w-2 h-2 bg-rose-500 rounded-full animate-pulse"></span>
      </router-link>

      <div class="flex items-center gap-3 pl-3 border-l border-slate-800">
        <div class="w-8 h-8 rounded-full bg-slate-800 border border-slate-700 flex items-center justify-center text-slate-300 text-xs font-semibold">
          AD
        </div>
        <div class="hidden md:block">
          <p class="text-xs font-medium text-slate-200">{{ authStore.user.fullName }}</p>
          <p class="text-[10px] text-emerald-400 flex items-center gap-1 font-mono">
            <ShieldCheck class="w-3 h-3" /> {{ authStore.user.role }}
          </p>
        </div>
      </div>
    </div>
  </header>
</template>
