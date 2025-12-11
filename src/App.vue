<script setup lang="ts">
import { ref } from 'vue'
import { RouterView, useRoute } from 'vue-router'
import { Icon } from '@iconify/vue'
import { ThemeSelector, ToastContainer } from '@/core/components'
import { initLogger } from '@/core/composables'
import { appConfig } from '@/config/app.config'
import { AnnouncementBar, SettingsModal, useLiveStore } from '@/modules/live'

// === 可选模块（不需要可注释掉）===
import { LoginModal, useAuthStore } from '@/modules/auth'
const authStore = useAuthStore()

// 初始化日志系统
initLogger()

const route = useRoute()
const liveStore = useLiveStore()

// 公告内容
const announcement = ref('欢迎使用京东直播助手，请先选择浏览器并配置商品信息。')

// 设置弹窗
const showSettings = ref(false)

// 是否显示导航栏（投屏内容页面隐藏）
const showNavbar = () => !route.path.startsWith('/screen-content')
</script>

<template>
  <div class="h-screen flex flex-col bg-base-200">
    <!-- 导航栏 -->
    <div v-if="showNavbar()" class="navbar bg-base-100 shadow-lg px-4 shrink-0 gap-2">
      <!-- 主题选择 -->
      <ThemeSelector />
      <!-- 公告栏 -->
      <div class="flex-1">
        <AnnouncementBar :content="announcement" />
      </div>
      <!-- 设置按钮 -->
      <button class="btn btn-ghost btn-sm" @click="showSettings = true">
        <Icon icon="mdi:cog" class="text-lg" />
        软件设置
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

    <!-- 设置弹窗 -->
    <SettingsModal
      :visible="showSettings"
      :settings="liveStore.settings"
      @update:visible="showSettings = $event"
      @save="liveStore.saveSettings"
    />

    <!-- 登录弹窗（启用 auth 模块且未登录时显示） -->
    <LoginModal v-if="appConfig.features.auth && !authStore.isLoggedIn" />

    <!-- 全局 Toast 提示 -->
    <ToastContainer v-if="appConfig.features.toast" />
  </div>
</template>
