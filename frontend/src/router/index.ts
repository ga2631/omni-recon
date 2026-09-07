import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '../views/DashboardView.vue'
import ReconciliationView from '../views/ReconciliationView.vue'
import UploadStatementView from '../views/UploadStatementView.vue'
import DiscrepancyAlertsView from '../views/DiscrepancyAlertsView.vue'
import ChannelsConfigView from '../views/ChannelsConfigView.vue'

const routes = [
  {
    path: '/',
    name: 'dashboard',
    component: DashboardView,
  },
  {
    path: '/reconciliation',
    name: 'reconciliation',
    component: ReconciliationView,
  },
  {
    path: '/upload',
    name: 'upload',
    component: UploadStatementView,
  },
  {
    path: '/alerts',
    name: 'alerts',
    component: DiscrepancyAlertsView,
  },
  {
    path: '/channels',
    name: 'channels',
    component: ChannelsConfigView,
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
