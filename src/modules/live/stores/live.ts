import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type {
  AppSettings,
  BrowserInfo,
  ProductItem,
  ProductFile,
  ImageSettings,
  LiveParameters,
  LogEntry,
  AIScript,
  AIScriptSettings,
  LiveSession,
  LiveProduct,
} from '../types'
import {
  STORAGE_KEYS,
  DEFAULT_AI_MODEL,
  AI_PROMPTS,
  DEFAULTS,
  LIVE_CONFIG,
} from '@/config/constants'

// 默认设置
const defaultSettings: AppSettings = {
  theme: 'light',
  loginInfo: {},
  aiConfig: {
    enabled: false,
  },
}

// 默认图片配置
const defaultImageConfig: ImageSettings = {
  width: DEFAULTS.IMAGE_WIDTH,
  height: DEFAULTS.IMAGE_HEIGHT,
}

// 默认直播参数（开始时间为当前时间 + 3 分钟）
const getDefaultLiveParams = (): LiveParameters => ({
  totalProducts: DEFAULTS.TOTAL_PRODUCTS,
  cartProducts: DEFAULTS.CART_PRODUCTS,
  startTime: new Date(Date.now() + LIVE_CONFIG.MIN_START_TIME_MINUTES * 60 * 1000),
})

export const useLiveStore = defineStore('live', () => {
  // 设置
  const settings = ref<AppSettings>(loadSettings())

  // 浏览器
  const browsers = ref<BrowserInfo[]>([])
  const selectedBrowserId = ref<string | null>(null)

  // 商品
  const products = ref<ProductItem[]>([])

  // 商品文件列表（持久化）
  const productFiles = ref<ProductFile[]>(loadProductFiles())

  // 图片配置
  const imageConfig = ref<ImageSettings>({ ...defaultImageConfig })

  // 直播参数
  const liveParams = ref<LiveParameters>(getDefaultLiveParams())

  // 执行状态
  const logs = ref<LogEntry[]>([])
  const isLiveRoomCreated = ref(false)
  const isLiveStarted = ref(false)
  const countdownTargetTime = ref<Date | null>(null)
  const countdownRunning = ref(false)
  const liveId = ref<number | null>(null)

  // AI 话术
  const aiScripts = ref<AIScript[]>([])
  const currentScriptIndex = ref(0)
  const aiScriptSettings = ref<AIScriptSettings>(loadAISettings())

  // 直播场次数据（持久化）
  const liveSessions = ref<LiveSession[]>(loadLiveSessions())
  const currentSession = ref<LiveSession | null>(null)

  // 自动讲解开关
  const autoExplainEnabled = ref(false)

  // 计算属性
  const selectedBrowser = computed(() =>
    browsers.value.find((b) => b.id === selectedBrowserId.value)
  )

  const canStartLive = computed(() => isLiveRoomCreated.value && !isLiveStarted.value)

  // 设置相关方法
  function loadSettings(): AppSettings {
    try {
      const saved = localStorage.getItem(STORAGE_KEYS.SETTINGS)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch {
      // 忽略解析错误
    }
    return { ...defaultSettings }
  }

  function saveSettings(newSettings: AppSettings) {
    settings.value = newSettings
    localStorage.setItem(STORAGE_KEYS.SETTINGS, JSON.stringify(newSettings))
  }

  // AI 设置相关方法
  function loadAISettings(): AIScriptSettings {
    try {
      const saved = localStorage.getItem(STORAGE_KEYS.AI_SETTINGS)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch {
      // 忽略解析错误
    }
    return {
      model: DEFAULT_AI_MODEL,
      apiKey: '',
      prompt: AI_PROMPTS.SCRIPT_SYSTEM,
    }
  }

  // 商品文件相关方法
  function loadProductFiles(): ProductFile[] {
    try {
      const saved = localStorage.getItem(STORAGE_KEYS.PRODUCT_FILES)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch {
      // 忽略解析错误
    }
    return []
  }

  function saveProductFiles() {
    localStorage.setItem(STORAGE_KEYS.PRODUCT_FILES, JSON.stringify(productFiles.value))
  }

  function addProductFile(file: ProductFile) {
    productFiles.value.push(file)
    saveProductFiles()
  }

  function removeProductFile(id: string) {
    productFiles.value = productFiles.value.filter((f) => f.id !== id)
    saveProductFiles()
  }

  function updateProductFiles(files: ProductFile[]) {
    productFiles.value = files
    saveProductFiles()
  }

  function updateProductFileUseCount(id: string, useCount: number) {
    const file = productFiles.value.find((f) => f.id === id)
    if (file) {
      file.useCount = useCount
      saveProductFiles()
    }
  }

  // 浏览器相关方法
  function setBrowsers(list: BrowserInfo[]) {
    browsers.value = list
  }

  function selectBrowser(id: string) {
    selectedBrowserId.value = id
  }

  // 商品相关方法
  function setProducts(list: ProductItem[]) {
    products.value = list
  }

  function updateProduct(id: string, updates: Partial<ProductItem>) {
    const index = products.value.findIndex((p) => p.id === id)
    if (index !== -1) {
      products.value[index] = { ...products.value[index], ...updates } as ProductItem
    }
  }

  // 图片配置方法
  function setImageConfig(config: ImageSettings) {
    imageConfig.value = config
  }

  // 直播参数方法
  function setLiveParams(params: LiveParameters) {
    liveParams.value = params
  }

  // 日志方法（超出自动清除旧的）
  function addLog(level: LogEntry['level'], message: string) {
    const entry: LogEntry = {
      id: `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`,
      timestamp: new Date(),
      level,
      message,
    }
    logs.value.push(entry)
    // 超出最大条数时，移除最旧的日志
    if (logs.value.length > LIVE_CONFIG.MAX_LOGS) {
      logs.value = logs.value.slice(-LIVE_CONFIG.MAX_LOGS)
    }
  }

  function clearLogs() {
    logs.value = []
  }

  // 直播间状态方法
  function setLiveRoomCreated(created: boolean, id?: number) {
    isLiveRoomCreated.value = created
    if (id !== undefined) {
      liveId.value = id
    }
  }

  function setLiveStarted(started: boolean) {
    isLiveStarted.value = started
  }

  // 倒计时方法
  function startCountdown(targetTime: Date) {
    countdownTargetTime.value = targetTime
    countdownRunning.value = true
  }

  function stopCountdown() {
    countdownRunning.value = false
  }

  // AI 话术方法
  function setAIScripts(scripts: AIScript[], resetIndex = true) {
    aiScripts.value = scripts
    if (resetIndex) {
      currentScriptIndex.value = 0
    }
  }

  // 更新单条话术（不重置索引）
  function updateAIScript(index: number, script: AIScript) {
    if (index >= 0 && index < aiScripts.value.length) {
      aiScripts.value[index] = script
    }
  }

  function nextScript() {
    if (currentScriptIndex.value < aiScripts.value.length - 1) {
      currentScriptIndex.value++
    }
  }

  function prevScript() {
    if (currentScriptIndex.value > 0) {
      currentScriptIndex.value--
    }
  }

  function saveAIScriptSettings(newSettings: AIScriptSettings) {
    aiScriptSettings.value = newSettings
    localStorage.setItem(STORAGE_KEYS.AI_SETTINGS, JSON.stringify(newSettings))
  }

  // 直播场次相关方法（从文件加载，初始返回空数组，需要调用 initLiveSessions 异步加载）
  function loadLiveSessions(): LiveSession[] {
    return []
  }

  // 异步初始化直播场次数据（从文件加载）
  async function initLiveSessions() {
    try {
      const json = await invoke<string>('load_live_sessions')
      liveSessions.value = JSON.parse(json)
    } catch (e) {
      console.error('加载直播场次数据失败:', e)
      liveSessions.value = []
    }
  }

  // 保存直播场次到文件
  async function saveLiveSessions() {
    try {
      const json = JSON.stringify(liveSessions.value)
      await invoke<string>('save_live_sessions', { sessionsJson: json })
    } catch (e) {
      console.error('保存直播场次数据失败:', e)
    }
  }

  // 创建新的直播场次
  function createLiveSession(
    liveIdNum: number,
    title: string,
    browserName: string,
    accountName: string,
    startTime: Date
  ): LiveSession {
    const session: LiveSession = {
      id: String(liveIdNum),
      liveId: liveIdNum,
      title,
      browserName,
      accountName,
      startTime: startTime.toISOString(),
      createdAt: new Date().toISOString(),
      products: [],
    }
    currentSession.value = session
    return session
  }

  // 添加商品到当前直播场次
  function addProductsToSession(products: LiveProduct[]) {
    if (!currentSession.value) return
    currentSession.value.products.push(...products)
  }

  // 保存当前直播场次
  async function saveCurrentSession() {
    if (!currentSession.value) return

    // 检查是否已存在，存在则更新
    const index = liveSessions.value.findIndex((s) => s.id === currentSession.value!.id)
    if (index !== -1) {
      liveSessions.value[index] = currentSession.value
    } else {
      liveSessions.value.unshift(currentSession.value) // 新的放在前面
    }

    // 只保留最近的场次
    if (liveSessions.value.length > LIVE_CONFIG.MAX_SESSIONS) {
      liveSessions.value = liveSessions.value.slice(0, LIVE_CONFIG.MAX_SESSIONS)
    }

    await saveLiveSessions()
  }

  // 获取当前直播商品列表
  function getCurrentProducts(): LiveProduct[] {
    return currentSession.value?.products ?? []
  }

  // 根据 liveId 加载历史场次
  function loadSessionByLiveId(liveIdNum: number): LiveSession | null {
    const session = liveSessions.value.find((s) => s.liveId === liveIdNum)
    if (session) {
      currentSession.value = session
    }
    return session ?? null
  }

  // 删除历史场次
  async function deleteSession(liveIdNum: number) {
    liveSessions.value = liveSessions.value.filter((s) => s.liveId !== liveIdNum)
    await saveLiveSessions()
  }

  return {
    // 状态
    settings,
    browsers,
    selectedBrowserId,
    selectedBrowser,
    products,
    productFiles,
    imageConfig,
    liveParams,
    logs,
    isLiveRoomCreated,
    isLiveStarted,
    liveId,
    countdownTargetTime,
    countdownRunning,
    aiScripts,
    currentScriptIndex,
    aiScriptSettings,
    canStartLive,

    // 方法
    saveSettings,
    setBrowsers,
    selectBrowser,
    setProducts,
    updateProduct,
    addProductFile,
    removeProductFile,
    updateProductFiles,
    updateProductFileUseCount,
    setImageConfig,
    setLiveParams,
    addLog,
    clearLogs,
    setLiveRoomCreated,
    setLiveStarted,
    startCountdown,
    stopCountdown,
    setAIScripts,
    updateAIScript,
    nextScript,
    prevScript,
    saveAIScriptSettings,

    // 直播场次
    liveSessions,
    currentSession,
    initLiveSessions,
    createLiveSession,
    addProductsToSession,
    saveCurrentSession,
    getCurrentProducts,
    loadSessionByLiveId,
    deleteSession,

    // 自动讲解
    autoExplainEnabled,
  }
})
