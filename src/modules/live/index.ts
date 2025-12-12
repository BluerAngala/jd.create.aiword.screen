/**
 * 京东直播助手模块
 */

// 导出类型
export * from './types'

// 导出 API
export * as jdApi from './api/jd'

// 导出 Store
export { useLiveStore } from './stores/live'

// 导出组件
export { default as AnnouncementBar } from './components/AnnouncementBar.vue'
export { default as SettingsModal } from './components/SettingsModal.vue'
export { default as BrowserList } from './components/BrowserList.vue'
export { default as ProductConfig } from './components/ProductConfig.vue'
export { default as ImageConfig } from './components/ImageConfig.vue'
export { default as ExecutionLog } from './components/ExecutionLog.vue'
export { default as LiveParams } from './components/LiveParams.vue'
export { default as CountdownTimer } from './components/CountdownTimer.vue'
export { default as AIScriptPanel } from './components/AIScriptPanel.vue'
export { default as AISettingsModal } from './components/AISettingsModal.vue'

// 导出页面
export { default as LiveAssistant } from './views/LiveAssistant.vue'
