<script setup lang="ts">
/**
 * 浏览器列表组件
 * 显示本地 Chrome 浏览器列表，支持选择和刷新
 */
import { Icon } from '@iconify/vue'
import type { BrowserInfo } from '../types'

interface Props {
  browsers: BrowserInfo[]
  selectedId?: string | null
  loading?: boolean
  accordionName?: string
}

interface Emits {
  (e: 'select', browser: BrowserInfo): void
  (e: 'refresh'): void
}

withDefaults(defineProps<Props>(), {
  selectedId: null,
  loading: false,
  accordionName: undefined,
})

const emit = defineEmits<Emits>()

function handleSelect(browser: BrowserInfo) {
  emit('select', browser)
}

function handleRefresh() {
  emit('refresh')
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm">
    <input type="radio" :name="accordionName" />
    <div class="collapse-title py-2 px-3 min-h-0 flex items-center gap-2">
      <Icon icon="mdi:google-chrome" class="text-lg" />
      <span class="text-sm font-medium">浏览器列表</span>
      <button class="btn btn-ghost btn-xs ml-auto" :disabled="loading" @click.stop="handleRefresh">
        <Icon icon="mdi:refresh" :class="{ 'animate-spin': loading }" />
      </button>
    </div>
    <div class="collapse-content px-3 pb-2">
      <!-- 空状态 -->
      <div v-if="browsers.length === 0 && !loading" class="text-center py-2 text-base-content/60">
        <p class="text-xs">未检测到 Chrome 浏览器</p>
      </div>
      <!-- 加载状态 -->
      <div v-else-if="loading" class="flex justify-center py-2">
        <span class="loading loading-spinner loading-sm"></span>
      </div>
      <!-- 浏览器列表 -->
      <div v-else class="space-y-1">
        <div
          v-for="browser in browsers"
          :key="browser.id"
          class="flex items-center gap-2 p-2 rounded cursor-pointer text-sm"
          :class="selectedId === browser.id ? 'bg-primary/10' : 'hover:bg-base-200'"
          @click="handleSelect(browser)"
        >
          <span class="truncate flex-1">{{ browser.name }}</span>
          <span v-if="browser.jdAccount?.isLoggedIn" class="text-xs text-success">
            {{ browser.jdAccount.nickname }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
