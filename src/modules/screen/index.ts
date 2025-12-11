/**
 * 投屏模块
 * 提供 OBS 投屏窗口创建功能
 *
 * 依赖：
 * - Rust 端需要 create_screen_window / close_screen_window 命令
 *
 * 不需要此模块时：
 * 1. 删除整个 modules/screen 文件夹
 * 2. 在 router/index.ts 中移除 screenRoutes
 * 3. 在 config/app.config.ts 中移除 screen 导航项
 */

import type { RouteRecordRaw } from 'vue-router'

// 路由配置
export const screenRoutes: RouteRecordRaw[] = [
  {
    path: '/screen',
    name: 'Screen',
    component: () => import('./views/ScreenWindow.vue'),
  },
  {
    path: '/screen-content',
    name: 'ScreenContent',
    component: () => import('./views/ScreenContent.vue'),
  },
]
