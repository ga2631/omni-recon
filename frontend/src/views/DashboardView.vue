<script setup lang="ts">
import { ref, onMounted } from 'vue'
import StatCard from '../components/common/StatCard.vue'
import {
  DollarSign,
  TrendingDown,
  AlertCircle,
  Truck,
  ArrowUpRight,
  ShieldAlert,
  RefreshCw
} from 'lucide-vue-next'
import { apiFetch } from '../utils/api'

const isLoading = ref(false)

const stats = ref({
  grossRevenue: '0 ₫',
  netSettled: '0 ₫',
  totalFees: '0 ₫',
  codPending: '0 ₫',
  discrepancyTotal: '0 ₫',
  discrepancyCount: 0,
})

const channels = ref<Array<{ name: string; revenue: string; fee: string; rate: string; status: string }>>([])

function formatVND(amount: number) {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(amount)
}

async function fetchDashboardMetrics() {
  isLoading.value = true
  try {
    const res = await apiFetch('/api/v1/dashboard/metrics')
    if (res.ok) {
      const json = await res.json()
      if (json.data && json.data.summary) {
        const s = json.data.summary
        stats.value = {
          grossRevenue: formatVND(s.gross_revenue),
          netSettled: formatVND(s.net_settled),
          totalFees: formatVND(s.total_platform_fees),
          codPending: formatVND(s.cod_pending),
          discrepancyTotal: formatVND(s.total_discrepancy_amount),
          discrepancyCount: s.discrepancy_count,
        }
      }
      if (json.data && json.data.channel_breakdown) {
        channels.value = json.data.channel_breakdown.map((c: any) => ({
          name: c.channel,
          revenue: formatVND(c.revenue),
          fee: formatVND(c.fees),
          rate: `${c.fee_rate}%`,
          status: 'Đang hoạt động',
        }))
      }
    }
  } catch (e) {
    console.error('Failed to fetch dashboard metrics', e)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  fetchDashboardMetrics()
})
</script>

<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
      <div>
        <h2 class="text-xl font-bold text-white tracking-tight">Tổng quan Dòng tiền & Đối soát</h2>
        <p class="text-xs text-slate-400 mt-1">Dữ liệu tổng hợp từ các kênh Shopee, TikTok Shop, Lazada, GHN, GHTK</p>
      </div>
      <div class="flex items-center gap-3">
        <router-link
          to="/upload"
          class="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg text-sm font-semibold shadow-lg shadow-emerald-900/30 transition-all flex items-center gap-2"
        >
          <ArrowUpRight class="w-4 h-4" />
          <span>Tải lên Bảng kê Mới</span>
        </router-link>
      </div>
    </div>

    <!-- Stat Cards Grid -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <StatCard
        title="Tổng Doanh thu Gộp"
        :value="stats.grossRevenue"
        subtitle="Tổng giá trị đơn hàng phát sinh"
        trend="+12.4%"
        :trend-positive="true"
      />
      <StatCard
        title="Thực nhận về Ví/Bank"
        :value="stats.netSettled"
        subtitle="Tiền đã quyết toán thành công"
        trend="+8.2%"
        :trend-positive="true"
      />
      <StatCard
        title="Tổng Phí sàn & Vận chuyển"
        :value="stats.totalFees"
        subtitle="Chiếm 14.5% trên doanh thu gộp"
        trend="-1.5%"
        :trend-positive="true"
      />
      <StatCard
        title="Tiền Thu hộ (COD) Đang giữ"
        :value="stats.codPending"
        subtitle="ĐVVC đang giữ chưa đối soát"
        trend="8 ngày SLA"
        :trend-positive="false"
      />
    </div>

    <!-- Anomaly Callout Banner -->
    <div class="glass-panel border-rose-500/30 bg-rose-500/5 rounded-xl p-4 flex items-start justify-between gap-4">
      <div class="flex items-start gap-3">
        <div class="p-2 bg-rose-500/20 text-rose-400 rounded-lg">
          <ShieldAlert class="w-5 h-5" />
        </div>
        <div>
          <h3 class="text-sm font-bold text-rose-300">Phát hiện {{ stats.discrepancyCount }} đơn hàng bị lệch tiền cần xử lý!</h3>
          <p class="text-xs text-slate-300 mt-0.5">
            Tổng số tiền chênh lệch phát hiện: <span class="font-bold text-rose-400">{{ stats.discrepancyTotal }}</span> (gồm lệch COD ĐVVC và lệch biểu phí sàn).
          </p>
        </div>
      </div>
      <router-link
        to="/alerts"
        class="px-3 py-1.5 bg-rose-600 hover:bg-rose-500 text-white text-xs font-semibold rounded-lg transition-all whitespace-nowrap"
      >
        Xem chi tiết
      </router-link>
    </div>

    <!-- Channel Breakdown Table -->
    <div class="glass-panel rounded-xl overflow-hidden">
      <div class="p-5 border-b border-slate-800 flex items-center justify-between">
        <h3 class="text-sm font-bold text-white">Hiệu quả Dòng tiền & Tỷ lệ Phí theo Kênh</h3>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full text-left text-sm text-slate-300">
          <thead class="bg-slate-800/60 text-xs font-semibold uppercase text-slate-400">
            <tr>
              <th class="px-6 py-3">Kênh / Sàn TMĐT</th>
              <th class="px-6 py-3">Doanh thu Gộp</th>
              <th class="px-6 py-3">Tổng Phí Khấu trừ</th>
              <th class="px-6 py-3">Tỷ lệ Phí Thực tế</th>
              <th class="px-6 py-3">Trạng thái Đối soát</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-slate-800">
            <tr v-for="c in channels" :key="c.name" class="hover:bg-slate-800/30">
              <td class="px-6 py-4 font-medium text-white">{{ c.name }}</td>
              <td class="px-6 py-4">{{ c.revenue }}</td>
              <td class="px-6 py-4 text-rose-400">{{ c.fee }}</td>
              <td class="px-6 py-4 font-mono font-semibold">{{ c.rate }}</td>
              <td class="px-6 py-4">
                <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                  {{ c.status }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
