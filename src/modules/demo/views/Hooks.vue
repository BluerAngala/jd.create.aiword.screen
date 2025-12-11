<script setup lang="ts">
import { ref } from 'vue'
import { Icon } from '@iconify/vue'
import {
  useMouse,
  useWindowSize,
  useDark,
  useToggle,
  useClipboard,
  useLocalStorage,
  useOnline,
  useBattery,
  usePreferredLanguages,
  useElementHover,
} from '@vueuse/core'

// 鼠标位置
const { x, y } = useMouse()

// 窗口大小
const { width, height } = useWindowSize()

// 深色模式
const isDark = useDark()
const toggleDark = useToggle(isDark)

// 剪贴板
const { copy, copied } = useClipboard()
const textToCopy = ref('Hello VueUse!')

// 本地存储
const counter = useLocalStorage('demo-counter', 0)

// 网络状态
const isOnline = useOnline()

// 电池状态
const { charging, level } = useBattery()

// 语言偏好
const languages = usePreferredLanguages()

// 元素悬停
const hoverRef = ref<HTMLElement>()
const isHovered = useElementHover(hoverRef)
</script>

<template>
  <div class="space-y-8">
    <h2 class="text-2xl font-bold">VueUse 示例</h2>

    <div class="alert alert-info">
      <Icon icon="mdi:information" class="text-xl" />
      <span>VueUse 提供 200+ 实用组合式函数，以下是常用示例。</span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <!-- 鼠标位置 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:cursor-default" class="text-primary" />
            useMouse
          </h3>
          <p class="text-base-content/70">实时追踪鼠标位置</p>
          <div class="stats bg-base-200">
            <div class="stat">
              <div class="stat-title">X</div>
              <div class="stat-value text-primary">{{ x }}</div>
            </div>
            <div class="stat">
              <div class="stat-title">Y</div>
              <div class="stat-value text-secondary">{{ y }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 窗口大小 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:resize" class="text-secondary" />
            useWindowSize
          </h3>
          <p class="text-base-content/70">响应式窗口尺寸</p>
          <div class="stats bg-base-200">
            <div class="stat">
              <div class="stat-title">宽度</div>
              <div class="stat-value text-primary">{{ width }}</div>
            </div>
            <div class="stat">
              <div class="stat-title">高度</div>
              <div class="stat-value text-secondary">{{ height }}</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 剪贴板 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:clipboard" class="text-accent" />
            useClipboard
          </h3>
          <p class="text-base-content/70">剪贴板操作</p>
          <div class="flex gap-2 mt-2">
            <input v-model="textToCopy" class="input input-bordered flex-1" />
            <button class="btn btn-primary" @click="copy(textToCopy)">
              <Icon :icon="copied ? 'mdi:check' : 'mdi:content-copy'" />
              {{ copied ? '已复制' : '复制' }}
            </button>
          </div>
        </div>
      </div>

      <!-- 本地存储 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:database" class="text-warning" />
            useLocalStorage
          </h3>
          <p class="text-base-content/70">持久化存储（刷新保留）</p>
          <div class="flex items-center gap-4 mt-2">
            <button class="btn btn-circle btn-outline" @click="counter--">
              <Icon icon="mdi:minus" />
            </button>
            <span class="text-3xl font-bold">{{ counter }}</span>
            <button class="btn btn-circle btn-outline" @click="counter++">
              <Icon icon="mdi:plus" />
            </button>
          </div>
        </div>
      </div>

      <!-- 网络状态 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:wifi" class="text-success" />
            useOnline
          </h3>
          <p class="text-base-content/70">网络连接状态</p>
          <div class="flex items-center gap-2 mt-2">
            <div class="badge badge-lg" :class="isOnline ? 'badge-success' : 'badge-error'">
              <Icon :icon="isOnline ? 'mdi:wifi' : 'mdi:wifi-off'" class="mr-1" />
              {{ isOnline ? '在线' : '离线' }}
            </div>
          </div>
        </div>
      </div>

      <!-- 电池状态 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:battery" class="text-error" />
            useBattery
          </h3>
          <p class="text-base-content/70">电池信息</p>
          <div class="flex items-center gap-4 mt-2">
            <div
              class="radial-progress text-primary"
              :style="`--value:${(level ?? 0) * 100}`"
            >
              {{ Math.round((level ?? 0) * 100) }}%
            </div>
            <div class="badge" :class="charging ? 'badge-success' : 'badge-ghost'">
              <Icon :icon="charging ? 'mdi:battery-charging' : 'mdi:battery'" class="mr-1" />
              {{ charging ? '充电中' : '未充电' }}
            </div>
          </div>
        </div>
      </div>

      <!-- 元素悬停 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:cursor-pointer" class="text-info" />
            useElementHover
          </h3>
          <p class="text-base-content/70">检测元素悬停状态</p>
          <div
            ref="hoverRef"
            class="mt-2 p-8 rounded-lg text-center transition-all cursor-pointer"
            :class="isHovered ? 'bg-primary text-primary-content' : 'bg-base-200'"
          >
            {{ isHovered ? '悬停中！' : '把鼠标移到这里' }}
          </div>
        </div>
      </div>

      <!-- 语言偏好 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:translate" class="text-purple-500" />
            usePreferredLanguages
          </h3>
          <p class="text-base-content/70">浏览器语言偏好</p>
          <div class="flex flex-wrap gap-2 mt-2">
            <div v-for="lang in languages" :key="lang" class="badge badge-outline">
              {{ lang }}
            </div>
          </div>
        </div>
      </div>

      <!-- 深色模式 -->
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h3 class="card-title">
            <Icon icon="mdi:theme-light-dark" class="text-orange-500" />
            useDark
          </h3>
          <p class="text-base-content/70">深色模式切换</p>
          <div class="flex items-center gap-4 mt-2">
            <button
              class="btn"
              :class="isDark ? 'btn-primary' : 'btn-outline'"
              @click="toggleDark()"
            >
              <Icon :icon="isDark ? 'mdi:weather-night' : 'mdi:weather-sunny'" />
              {{ isDark ? '深色模式' : '浅色模式' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
