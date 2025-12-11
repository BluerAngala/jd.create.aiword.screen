// Cookie Store - 管理 Chrome Cookie 数据
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// Cookie 数据接口
export interface Cookie {
  name: string
  value: string
  domain: string
  path: string
  expires: number | null
  isSecure: boolean
  isHttpOnly: boolean
}

export const useCookieStore = defineStore('cookie', () => {
  // 状态
  const cookies = ref<Cookie[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const lastDomain = ref('')

  // 当前使用的 Chrome Profile
  const currentProfile = ref<string | null>(null)

  // 读取 Chrome Cookie（使用 CDP 协议）
  async function fetchCookies(domain: string, profile?: string) {
    loading.value = true
    error.value = null
    lastDomain.value = domain
    currentProfile.value = profile || null

    try {
      const result = await invoke<Cookie[]>('read_chrome_cookies', {
        domain,
        profile: profile || null,
      })
      // 转换字段名（Rust 使用 snake_case，前端使用 camelCase）
      cookies.value = result.map((c) => ({
        name: c.name,
        value: c.value,
        domain: c.domain,
        path: c.path,
        expires: (c as unknown as { expires: number | null }).expires,
        isSecure: (c as unknown as { is_secure: boolean }).is_secure,
        isHttpOnly: (c as unknown as { is_http_only: boolean }).is_http_only,
      }))
    } catch (e) {
      error.value = e as string
      cookies.value = []
    } finally {
      loading.value = false
    }
  }

  // 清空 Cookie
  function clearCookies() {
    cookies.value = []
    error.value = null
    lastDomain.value = ''
  }

  // 导出为 JSON
  function exportToJson(): string {
    return JSON.stringify(cookies.value, null, 2)
  }

  return {
    cookies,
    loading,
    error,
    lastDomain,
    currentProfile,
    fetchCookies,
    clearCookies,
    exportToJson,
  }
})
