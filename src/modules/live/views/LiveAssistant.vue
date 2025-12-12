<script setup lang="ts">
/**
 * 京东直播助手主页面
 * 整合所有组件，实现完整的直播助手界面
 */
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLiveStore } from '../stores/live'
import { useToast } from '@/core/composables/useToast'
import BrowserList from '../components/BrowserList.vue'
import ProductConfig from '../components/ProductConfig.vue'
import ImageConfig from '../components/ImageConfig.vue'
import ExecutionLog from '../components/ExecutionLog.vue'
import LiveParams from '../components/LiveParams.vue'
import CountdownTimer from '../components/CountdownTimer.vue'
import AIScriptPanel from '../components/AIScriptPanel.vue'
import AISettingsModal from '../components/AISettingsModal.vue'
import type {
  BrowserInfo,
  ImageSettings,
  LiveParameters,
  AIScriptSettings,
  Cookie,
  CreateLiveRequest,
  SkuInfo,
  LiveProduct,
  LiveSession,
} from '../types'
import {
  createLiveRoom,
  getSkuInfoByFile,
  addSkuToBagBatch,
  startExplain,
  endExplain,
} from '../api/jd'

const store = useLiveStore()
const toast = useToast()

// 直播间标题配置存储 key
const TITLE_SETTINGS_KEY = 'jd-live-title-settings'

// 手风琴展开状态
const expandedPanel = ref<string | null>(null)

function togglePanel(panel: string) {
  expandedPanel.value = expandedPanel.value === panel ? null : panel
}

// AI 设置弹窗
const showAISettings = ref(false)

function handleAISettingsSave(settings: AIScriptSettings) {
  store.saveAIScriptSettings(settings)
  store.addLog('success', 'AI 设置已保存')
}

// 浏览器加载状态
const browserLoading = ref(false)

// 是否已选择并登录浏览器
const isBrowserLoggedIn = computed(
  () => store.selectedBrowser?.jdAccount?.isLoggedIn === true,
)

// 开始直播按钮状态
const canStartLive = computed(
  () => isBrowserLoggedIn.value && store.isLiveRoomCreated && !store.isLiveStarted,
)

// 是否可以讲解（已登录 + 已创建直播间）
const canExplain = computed(() => isBrowserLoggedIn.value && store.isLiveRoomCreated)

// 讲解状态
const isExplaining = ref(false)
const currentExplainingSku = ref<string | null>(null)

// 投屏状态
const isScreening = ref(false)
const screenImageUrl = ref<string | null>(null)

// 是否可以投屏（有商品数据）
const canScreen = computed(() => store.getCurrentProducts().length > 0)

// 讲解时间和休息时间设置（秒）
const explainDuration = ref(60)

// 选择直播间相关
const showLiveRoomSelect = ref(false)
const restDuration = ref(10)

// 处理浏览器选择
function handleBrowserSelect(browser: BrowserInfo) {
  store.selectBrowser(browser.id)
}

// 刷新浏览器列表（由组件内部处理）
function handleBrowserRefresh() {
  // 组件内部会自动处理刷新逻辑
}

// 更新浏览器列表
function handleBrowsersUpdate(browsers: BrowserInfo[]) {
  store.setBrowsers(browsers)
}



// 处理图片配置更新
function handleImageConfigUpdate(config: ImageSettings) {
  store.setImageConfig(config)
}

// 处理直播参数更新
function handleLiveParamsUpdate(params: LiveParameters) {
  store.setLiveParams(params)
}

// 读取 Cookie（通过 Tauri 后端）
async function readCookiesFromFile(browserId: string): Promise<Cookie[]> {
  try {
    // 重新获取 Cookie
    const cookies = await invoke<Cookie[]>('read_chrome_cookies', {
      domain: 'jd.com',
      profile: browserId,
    })
    return cookies
  } catch {
    return []
  }
}

// 获取直播间标题配置
function getTitleSettings(): { titles: string[] } {
  try {
    const saved = localStorage.getItem(TITLE_SETTINGS_KEY)
    if (saved) {
      return JSON.parse(saved)
    }
  } catch {
    // 忽略
  }
  return { titles: [] }
}

