import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useToast } from '@/core/composables'

// 卡密校验接口地址
const VERIFY_API =
  'https://env-00jxu65bfie3.dev-hz.cloudbasefunction.cn/http/router/admin/card/pub/verify'

// 产品 ID（根据实际情况修改）
const PRODUCT_ID = 'exe-explain'

// 本地存储 key
const STORAGE_KEY = 'auth_info'

// 检查间隔（1 小时）
const CHECK_INTERVAL = 60 * 60 * 1000

// 授权信息类型
export interface AuthInfo {
  cardCode: string
  userId: string
  cardId: string
  productName: string
  expireTime: number
  expireTimeText: string
  activateTimeText: string
  remainingTimes: number
  hasTimeLimit: boolean
  hasTimesLimit: boolean
  authorizedMachines: string[]
  currentMachineCount: number
  maxMachineCount: number
}

export const useAuthStore = defineStore('auth', () => {
  const toast = useToast()

  // 状态
  const authInfo = ref<AuthInfo | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)
  const machineCode = ref<string>('')

  // 计算属性
  const isLoggedIn = computed(() => !!authInfo.value)
  const isExpired = computed(() => {
    if (!authInfo.value?.hasTimeLimit) return false
    return authInfo.value.expireTime < Date.now()
  })

  // 获取机器码
  async function getMachineCode(): Promise<string> {
    if (machineCode.value) return machineCode.value
    try {
      const code = await invoke<string>('get_machine_code')
      machineCode.value = code
      return code
    } catch {
      // 如果 Tauri 命令失败，使用备用方案
      const fallback = `web_${navigator.userAgent.slice(0, 32)}_${screen.width}x${screen.height}`
      machineCode.value = fallback
      return fallback
    }
  }

  // 从本地存储加载
  function loadFromStorage() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY)
      if (stored) {
        const data = JSON.parse(stored) as AuthInfo
        // 检查是否过期
        if (data.hasTimeLimit && data.expireTime < Date.now()) {
          localStorage.removeItem(STORAGE_KEY)
          return false
        }
        authInfo.value = data
        return true
      }
    } catch {
      localStorage.removeItem(STORAGE_KEY)
    }
    return false
  }

  // 保存到本地存储
  function saveToStorage() {
    if (authInfo.value) {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(authInfo.value))
    }
  }

  // 登录（卡密校验）
  async function login(cardCode: string, userId: string): Promise<boolean> {
    isLoading.value = true
    error.value = null

    try {
      const machine = await getMachineCode()

      const body = JSON.stringify({
        key: cardCode,
        product_id: PRODUCT_ID,
        machineCode: machine,
        id: userId,
      })

      // 使用 Tauri 后端发送请求，绕过 CORS 限制
      const responseText = await invoke<string>('http_post', { url: VERIFY_API, body })
      const result = JSON.parse(responseText)
      console.log('[Auth] 登录响应:', result)

      if (result.code === 0) {
        const data = result.data
        authInfo.value = {
          cardCode,
          userId,
          cardId: data.card_id,
          productName: data.product_name,
          expireTime: data.expire_time,
          expireTimeText: data.expire_time_text,
          activateTimeText: data.activate_time_text,
          remainingTimes: data.remaining_times,
          hasTimeLimit: data.has_time_limit,
          hasTimesLimit: data.has_times_limit,
          authorizedMachines: data.authorized_machines,
          currentMachineCount: data.current_machine_count,
          maxMachineCount: data.max_machine_count,
        }
        saveToStorage()
        startAuthCheck() // 登录成功后启动定时检查
        toast.success(`登录成功！有效期至：${data.expire_time_text || '永久'}`)
        return true
      } else {
        error.value = result.msg || '校验失败'
        toast.error(result.msg || '校验失败')
        return false
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : '网络请求失败'
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 退出登录
  function logout() {
    authInfo.value = null
    localStorage.removeItem(STORAGE_KEY)
    stopAuthCheck()
  }

  // 检查登录状态（向服务器验证卡密是否仍然有效）
  async function checkAuthStatus(): Promise<boolean> {
    if (!authInfo.value) return false

    try {
      const machine = await getMachineCode()
      const body = JSON.stringify({
        key: authInfo.value.cardCode,
        product_id: PRODUCT_ID,
        machineCode: machine,
        id: authInfo.value.userId,
      })

      const responseText = await invoke<string>('http_post', { url: VERIFY_API, body })
      const result = JSON.parse(responseText)
      console.log('[Auth] 状态检查响应:', result)

      if (result.code === 0) {
        // 更新授权信息
        const data = result.data
        authInfo.value = {
          ...authInfo.value,
          expireTime: data.expire_time,
          expireTimeText: data.expire_time_text,
          remainingTimes: data.remaining_times,
        }
        saveToStorage()
        return true
      } else {
        // 卡密已失效，强制退出
        toast.error(`授权已失效：${result.msg || '请重新登录'}`)
        logout()
        return false
      }
    } catch (e) {
      console.error('[Auth] 状态检查失败:', e)
      // 网络错误时检查本地过期时间
      if (authInfo.value?.hasTimeLimit && authInfo.value.expireTime < Date.now()) {
        toast.error('授权已过期，请重新登录')
        logout()
        return false
      }
      return true // 网络错误但本地未过期，暂时允许使用
    }
  }

  // 定时检查器
  let checkTimer: ReturnType<typeof setInterval> | null = null

  function startAuthCheck() {
    if (checkTimer) return
    // 立即检查一次
    checkAuthStatus()
    // 每小时检查一次
    checkTimer = setInterval(() => {
      checkAuthStatus()
    }, CHECK_INTERVAL)
  }

  function stopAuthCheck() {
    if (checkTimer) {
      clearInterval(checkTimer)
      checkTimer = null
    }
  }

  // 初始化时加载并启动检查
  const loaded = loadFromStorage()
  if (loaded) {
    startAuthCheck()
  }

  return {
    authInfo,
    isLoading,
    error,
    machineCode,
    isLoggedIn,
    isExpired,
    login,
    logout,
    getMachineCode,
    loadFromStorage,
    checkAuthStatus,
    startAuthCheck,
    stopAuthCheck,
  }
})
