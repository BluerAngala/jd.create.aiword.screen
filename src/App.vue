<script setup lang="ts">
import { RouterView, RouterLink, useRoute } from 'vue-router'
import { ThemeSelector, ToastContainer } from '@/core/components'
import { initLogger } from '@/core/composables'
import { appConfig, navItems } from '@/config/app.config'

// === 可选模块（不需要可注释掉）===
import { LoginModal, useAuthStore } from '@/modules/auth'
const authStore = useAuthStore()

// 初始化日志系统
initLogger()

const route = useRoute()

// 过滤启用的导航项
const enabledNavItems = navItems.filter((item) => item.enabled)

// 是否显示导航栏（投屏内容页面隐藏）
const showNavbar = () => !route.path.startsWith('/screen-content')
</script>

<template>
  <div class="min-h-screen bg-base-200">
    <!-- 导航栏 -->
    <div v-if="showNavbar()" class="navbar bg-base-100 shadow-lg">
      <div class="flex-1">
        <span class="text-xl font-bold px-4">{{ appConfig.name }}</span>
      </div>
      <div class="flex-none gap-2">
        <ul class="menu menu-horizontal px-1">
          <li v-for="item in enabledNavItems" :key="item.path">
            <RouterLink :to="item.path" :class="{ active: route.path === item.path }">
              {{ item.label }}
            </RouterLink>
          </li>
        </ul>
        <ThemeSelector />
        <!-- 登录状态（启用 auth 模块时显示） -->
        <button
          v-if="appConfig.features.auth && authStore.isLoggedIn"
          class="btn btn-ghost btn-sm"
          @click="authStore.logout()"
        >
          退出
        </button>
      </div>
    </div>

    <!-- 主内容区 -->
    <main :class="route.path.startsWith('/screen-content') ? '' : 'container mx-auto p-8 max-w-4xl'">
      <RouterView />
    </main>

    <!-- 登录弹窗（启用 auth 模块且未登录时显示） -->
    <LoginModal v-if="appConfig.features.auth && !authStore.isLoggedIn" />

    <!-- 全局 Toast 提示 -->
    <ToastContainer v-if="appConfig.features.toast" />
  </div>
</template>
