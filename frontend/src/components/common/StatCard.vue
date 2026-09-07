<script setup lang="ts">
defineProps<{
  title: string
  value: string
  subtitle?: string
  trend?: string
  trendPositive?: boolean
  color?: 'green' | 'blue' | 'amber' | 'rose'
}>()
</script>

<template>
  <div class="glass-panel rounded-xl p-5 hover:border-slate-600 transition-all duration-200">
    <div class="flex items-center justify-between">
      <span class="text-xs font-medium uppercase tracking-wider text-slate-400">{{ title }}</span>
      <span
        v-if="trend"
        class="text-xs font-semibold px-2 py-0.5 rounded-full"
        :class="trendPositive ? 'bg-emerald-500/10 text-emerald-400' : 'bg-rose-500/10 text-rose-400'"
      >
        {{ trend }}
      </span>
    </div>
    <div class="mt-2 flex items-baseline gap-2">
      <span class="text-2xl font-bold tracking-tight text-white">{{ value }}</span>
    </div>
    <p v-if="subtitle" class="mt-1 text-xs text-slate-400">{{ subtitle }}</p>
  </div>
</template>
