import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// === 核心路由（必须保留）===
const coreRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'LiveAssistant',
    component: () => import('@/modules/live/views/LiveAssistant.vue'),
  },
]

// === 合并所有路由 ===
const routes: RouteRecordRaw[] = [...coreRoutes]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
