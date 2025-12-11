import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

// === 核心路由（必须保留）===
const coreRoutes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/Home.vue'),
  },
]

// === 可选模块路由（不需要可注释掉）===

// 演示模块
import { demoRoutes } from '@/modules/demo'

// Cookie 读取模块
import { cookieRoutes } from '@/modules/cookie'

// 投屏模块
import { screenRoutes } from '@/modules/screen'

// === 合并所有路由 ===
const routes: RouteRecordRaw[] = [
  ...coreRoutes,
  ...demoRoutes, // 注释此行移除演示模块
  ...cookieRoutes, // 注释此行移除 Cookie 模块
  ...screenRoutes, // 注释此行移除投屏模块
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
