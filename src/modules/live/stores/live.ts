import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
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
} from '../types'

const SETTINGS_KEY = 'jd-live-assistant-settings'
const AI_SETTINGS_KEY = 'jd-live-assistant-ai-settings'
const PRODUCT_FILES_KEY = 'jd-live-assistant-product-files'

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
  width: 800,
  height: 800,
}

// 默认直播参数（开始时间为当前时间 + 3 分钟）
const getDefaultLiveParams = (): LiveParameters => ({
  totalProducts: 10,
  cartProducts: 5,
  startTime: new Date(Date.now() + 3 * 60 * 1000),
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

  // AI 话术
  const aiScripts = ref<AIScript[]>([])
  const currentScriptIndex = ref(0)
  const aiScriptSettings = ref<AIScriptSettings>(loadAISettings())

  // 计算属性
  const selectedBrowser = computed(() =>
    browsers.value.find((b) => b.id === selectedBrowserId.value),
  )

  const canStartLive = computed(() => isLiveRoomCreated.value && !isLiveStarted.value)

  // 设置相关方法
  function loadSettings(): AppSettings {
    try {
      const saved = localStorage.getItem(SETTINGS_KEY)
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
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(newSettings))
  }

  // AI 设置相关方法
  function loadAISettings(): AIScriptSettings {
    try {
      const saved = localStorage.getItem(AI_SETTINGS_KEY)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch {
      // 忽略解析错误
    }
    return {
      model: 'Qwen/Qwen2-7B-Instruct',
      apiKey: '',
      prompt: '你是一个专业的直播带货主播，请根据商品信息生成吸引人的直播话术。',
    }
  }

  // 商品文件相关方法
  function loadProductFiles(): ProductFile[] {
    try {
      const saved = localStorage.getItem(PRODUCT_FILES_KEY)
      if (saved) {
        return JSON.parse(saved)
      }
    } catch {
      // 忽略解析错误
    }
    return []
  }

  function saveProductFiles() {
    localStorage.setItem(PRODUCT_FILES_KEY, JSON.stringify(productFiles.value))
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

  // 日志方法（最大 500 条，超出自动清除旧的）
  const MAX_LOGS = 500
  function addLog(level: LogEntry['level'], message: string) {
    const entry: LogEntry = {
      id: `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`,
      timestamp: new Date(),
      level,
      message,
    }
    logs.value.push(entry)
    // 超出最大条数时，移除最旧的日志
    if (logs.value.length > MAX_LOGS) {
      logs.value = logs.value.slice(-MAX_LOGS)
    }
  }

  function clearLogs() {
    logs.value = []
  }


  // 直播间状态方法
  function setLiveRoomCreated(created: boolean) {
    isLiveRoomCreated.value = created
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
  function setAIScripts(scripts: AIScript[]) {
    aiScripts.value = scripts
    currentScriptIndex.value = 0
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
    localStorage.setItem(AI_SETTINGS_KEY, JSON.stringify(newSettings))
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
    nextScript,
    prevScript,
    saveAIScriptSettings,
  }
})
