<script setup lang="ts">
/**
 * 浏览器列表组件
 * 显示本地 Chrome 浏览器配置文件列表，支持选择和获取 Cookie
 */
import { ref, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import type { BrowserInfo } from '../types'
import { useLiveStore } from '../stores/live'

interface ChromeProfile {
  id: string
  name: string
  profile_path: string
}

interface Cookie {
  name: string
  value: string
  domain: string
  path: string
  expires: number | null
  is_secure: boolean
  is_http_only: boolean
}

interface Props {
  browsers: BrowserInfo[]
  selectedId?: string | null
  loading?: boolean
  expanded?: boolean
}

interface Emits {
  (e: 'select', browser: BrowserInfo): void
  (e: 'refresh'): void
  (e: 'update:browsers', browsers: BrowserInfo[]): void
  (e: 'toggle'): void
}

const props = withDefaults(defineProps<Props>(), {
  selectedId: null,
  loading: false,
  expanded: false,
})

const emit = defineEmits<Emits>()
const liveStore = useLiveStore()
const isLoading = ref(false)
const isFetchingCookie = ref(false)

// 组件挂载时自动加载浏览器列表
onMounted(() => {
  if (props.browsers.length === 0) {
    loadBrowserProfiles()
  }
})

/**
 * 加载浏览器配置文件列表
 */
async function loadBrowserProfiles() {
  isLoading.value = true
  liveStore.addLog('info', '正在扫描本地 Chrome 浏览器配置文件...')

  try {
    const profiles = await invoke<ChromeProfile[]>('get_browser_profiles')

    const browsers: BrowserInfo[] = profiles.map((p) => ({
      id: p.id,
      name: p.name,
      profilePath: p.profile_path,
    }))

    emit('update:browsers', browsers)
    liveStore.addLog('success', `找到 ${browsers.length} 个浏览器配置文件`)
  } catch (error) {
    liveStore.addLog('error', `扫描浏览器失败: ${error}`)
  } finally {
    isLoading.value = false
  }
}

/**
 * 获取指定浏览器的京东 Cookie
 */
async function fetchJdCookies(browser: BrowserInfo) {
  isFetchingCookie.value = true
  liveStore.addLog('info', `正在获取 ${browser.name} 的京东 Cookie...`)

  try {
    const cookies = await invoke<Cookie[]>('read_chrome_cookies', {
      domain: 'jd.com',
      profile: browser.id,
    })

    // 打印 Cookie 到日志
    liveStore.addLog('success', `获取到 ${cookies.length} 个京东域名的 Cookie`)

    // 打印每个 Cookie 的详细信息
    for (const cookie of cookies) {
      liveStore.addLog('info', `Cookie: ${cookie.name} = ${cookie.value.substring(0, 20)}... (${cookie.domain})`)
    }

    // 尝试从 Cookie 中获取用户昵称（通常在 pin 或 unick 中）
    const pinCookie = cookies.find((c) => c.name === 'pin' || c.name === 'unick')
    const nickname = pinCookie ? decodeURIComponent(pinCookie.value) : undefined

    // 更新浏览器信息
    const updatedBrowsers = props.browsers.map((b) => {
      if (b.id === browser.id) {
        return {
          ...b,
          jdAccount: nickname ? { nickname, isLoggedIn: true } : { nickname: '未登录', isLoggedIn: false },
        }
      }
      return b
    })

    emit('update:browsers', updatedBrowsers)

    if (nickname) {
      liveStore.addLog('success', `京东账号: ${nickname}`)
    } else {
      liveStore.addLog('warn', '未检测到京东登录状态')
    }
  } catch (error) {
    liveStore.addLog('error', `获取 Cookie 失败: ${error}`)

    // 更新为未登录状态
    const updatedBrowsers = props.browsers.map((b) => {
      if (b.id === browser.id) {
        return { ...b, jdAccount: { nickname: '获取失败', isLoggedIn: false } }
      }
      return b
    })
    emit('update:browsers', updatedBrowsers)
  } finally {
    isFetchingCookie.value = false
  }
}

async function handleSelect(browser: BrowserInfo) {
  emit('select', browser)
  await fetchJdCookies(browser)
}

function handleRefresh() {
  loadBrowserProfiles()
  emit('refresh')
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer" @click="emit('toggle')">
      <Icon icon="mdi:google-chrome" class="text-lg" />
      <span class="text-sm font-medium flex-1">浏览器列表</span>
      <button class="btn btn-ghost btn-xs" :disabled="isLoading || loading" @click.stop="handleRefresh">
        <Icon icon="mdi:refresh" :class="{ 'animate-spin': isLoading || loading }" />
      </button>
    </div>
    <div v-show="expanded" class="collapse-content px-3 pb-2">
      <!-- 空状态 -->
      <div v-if="browsers.length === 0 && !isLoading && !loading" class="text-center py-2 text-base-content/60">
        <p class="text-xs">未检测到 Chrome 浏览器配置文件</p>
      </div>
      <!-- 加载状态 -->
      <div v-else-if="isLoading || loading" class="flex justify-center py-2">
        <span class="loading loading-spinner loading-sm"></span>
      </div>
      <!-- 浏览器列表 -->
      <div v-else class="space-y-1">
        <div
          v-for="browser in browsers"
          :key="browser.id"
          class="flex items-center gap-2 p-2 rounded cursor-pointer text-sm"
          :class="[selectedId === browser.id ? 'bg-primary/10' : 'hover:bg-base-200', isFetchingCookie ? 'opacity-50 pointer-events-none' : '']"
          @click="handleSelect(browser)"
        >
          <span class="truncate flex-1">{{ browser.name }}</span>
          <span v-if="browser.jdAccount?.isLoggedIn" class="text-xs text-success">
            {{ browser.jdAccount.nickname }}
          </span>
          <span v-else-if="browser.jdAccount" class="text-xs text-base-content/50">
            {{ browser.jdAccount.nickname }}
          </span>
        </div>
      </div>
    </div>
  </div>
</template>
