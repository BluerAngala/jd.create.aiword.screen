import { createRouter, createWebHashHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// === 核心路由（必须保留）===
const coreRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'LiveAssistant',
    component: () => import('@/modules/live/views/LiveAssistant.vue'),
  },
  {
    path: '/screen-content',
    name: 'ScreenContent',
    component: () => import('@/modules/live/views/ScreenContent.vue'),
  },
  {
    path: '/screen-countdown',
    name: 'ScreenCountdown',
    component: () => import('@/modules/live/views/ScreenCountdown.vue'),
  },
  {
    path: '/screen-script',
    name: 'ScreenScript',
    component: () => import('@/modules/live/views/ScreenScript.vue'),
  },
]

// === 合并所有路由 ===
const routes: RouteRecordRaw[] = [...coreRoutes]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
