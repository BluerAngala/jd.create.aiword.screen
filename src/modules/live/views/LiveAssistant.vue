<script setup lang="ts">
/**
 * 京东直播助手主页面
 * 整合所有组件，实现完整的直播助手界面
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
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
} from '../types'
import { createLiveRoom, getSkuInfoByFile, addSkuToBagBatch } from '../api/jd'

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

// 开始直播按钮状态
const canStartLive = computed(() => store.isLiveRoomCreated && !store.isLiveStarted)

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
          if (result.success) {
            fileSuccess += result.success_count
            totalSuccess += result.success_count
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

    // 7. 添加商品到购物袋
    const successCount = await addSkusToBag(liveId, cookies)
    if (successCount > 0) {
      store.addLog('success', `【步骤7】✓ 成功添加 ${successCount} 个商品到购物袋`)
    } else {
      store.addLog('warn', '【步骤7】⚠ 未能添加商品到购物袋')
    }

    toast.success(`直播间创建成功，已添加 ${successCount} 个商品`)
  } catch (error) {
    const msg = `创建直播间失败: ${error}`
    store.addLog('error', `【步骤6】❌ ${msg}`)
    toast.error(msg)
    return
  }

  store.addLog('success', '========== 直播间创建完成 ==========')
}

// 开始直播
async function handleStartLive() {
  if (!canStartLive.value) return

  store.addLog('info', '直播将在 1 分钟后开始...')
  store.setLiveStarted(true)

  const targetTime = new Date(Date.now() + 60 * 1000)
  store.startCountdown(targetTime)

  store.setAIScripts([
    { id: '1', content: '欢迎来到直播间！今天给大家带来超值好物推荐~' },
    { id: '2', content: '这款商品是我们精心挑选的，品质有保障！' },
    { id: '3', content: '喜欢的朋友们赶紧下单，数量有限哦！' },
  ])
}

// 倒计时结束
function handleCountdownComplete() {
  store.addLog('success', '倒计时结束，直播正式开始！')
  store.stopCountdown()
}
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
          @complete="handleCountdownComplete"
        />
        <div class="flex-1 min-h-0">
          <AIScriptPanel
            :scripts="store.aiScripts"
            :current-index="store.currentScriptIndex"
            @prev="store.prevScript"
            @next="store.nextScript"
            @open-settings="showAISettings = true"
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
          @update="handleImageConfigUpdate"
          @toggle="togglePanel('image')"
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
          class="btn btn-secondary btn-sm"
          :disabled="store.isLiveRoomCreated"
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