// 封面图片类型
interface CoverImage {
  fourToThree?: string
  twoToOne?: string
  oneToOne?: string
  threeToFour?: string
}

// 格式化日期为 YYYY-MM-DD HH:mm:ss
function formatDateTime(date: Date): string {
  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`
}

// 添加商品到购物袋（按文件顺序处理）
async function addSkusToBag(liveId: number, cookies: Cookie[]): Promise<number> {
  const targetCount = store.liveParams.cartProducts
  store.addLog('info', `【步骤7】开始添加商品到购物袋，目标数量: ${targetCount}`)

  const productFiles = store.productFiles
  if (!productFiles || productFiles.length === 0) {
    store.addLog('warn', '【步骤7】没有商品文件')
    return 0
  }

  let totalSuccess = 0
  const maxBatchSize = 150

  // 按文件顺序处理
  for (let fileIndex = 0; fileIndex < productFiles.length; fileIndex++) {
    if (totalSuccess >= targetCount) break

    const file = productFiles[fileIndex]!
    const useCount = file.useCount === 999 ? file.productIds.length : file.useCount
    const fileSkuIds = file.productIds.slice(0, useCount)

    store.addLog('info', `【文件${fileIndex + 1}】开始处理: ${file.name}，共 ${fileSkuIds.length} 个商品`)

    let fileSuccess = 0
    let fileInvalid = 0
    let skuIndex = 0

    // 处理当前文件的商品
    while (totalSuccess < targetCount && skuIndex < fileSkuIds.length) {
      // 计算本批次获取数量
      const remaining = targetCount - totalSuccess
      const available = fileSkuIds.length - skuIndex
      const fetchCount = Math.min(remaining + 20, available, 100) // 多取一些备用

      if (fetchCount <= 0) break

      const batchIds = fileSkuIds.slice(skuIndex, skuIndex + fetchCount)
      skuIndex += fetchCount

      // 获取商品详情
      let skuInfos: SkuInfo[] = []
      try {
        skuInfos = await getSkuInfoByFile(cookies, liveId, batchIds)
        const invalidCount = batchIds.length - skuInfos.length
        fileInvalid += invalidCount
        if (invalidCount > 0) {
          store.addLog('warn', `【文件${fileIndex + 1}】获取详情: ${skuInfos.length} 有效，${invalidCount} 无效`)
        }
      } catch (error) {
        store.addLog('error', `【文件${fileIndex + 1}】获取商品详情失败: ${error}`)
        continue
      }

      if (skuInfos.length === 0) continue

      // 添加商品到购物袋
      let infoIndex = 0
      while (totalSuccess < targetCount && infoIndex < skuInfos.length) {
        const batchRemaining = targetCount - totalSuccess
        const batchAvailable = skuInfos.length - infoIndex
        const batchSize = Math.min(batchRemaining, maxBatchSize, batchAvailable)

        const batch = skuInfos.slice(infoIndex, infoIndex + batchSize)
        infoIndex += batchSize

        try {
          const result = await addSkuToBagBatch(cookies, liveId, batch)
          if (result.success && result.success_count > 0) {
            fileSuccess += result.success_count
            totalSuccess += result.success_count

            // 保存成功添加的商品到当前直播场次
            const addedProducts: LiveProduct[] = batch.slice(0, result.success_count).map((sku) => ({
              sku: sku.sku,
              title: sku.title ?? '',
              img: sku.img ?? '',
              price: sku.price,
              shopName: sku.shopName,
            }))
            store.addProductsToSession(addedProducts)
          }
        } catch (error) {
          store.addLog('error', `【文件${fileIndex + 1}】添加失败: ${error}`)
        }
      }
    }

    // 输出当前文件的统计
    store.addLog(
      'success',
      `【文件${fileIndex + 1}】${file.name} 完成: 添加 ${fileSuccess} 个，无效 ${fileInvalid} 个，累计 ${totalSuccess}/${targetCount}`,
    )
  }

  if (totalSuccess < targetCount) {
    store.addLog('warn', `【步骤7】所有文件处理完毕，实际添加 ${totalSuccess}/${targetCount} 个`)
  } else {
    store.addLog('success', `【步骤7】✓ 成功添加 ${totalSuccess} 个商品`)
  }
  return totalSuccess
}

// 新建直播间
async function handleCreateLiveRoom() {
  store.addLog('info', '========== 开始创建直播间 ==========')

  // 1. 检查是否已选中浏览器并且登录成功
  store.addLog('info', '【检查1】检查浏览器选择和登录状态...')
  const selectedBrowser = store.selectedBrowser
  if (!selectedBrowser) {
    const msg = '未选择浏览器，请先选择一个浏览器'
    store.addLog('error', `【检查1】❌ ${msg}`)
    toast.error(msg)
    return
  }
  store.addLog('info', `【检查1】已选择浏览器: ${selectedBrowser.name}`)

  if (!selectedBrowser.jdAccount?.isLoggedIn) {
    const msg = '浏览器未登录京东，请先点击浏览器获取登录状态'
    store.addLog('error', `【检查1】❌ ${msg}`)
    toast.error(msg)
    return
  }
  store.addLog('success', `【检查1】✓ 浏览器已登录，账号: ${selectedBrowser.jdAccount.nickname}`)

  // 2. 检查直播间标题配置
  store.addLog('info', '【检查2】检查直播间标题配置...')
  const titleSettings = getTitleSettings()
  if (!titleSettings.titles || titleSettings.titles.length === 0) {
    const msg = '未配置直播间标题，请先设置直播间标题'
    store.addLog('error', `【检查2】❌ ${msg}`)
    toast.error(msg)
    return
  }
  // 随机获取一条标题
  const randomIndex = Math.floor(Math.random() * titleSettings.titles.length)
  const randomTitle = titleSettings.titles[randomIndex] ?? '直播间'
  store.addLog('success', `【检查2】✓ 已配置 ${titleSettings.titles.length} 条标题，随机选中: "${randomTitle}"`)

  // 3. 检查商品文件配置
  store.addLog('info', '【检查3】检查商品文件配置...')
  const productFiles = store.productFiles
  if (!productFiles || productFiles.length === 0) {
    const msg = '未设置商品文件，请先导入商品文件'
    store.addLog('error', `【检查3】❌ ${msg}`)
    toast.error(msg)
    return
  }
  // 检查是否有有效数据
  const totalProducts = productFiles.reduce((sum, f) => sum + f.uniqueCount, 0)
  if (totalProducts === 0) {
    const msg = '商品文件中没有有效数据'
    store.addLog('error', `【检查3】❌ ${msg}`)
    toast.error(msg)
    return
  }
  store.addLog(
    'success',
    `【检查3】✓ 已配置 ${productFiles.length} 个商品文件，共 ${totalProducts} 条有效商品`,
  )

  // 3.5 检查直播时间（必须在未来 3 分钟到 30 天内）
  store.addLog('info', '【检查3.5】检查直播时间...')
  const now = Date.now()
  const minTime = now + 3 * 60 * 1000 // 3 分钟后
  const maxTime = now + 30 * 24 * 60 * 60 * 1000 // 30 天后
  const startTime = store.liveParams.startTime.getTime()

  if (startTime < minTime) {
    // 自动调整为 3 分钟后
    const newStartTime = new Date(minTime)
    store.setLiveParams({ ...store.liveParams, startTime: newStartTime })
    store.addLog('warn', '【检查3.5】⚠ 直播时间已过期，已自动调整为 3 分钟后')
  } else if (startTime > maxTime) {
    const msg = '直播时间不能超过 30 天后'
    store.addLog('error', `【检查3.5】❌ ${msg}`)
    toast.error(msg)
    return
  } else {
    store.addLog('success', '【检查3.5】✓ 直播时间有效')
  }

  // 4. 获取 Cookie
  store.addLog('info', '【检查4】正在获取 Cookie...')
  let cookies: Cookie[] = []
  try {
    cookies = await readCookiesFromFile(selectedBrowser.id)
    if (cookies.length === 0) {
      const msg = '无法获取 Cookie，请重新选择浏览器'
      store.addLog('error', `【检查4】❌ ${msg}`)
      toast.error(msg)
      return
    }
    store.addLog('success', `【检查4】✓ 获取到 ${cookies.length} 个 Cookie`)
  } catch (error) {
    const errorMsg = String(error)
    store.addLog('error', `【检查4】❌ 获取 Cookie 失败: ${errorMsg}`)
    // 检查是否是浏览器已打开的错误
    if (errorMsg.includes('浏览器启动失败') || errorMsg.includes('BrowserLaunchFailed')) {
      toast.error('请先关闭 Chrome 浏览器，然后重新执行')
    } else {
      toast.error(`获取 Cookie 失败: ${errorMsg}`)
    }
    return
  }

  // 5. 获取封面图片
  store.addLog('info', '【检查5】正在获取封面图片...')
  let coverImages: CoverImage[] = []
  try {
    coverImages = await invoke<CoverImage[]>('get_cover_images', { cookies })
    if (!coverImages || coverImages.length === 0) {
      const msg = '没有可用的封面图片，请先上传封面图片'
      store.addLog('error', `【检查5】❌ ${msg}`)
      toast.error(msg)
      return
    }
    store.addLog('success', `【检查5】✓ 获取到 ${coverImages.length} 张封面图片`)
  } catch (error) {
    const msg = `获取封面图片失败: ${error}`
    store.addLog('error', `【检查5】❌ ${msg}`)
    toast.error(msg)
    return
  }

  // 6. 创建直播间
  store.addLog('info', '【步骤6】正在创建直播间...')
  try {
    // 随机选择一张封面图片
    const coverIndex = Math.floor(Math.random() * coverImages.length)
    const cover = coverImages[coverIndex] as CoverImage

    // 构建请求参数
    const publishTime = formatDateTime(store.liveParams.startTime)
    const request: CreateLiveRequest = {
      title: randomTitle,
      indexImage: cover?.fourToThree ?? '',
      resizeIndexImage: cover?.twoToOne ?? '',
      squareIndexImage: cover?.oneToOne ?? '',
      portraitIndexImage: cover?.threeToFour ?? '',
      type: 69,
      publishTime,
      screen: 0,
      test: 0,
      locationDetail: null,
      canExplain: 1,
      preVideoType: 0,
      desc: '',
      welcome: '欢迎来到我的直播间，喜欢我可以点一下关注哦~',
      channelNum: '',
      pcVersion: 1,
    }

    store.addLog('info', `【步骤6】直播间标题: ${randomTitle}`)
    store.addLog('info', `【步骤6】发布时间: ${publishTime}`)

    const liveId = await createLiveRoom(cookies, request)
    store.setLiveRoomCreated(true, liveId)
    store.addLog('success', `【步骤6】✓ 直播间创建成功，ID: ${liveId}`)

    // 创建直播场次，用于保存商品数据
    store.createLiveSession(
      liveId,
      randomTitle,
      selectedBrowser.name,
      selectedBrowser.jdAccount?.nickname ?? '',
      store.liveParams.startTime,
    )

    // 7. 添加商品到购物袋
    const successCount = await addSkusToBag(liveId, cookies)
    if (successCount > 0) {
      store.addLog('success', `【步骤7】✓ 成功添加 ${successCount} 个商品到购物袋`)
    } else {
      store.addLog('warn', '【步骤7】⚠ 未能添加商品到购物袋')
    }

    // 保存直播场次数据（包含商品列表）
    store.saveCurrentSession()
    store.addLog('info', `【步骤8】✓ 已保存 ${store.getCurrentProducts().length} 个商品数据`)

    toast.success(`直播间创建成功，已添加 ${successCount} 个商品`)
  } catch (error) {
    const msg = `创建直播间失败: ${error}`
    store.addLog('error', `【步骤6】❌ ${msg}`)
    toast.error(msg)
    return
  }

  store.addLog('success', '========== 直播间创建完成 ==========')
}

// 生成 AI 话术（基于商品数据）
function generateAIScripts() {
  const products = store.getCurrentProducts()
  // 取前 10 条商品生成话术
  const topProducts = products.slice(0, 10)

  if (topProducts.length === 0) {
    store.setAIScripts([
      { id: '1', content: '欢迎来到直播间！今天给大家带来超值好物推荐~' },
    ])
    return
  }

  const scripts = topProducts.map((product, index) => ({
    id: String(index + 1),
    productId: product.sku,
    content: generateProductScript(product, index),
  }))

  store.setAIScripts(scripts)
  store.addLog('success', `已生成 ${scripts.length} 条商品话术`)
}

// 生成单个商品话术
function generateProductScript(product: LiveProduct, index: number): string {
  const templates = [
    `【第${index + 1}款】${product.title}\n\n这款商品非常推荐给大家！品质有保障，价格也很实惠。喜欢的朋友们可以点击下方链接下单哦~`,
    `【第${index + 1}款】${product.title}\n\n家人们看过来！这款商品是我们精心挑选的，性价比超高！数量有限，先到先得~`,
    `【第${index + 1}款】${product.title}\n\n宝子们，这款商品真的太棒了！我自己也在用，强烈推荐给大家！赶紧下单吧~`,
  ]
  return templates[index % templates.length] ?? templates[0]!
}

// 开始直播
async function handleStartLive() {
  if (!canStartLive.value) return

  store.addLog('info', '直播已开始，请点击"开始讲解"按钮开始倒计时')
  store.setLiveStarted(true)

  // 基于商品数据生成话术
  generateAIScripts()
}

// 倒计时结束
function handleCountdownComplete() {
  store.addLog('success', '倒计时结束，直播正式开始！')
  store.stopCountdown()
}

// 获取当前 Cookie
async function getCurrentCookies(): Promise<Cookie[]> {
  const selectedBrowser = store.selectedBrowser
  if (!selectedBrowser) return []
  return readCookiesFromFile(selectedBrowser.id)
}

// 开始讲解商品
async function handleStartExplain(productId: string) {
  if (!store.liveId) {
    toast.error('直播间未创建')
    return
  }

  try {
    const cookies = await getCurrentCookies()
    if (cookies.length === 0) {
      toast.error('无法获取 Cookie')
      return
    }

    await startExplain(cookies, String(store.liveId), productId)
    isExplaining.value = true
    currentExplainingSku.value = productId
    store.addLog('success', `开始讲解商品: ${productId}`)

    // 开始讲解时启动倒计时
    if (!store.countdownRunning) {
      const targetTime = new Date(Date.now() + explainDuration.value * 1000)
      store.startCountdown(targetTime)
    }

    // 更新投屏图片为当前商品
    const products = store.getCurrentProducts()
    const product = products.find((p) => p.sku === productId)
    if (product?.img) {
      screenImageUrl.value = product.img
    }
  } catch (error) {
    store.addLog('error', `开始讲解失败: ${error}`)
    toast.error(`开始讲解失败: ${error}`)
  }
}

// 结束讲解商品
async function handleEndExplain(productId: string) {
  if (!store.liveId) return

  try {
    const cookies = await getCurrentCookies()
    if (cookies.length === 0) return

    await endExplain(cookies, String(store.liveId), productId)
    isExplaining.value = false
    currentExplainingSku.value = null
    store.addLog('info', `结束讲解商品: ${productId}`)
  } catch (error) {
    store.addLog('error', `结束讲解失败: ${error}`)
  }
}

// 开启投屏（调用后端创建独立窗口）
async function handleStartScreen() {
  const products = store.getCurrentProducts()
  if (products.length === 0) {
    toast.error('没有商品数据')
    return
  }

  const firstProduct = products[0]
  if (!firstProduct?.img) {
    toast.error('第一个商品没有图片')
    return
  }

  try {
    // 调用后端创建投屏窗口（无边框）
    await invoke('create_screen_window', {
      label: 'screen-window',
      title: '商品投屏',
      width: store.imageConfig.width,
      height: store.imageConfig.height,
      transparent: false,
      alwaysOnTop: true,
      decorations: false, // 无边框，去掉顶部操作栏
      resizable: true,
      backgroundColor: '#000000',
      extraParams: `imageUrl=${encodeURIComponent(firstProduct.img)}`,
    })

    screenImageUrl.value = firstProduct.img
    isScreening.value = true
    store.addLog('success', `投屏已开启，显示商品: ${firstProduct.title}`)
  } catch (error) {
    store.addLog('error', `开启投屏失败: ${error}`)
    toast.error(`开启投屏失败: ${error}`)
  }
}

// 停止投屏
async function handleStopScreen() {
  try {
    await invoke('close_screen_window', { label: 'screen-window' })
  } catch {
    // 忽略关闭错误
  }
  isScreening.value = false
  screenImageUrl.value = null
  store.addLog('info', '投屏已关闭')
}

// 获取本地保存的直播间列表
function fetchRecentLiveRooms() {
  const sessions = store.liveSessions
  if (sessions.length === 0) {
    toast.info('暂无本地保存的直播间，请先新建直播间')
    return
  }
  showLiveRoomSelect.value = true
  store.addLog('info', `获取到 ${sessions.length} 个本地直播间`)
}

// 选择本地保存的直播间
function handleSelectLiveRoom(session: LiveSession) {
  store.setLiveRoomCreated(true, session.liveId)
  store.loadSessionByLiveId(session.liveId)
  showLiveRoomSelect.value = false
  store.addLog('success', `已加载直播间: ${session.title}，商品数量: ${session.products.length}`)
  toast.success(`已选择直播间: ${session.title}`)
}

// 格式化日期显示
function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours()}:${String(date.getMinutes()).padStart(2, '0')}`
}

