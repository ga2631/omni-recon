<script setup lang="ts">
import { computed } from 'vue'
import type { ReconStatus, AlertSeverity } from '../../types'

const props = defineProps<{
  status?: ReconStatus | string
  severity?: AlertSeverity | string
}>()

const badgeConfig = computed(() => {
  if (props.severity) {
    switch (props.severity) {
      case 'CRITICAL':
        return { text: 'Nghiêm trọng', bg: 'bg-red-500/10 text-red-400 border-red-500/20' }
      case 'HIGH':
        return { text: 'Cao', bg: 'bg-orange-500/10 text-orange-400 border-orange-500/20' }
      case 'MEDIUM':
        return { text: 'Trung bình', bg: 'bg-yellow-500/10 text-yellow-400 border-yellow-500/20' }
      default:
        return { text: 'Thông tin', bg: 'bg-blue-500/10 text-blue-400 border-blue-500/20' }
    }
  }

  switch (props.status) {
    case 'MATCHED':
      return { text: 'Khớp chuẩn', bg: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20' }
    case 'COD_MISMATCH':
      return { text: 'Lệch COD', bg: 'bg-rose-500/10 text-rose-400 border-rose-500/20' }
    case 'FEE_MISMATCH':
      return { text: 'Lệch phí sàn', bg: 'bg-amber-500/10 text-amber-400 border-amber-500/20' }
    case 'MISSING_SETTLEMENT':
      return { text: 'Chưa về tiền', bg: 'bg-purple-500/10 text-purple-400 border-purple-500/20' }
    default:
      return { text: props.status || 'Chưa rõ', bg: 'bg-slate-500/10 text-slate-400 border-slate-500/20' }
  }
})
</script>

<template>
  <span
    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border"
    :class="badgeConfig.bg"
  >
    {{ badgeConfig.text }}
  </span>
</template>
