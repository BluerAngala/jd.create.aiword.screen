<script setup lang="ts">
import { ref } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import { Icon } from '@iconify/vue'
import { openUrl } from '@tauri-apps/plugin-opener'
import { ThemeSelector, ToastContainer } from '@/core/components'
import { initLogger } from '@/core/composables'
import { appConfig } from '@/config/app.config'
import { AnnouncementBar } from '@/modules/live'

// === 可选模块（不需要可注释掉）===
import { LoginModal, useAuthStore } from '@/modules/auth'
import { useLiveStore } from '@/modules/live'
const authStore = useAuthStore()
const liveStore = useLiveStore()

// 初始化日志系统
initLogger()

const route = useRoute()

// 公告内容
const announcement = ref('欢迎使用JD直播助手PLUS版，请先选择浏览器并配置商品信息。')

// 打开个人中心
const openUserCenter = () => {
  openUrl('https://bluerangala.feishu.cn/docx/TyoBdH9p3o2aK8xEtNbcBY2bnNg?from=from_copylink')
}

// 是否显示导航栏（投屏页面隐藏）
const showNavbar = () =>
  !['ScreenContent', 'ScreenCountdown', 'ScreenScript'].includes(route.name as string)
</script>

<template>
  <div class="h-screen flex flex-col bg-base-200">
    <!-- 导航栏 -->
    <div v-if="showNavbar()" class="navbar bg-base-100 shadow-lg px-4 shrink-0 gap-2">
      <!-- 主题选择 -->
      <ThemeSelector />
      <!-- 公告栏 -->
      <div class="flex-1 ">
        <AnnouncementBar :content="announcement" />
      </div>
      <!-- 自动讲解开关 -->
      <label class="btn btn-ghost btn-sm cursor-pointer">
        <input
          v-model="liveStore.autoExplainEnabled"
          type="checkbox"
          class="checkbox checkbox-success checkbox-sm"
        />
        <span :class="liveStore.autoExplainEnabled ? 'text-success' : ''">自动讲解</span>
      </label>
      <!-- 使用教程 -->
      <button class="btn btn-sm btn-info" @click="openUserCenter">
        <Icon icon="mdi:book-open-page-variant" class="text-lg" />
        使用教程
      </button>
      <!-- 退出登录 -->
      <button
        v-if="appConfig.features.auth && authStore.isLoggedIn"
        class="btn btn-sm bg-red-500 hover:bg-red-600 text-white border-none"
        @click="authStore.logout()"
      >
        退出登录
      </button>
    </div>

    <!-- 主内容区 -->
    <main class="flex-1 overflow-hidden">
      <RouterView />
    </main>

    <!-- 登录弹窗（启用 auth 模块且未登录时显示） -->
    <LoginModal v-if="appConfig.features.auth && !authStore.isLoggedIn" />

    <!-- 全局 Toast 提示 -->
    <ToastContainer v-if="appConfig.features.toast" />
  </div>
</template>
