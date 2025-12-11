/**
 * 认证模块
 * 提供卡密登录校验功能
 *
 * 使用方式：
 * 1. 在 App.vue 中导入 LoginModal 组件
 * 2. 在需要的地方使用 useAuthStore
 *
 * 不需要此模块时：
 * 1. 删除整个 modules/auth 文件夹
 * 2. 在 App.vue 中移除相关导入和组件
 * 3. 在 config/app.config.ts 中设置 features.auth = false
 */

export { useAuthStore } from './stores/auth'
export type { AuthInfo } from './stores/auth'
export { default as LoginModal } from './components/LoginModal.vue'
