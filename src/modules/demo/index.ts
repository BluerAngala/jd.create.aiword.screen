/**
 * 演示模块
 * 包含各种功能示例页面
 *
 * 不需要此模块时：
 * 1. 删除整个 modules/demo 文件夹
 * 2. 在 router/index.ts 中移除 demoRoutes
 * 3. 在 config/app.config.ts 中移除相关导航项
 */

import type { RouteRecordRaw } from 'vue-router'

// 路由配置
export const demoRoutes: RouteRecordRaw[] = [
  {
    path: '/demo',
    name: 'Demo',
    component: () => import('./views/Demo.vue'),
  },
  {
    path: '/animation',
    name: 'Animation',
    component: () => import('./views/Animation.vue'),
  },
  {
    path: '/icons',
    name: 'Icons',
    component: () => import('./views/Icons.vue'),
  },
  {
    path: '/hooks',
    name: 'Hooks',
    component: () => import('./views/Hooks.vue'),
  },
]
