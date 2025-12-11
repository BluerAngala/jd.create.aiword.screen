<script setup lang="ts">
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import { useClipboard } from '@vueuse/core'
import { useCookieStore } from '../stores/cookie'
import { useLogger } from '@/core/composables'

const logger = useLogger('CookieReader')
const cookieStore = useCookieStore()
const { copy } = useClipboard()

// 输入的 URL
const urlInput = ref('')
// Chrome Profile 名称
const profileInput = ref('Default')
// 展开的 Cookie 值索引
const expandedValues = ref<Set<number>>(new Set())
// 复制成功提示
const copiedIndex = ref<number | null>(null)

// 常用 Profile 选项
const profileOptions = ['Default', 'Profile 1', 'Profile 2', 'Profile 3', 'Profile 8']

// 格式化过期时间
function formatExpires(timestamp: number | null): string {
  if (!timestamp) return '会话结束'
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('zh-CN')
}

// 切换展开状态
function toggleExpand(index: number) {
  if (expandedValues.value.has(index)) {
    expandedValues.value.delete(index)
  } else {
    expandedValues.value.add(index)
  }
}

// 复制 Cookie 值
async function copyValue(value: string, index: number) {
  await copy(value)
  copiedIndex.value = index
  logger.info(`已复制 Cookie 值`)
  setTimeout(() => {
    copiedIndex.value = null
  }, 2000)
}

// 读取 Cookie
async function handleReadCookies() {
  if (!urlInput.value.trim()) return
  logger.info(`开始读取 Cookie: ${urlInput.value}, Profile: ${profileInput.value}`)
  await cookieStore.fetchCookies(urlInput.value, profileInput.value)
}

// 导出 JSON
function handleExport() {
  const json = cookieStore.exportToJson()
  const blob = new Blob([json], { type: 'application/json' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `cookies-${cookieStore.lastDomain}-${Date.now()}.json`
  a.click()
  URL.revokeObjectURL(url)
  logger.info('Cookie 已导出')
}

// 是否有 Cookie 数据
const hasCookies = computed(() => cookieStore.cookies.length > 0)
</script>

<template>
  <div class="p-6 max-w-6xl mx-auto">
    <!-- 标题 -->
    <h1 class="text-2xl font-bold mb-6 flex items-center gap-2">
      <Icon icon="mdi:cookie" class="text-3xl text-warning" />
      Chrome Cookie 读取器
    </h1>

    <!-- 输入区域 -->
    <div class="card bg-base-200 shadow-lg mb-6">
      <div class="card-body">
        <div class="flex gap-4 flex-wrap">
          <input
            v-model="urlInput"
            type="text"
            placeholder="输入网站域名，例如：jd.com"
            class="input input-bordered flex-1 min-w-48"
            @keyup.enter="handleReadCookies"
          />
          <select v-model="profileInput" class="select select-bordered w-40">
            <option v-for="opt in profileOptions" :key="opt" :value="opt">
              {{ opt }}
            </option>
          </select>
          <input
            v-model="profileInput"
            type="text"
            placeholder="或输入 Profile 名称"
            class="input input-bordered w-48"
          />
          <button
            class="btn btn-primary"
            :disabled="!urlInput.trim() || cookieStore.loading"
            @click="handleReadCookies"
          >
            <span v-if="cookieStore.loading" class="loading loading-spinner loading-sm"></span>
            <Icon v-else icon="mdi:magnify" class="text-lg" />
            读取 Cookie
          </button>
        </div>
        <p class="text-sm text-base-content/50 mt-2">
          <Icon icon="mdi:information" class="inline" />
          使用 CDP 协议读取，需要先关闭 Chrome 浏览器
        </p>

        <!-- 错误提示 -->
        <div v-if="cookieStore.error" class="alert alert-error mt-4">
          <Icon icon="mdi:alert-circle" class="text-xl" />
          <span>{{ cookieStore.error }}</span>
        </div>
      </div>
    </div>

    <!-- Cookie 表格 -->
    <div v-if="hasCookies" class="card bg-base-200 shadow-lg">
      <div class="card-body">
        <div class="flex justify-between items-center mb-4">
          <h2 class="card-title">找到 {{ cookieStore.cookies.length }} 个 Cookie</h2>
          <button class="btn btn-secondary btn-sm" @click="handleExport">
            <Icon icon="mdi:download" class="text-lg" />
            导出 JSON
          </button>
        </div>

        <div class="overflow-x-auto">
          <table class="table table-zebra table-fixed w-full">
            <thead>
              <tr>
                <th class="w-32">名称</th>
                <th class="w-64">值</th>
                <th class="w-32">域名</th>
                <th class="w-16">路径</th>
                <th class="w-40">过期时间</th>
                <th class="w-24">属性</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(cookie, index) in cookieStore.cookies" :key="index">
                <td class="font-mono text-xs truncate" :title="cookie.name">{{ cookie.name }}</td>
                <td>
                  <div class="flex items-center gap-1">
                    <span
                      class="font-mono text-xs cursor-pointer hover:text-primary break-all"
                      :class="{ 'line-clamp-2': !expandedValues.has(index) }"
                      :title="cookie.value"
                      @click="copyValue(cookie.value, index)"
                    >
                      {{ cookie.value }}
                    </span>
                    <button
                      v-if="cookie.value.length > 30"
                      class="btn btn-ghost btn-xs shrink-0"
                      @click="toggleExpand(index)"
                    >
                      <Icon
                        :icon="expandedValues.has(index) ? 'mdi:chevron-up' : 'mdi:chevron-down'"
                      />
                    </button>
                    <span v-if="copiedIndex === index" class="badge badge-success badge-xs shrink-0">
                      已复制
                    </span>
                  </div>
                </td>
                <td class="font-mono text-xs truncate" :title="cookie.domain">
                  {{ cookie.domain }}
                </td>
                <td class="font-mono text-xs">{{ cookie.path }}</td>
                <td class="text-xs whitespace-nowrap">{{ formatExpires(cookie.expires) }}</td>
                <td>
                  <div class="flex gap-1 flex-wrap">
                    <span v-if="cookie.isSecure" class="badge badge-info badge-xs">S</span>
                    <span v-if="cookie.isHttpOnly" class="badge badge-warning badge-xs">H</span>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- 空状态 -->
    <div
      v-else-if="!cookieStore.loading && !cookieStore.error && urlInput"
      class="card bg-base-200 shadow-lg"
    >
      <div class="card-body items-center text-center">
        <Icon icon="mdi:cookie-off" class="text-6xl text-base-content/30" />
        <p class="text-base-content/50">输入网站 URL 并点击读取按钮</p>
      </div>
    </div>
  </div>
</template>
