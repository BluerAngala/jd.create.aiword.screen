/**
 * Cookie 读取模块
 * 提供 Chrome Cookie 读取功能（通过 CDP 协议）
 *
 * 依赖：
 * - Rust 端需要 chromiumoxide 依赖
 *
 * 不需要此模块时：
 * 1. 删除整个 modules/cookie 文件夹
 * 2. 在 router/index.ts 中移除 cookieRoutes
 * 3. 在 config/app.config.ts 中移除 cookie 导航项
 * 4. 在 Cargo.toml 中注释掉 chromiumoxide 依赖
 */

import type { RouteRecordRaw } from 'vue-router'

export { useCookieStore } from './stores/cookie'
export type { Cookie } from './stores/cookie'

// 路由配置
export const cookieRoutes: RouteRecordRaw[] = [
  {
    path: '/cookie',
    name: 'Cookie',
    component: () => import('./views/CookieReader.vue'),
  },
]
