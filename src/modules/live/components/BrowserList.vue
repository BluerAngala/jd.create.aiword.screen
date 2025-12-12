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
import { useToast } from '@/core/composables/useToast'

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
const toast = useToast()
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

// 京东登录验证结果类型
interface JdLoginResult {
  is_logged_in: boolean
  nickname: string | null
  avatar: string | null
}

/**
 * 获取指定浏览器的京东 Cookie
 */
async function fetchJdCookies(browser: BrowserInfo) {
  isFetchingCookie.value = true
  liveStore.addLog('info', `正在获取 ${browser.name} 的京东 Cookie...`)

  try {
    // 获取所有 jd.com 及其子域名的 Cookie
    const cookies = await invoke<Cookie[]>('read_chrome_cookies', {
      domain: 'jd.com',
      profile: browser.id,
    })

    // 打印 Cookie 结果
    liveStore.addLog('success', `获取到 ${cookies.length} 个京东域名的 Cookie`)

    // 通过后端 API 验证京东登录状态
    liveStore.addLog('info', '正在验证京东登录状态...')
    const loginResult = await invoke<JdLoginResult>('verify_jd_login', { cookies })

    // 只有登录成功才保存 Cookie
    if (loginResult.is_logged_in && cookies.length > 0) {
      try {
        const filename = `jd_cookies_${browser.id.replace(/\s+/g, '_')}.json`
        const savedPath = await invoke<string>('save_cookies_to_file', {
          cookies,
          filename,
        })
        liveStore.addLog('success', `Cookie 已保存到: ${savedPath}`)
      } catch (saveError) {
        liveStore.addLog('warn', `Cookie 保存失败: ${saveError}`)
      }
    }

    // 更新浏览器信息
    const updatedBrowsers = props.browsers.map((b) => {
      if (b.id === browser.id) {
        return {
          ...b,
          jdAccount: loginResult.is_logged_in
            ? { nickname: loginResult.nickname || '已登录', isLoggedIn: true }
            : { nickname: '未登录', isLoggedIn: false },
        }
      }
      return b
    })

    emit('update:browsers', updatedBrowsers)

    if (loginResult.is_logged_in) {
      liveStore.addLog('success', `京东账号: ${loginResult.nickname}`)
    } else {
      liveStore.addLog('warn', '未检测到京东登录状态')
    }
  } catch (error) {
    const errorMsg = String(error)
    liveStore.addLog('error', `获取 Cookie 失败: ${errorMsg}`)

    // 检查是否是浏览器已打开的错误
    if (errorMsg.includes('浏览器启动失败') || errorMsg.includes('BrowserLaunchFailed')) {
      toast.error('请先关闭 Chrome 浏览器，然后重新点击获取')
    } else {
      toast.error(`获取 Cookie 失败: ${errorMsg}`)
    }

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
    <div v-if="expanded" class="px-3 pb-2">
      <!-- 空状态 -->
      <div v-if="browsers.length === 0 && !isLoading && !loading" class="text-center py-2 text-base-content/60">
        <p class="text-xs">未检测到 Chrome 浏览器配置文件</p>
      </div>
      <!-- 加载状态 -->
      <div v-else-if="isLoading || loading" class="flex justify-center py-2">
        <span class="loading loading-spinner loading-sm"></span>
      </div>
      <!-- 浏览器列表 -->
      <div v-else class="space-y-2">
        <div
          v-for="browser in browsers"
          :key="browser.id"
          class="flex items-center gap-3 p-3 rounded-lg border cursor-pointer transition-all"
          :class="[
            selectedId === browser.id
              ? 'bg-primary/10 border-primary'
              : 'border-base-300 hover:bg-base-200 hover:border-base-content/20',
            isFetchingCookie ? 'opacity-50 pointer-events-none' : '',
          ]"
          @click="handleSelect(browser)"
        >
          <Icon icon="mdi:account-circle" class="text-2xl text-base-content/40" />
          <div class="flex-1 min-w-0">
            <div class="font-medium text-sm truncate">{{ browser.name }}</div>
            <div v-if="browser.jdAccount?.isLoggedIn" class="text-xs text-success flex items-center gap-1">
              <Icon icon="mdi:check-circle" class="text-xs" />
              {{ browser.jdAccount.nickname }}
            </div>
            <div v-else-if="browser.jdAccount" class="text-xs text-base-content/50">
              {{ browser.jdAccount.nickname }}
            </div>
            <div v-else class="text-xs text-base-content/40">点击获取登录状态</div>
          </div>
          <Icon v-if="selectedId === browser.id" icon="mdi:check" class="text-primary" />
        </div>
      </div>
    </div>
  </div>
</template>