// 窗口关闭事件监听器
let unlistenScreenWindow: UnlistenFn | null = null

// 组件挂载时初始化
onMounted(async () => {
  await store.initLiveSessions()

  // 监听图片投屏窗口关闭事件
  unlistenScreenWindow = await listen('screen-window-closed', () => {
    isScreening.value = false
    screenImageUrl.value = null
    store.addLog('info', '投屏窗口已关闭')
  })
})

// 组件卸载时清理监听器
onUnmounted(() => {
  if (unlistenScreenWindow) {
    unlistenScreenWindow()
  }
})
</script>

<template>
  <div class="flex flex-col h-full p-4 gap-3">
    <!-- 主体区域：左右分栏 3:2 -->
    <div class="flex-1 flex gap-3 min-h-0">
      <!-- 左侧：倒计时 + AI 话术（占 3/5） -->
      <div class="w-3/5 flex flex-col gap-3">
        <CountdownTimer
          :target-time="store.countdownTargetTime"
          :is-running="store.countdownRunning"
          :explain-duration="explainDuration"
          :rest-duration="restDuration"
          @complete="handleCountdownComplete"
          @update:explain-duration="explainDuration = $event"
          @update:rest-duration="restDuration = $event"
        />
        <div class="flex-1 min-h-0">
          <AIScriptPanel
            :scripts="store.aiScripts"
            :current-index="store.currentScriptIndex"
            :total-products="store.getCurrentProducts().length"
            :is-explaining="isExplaining"
            :can-explain="canExplain"
            @prev="store.prevScript"
            @next="store.nextScript"
            @open-settings="showAISettings = true"
            @start-explain="handleStartExplain"
            @end-explain="handleEndExplain"
          />
        </div>
      </div>

      <!-- 右侧：设置项（占 2/5）- 手风琴模式 -->
      <div class="w-2/5 flex flex-col gap-2 settings-accordion">
        <BrowserList
          :browsers="store.browsers"
          :selected-id="store.selectedBrowserId"
          :loading="browserLoading"
          :expanded="expandedPanel === 'browser'"
          @select="handleBrowserSelect"
          @refresh="handleBrowserRefresh"
          @update:browsers="handleBrowsersUpdate"
          @toggle="togglePanel('browser')"
        />
        <ProductConfig :expanded="expandedPanel === 'product'" @toggle="togglePanel('product')" />
        <ImageConfig
          :config="store.imageConfig"
          :expanded="expandedPanel === 'image'"
          :can-screen="canScreen"
          :is-screening="isScreening"
          @update="handleImageConfigUpdate"
          @toggle="togglePanel('image')"
          @start-screen="handleStartScreen"
          @stop-screen="handleStopScreen"
        />
        <ExecutionLog
          :logs="store.logs"
          max-height="300px"
          :expanded="expandedPanel === 'log'"
          @toggle="togglePanel('log')"
          @clear="store.clearLogs"
        />
      </div>
    </div>

    <!-- 底部：直播参数 + 操作按钮 -->
    <div class="flex items-center justify-between gap-4 pt-2 border-t border-base-300">
      <LiveParams :params="store.liveParams" @update="handleLiveParamsUpdate" />
      <div class="flex gap-2">
        <button
          class="btn btn-outline btn-sm"
          :disabled="!isBrowserLoggedIn"
          @click="fetchRecentLiveRooms"
        >
          <Icon icon="mdi:format-list-bulleted" />
          选择直播间
        </button>
        <button
          class="btn btn-secondary btn-sm"
          :disabled="!isBrowserLoggedIn"
          @click="handleCreateLiveRoom"
        >
          <Icon icon="mdi:plus-circle" />
          新建直播间
        </button>
        <button class="btn btn-primary btn-sm" :disabled="!canStartLive" @click="handleStartLive">
          <Icon icon="mdi:play" />
          开始直播
        </button>
      </div>
    </div>

    <!-- AI 设置弹窗 -->
    <AISettingsModal
      :visible="showAISettings"
      :settings="store.aiScriptSettings"
      @update:visible="showAISettings = $event"
      @save="handleAISettingsSave"
    />

    <!-- 选择直播间弹窗 -->
    <dialog :class="['modal', { 'modal-open': showLiveRoomSelect }]">
      <div class="modal-box max-w-lg">
        <h3 class="font-bold text-lg mb-4">
          <Icon icon="mdi:format-list-bulleted" class="inline mr-2" />
          选择直播间（本地记录）
        </h3>

        <div v-if="store.liveSessions.length === 0" class="text-center py-8 text-base-content/60">
          <Icon icon="mdi:inbox-outline" class="text-4xl mb-2" />
          <p>暂无本地保存的直播间</p>
          <p class="text-xs mt-1">请先新建直播间</p>
        </div>

        <div v-else class="space-y-2 max-h-80 overflow-y-auto">
          <div
            v-for="session in store.liveSessions"
            :key="session.id"
            class="flex items-center gap-3 p-3 rounded-lg border border-base-300 hover:bg-base-200 cursor-pointer transition-colors"
            @click="handleSelectLiveRoom(session)"
          >
            <!-- 商品数量图标 -->
            <div class="w-12 h-12 bg-primary/10 rounded flex flex-col items-center justify-center">
              <Icon icon="mdi:package-variant" class="text-primary text-lg" />
              <span class="text-xs text-primary font-medium">{{ session.products.length }}</span>
            </div>

            <!-- 信息 -->
            <div class="flex-1 min-w-0">
              <p class="font-medium truncate">{{ session.title || '未命名直播间' }}</p>
              <p class="text-xs text-base-content/60">
                ID: {{ session.liveId }} · {{ session.accountName }}
              </p>
              <p class="text-xs text-base-content/40">
                {{ formatDate(session.createdAt) }}
              </p>
            </div>

            <Icon icon="mdi:chevron-right" class="text-base-content/40" />
          </div>
        </div>

        <div class="modal-action">
          <button class="btn btn-ghost" @click="showLiveRoomSelect = false">
            取消
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showLiveRoomSelect = false">关闭</button>
      </form>
    </dialog>
  </div>
</template>

<style scoped>
/* 手风琴模式：展开的折叠项自动填充剩余空间 */
.settings-accordion {
  overflow-y: auto;
}

.settings-accordion :deep(.collapse) {
  flex-shrink: 0;
}

.settings-accordion :deep(.collapse.collapse-open) {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.settings-accordion :deep(.collapse.collapse-open > div:last-child) {
  flex: 1;
  overflow-y: auto;
}
</style>
