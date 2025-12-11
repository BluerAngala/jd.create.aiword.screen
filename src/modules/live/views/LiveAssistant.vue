<script setup lang="ts">
/**
 * 京东直播助手主页面
 * 整合所有组件，实现完整的直播助手界面
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import { useLiveStore } from '../stores/live'
import BrowserList from '../components/BrowserList.vue'
import ProductConfig from '../components/ProductConfig.vue'
import ImageConfig from '../components/ImageConfig.vue'
import ExecutionLog from '../components/ExecutionLog.vue'
import LiveParams from '../components/LiveParams.vue'
import CountdownTimer from '../components/CountdownTimer.vue'
import AIScriptPanel from '../components/AIScriptPanel.vue'
import AISettingsModal from '../components/AISettingsModal.vue'
import type { BrowserInfo, ProductItem, ImageSettings, LiveParameters, AIScriptSettings } from '../types'

const store = useLiveStore()

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

// 处理商品更新
function handleProductUpdate(products: ProductItem[]) {
  store.setProducts(products)
}

// 处理商品导入（由组件内部处理 xlsx 解析）
// eslint-disable-next-line @typescript-eslint/no-unused-vars
function handleProductImport(_file: File) {
  // 组件内部会自动处理 xlsx 解析逻辑
}

// AI 生成标题
function handleGenerateTitles(productId: string) {
  store.addLog('info', `正在为商品 ${productId} 生成 AI 标题...`)
  setTimeout(() => {
    const product = store.products.find((p) => p.id === productId)
    if (product) {
      store.updateProduct(productId, {
        titles: [...product.titles, `AI 生成标题 ${Date.now()}`],
      })
      store.addLog('success', 'AI 标题生成完成')
    }
  }, 1000)
}

// 处理图片配置更新
function handleImageConfigUpdate(config: ImageSettings) {
  store.setImageConfig(config)
}

// 处理直播参数更新
function handleLiveParamsUpdate(params: LiveParameters) {
  store.setLiveParams(params)
}

// 新建直播间
async function handleCreateLiveRoom() {
  store.addLog('info', '正在创建直播间...')
  setTimeout(() => {
    store.setLiveRoomCreated(true)
    store.addLog('success', '直播间创建成功')
  }, 1500)
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
        <ProductConfig
          :products="store.products"
          :expanded="expandedPanel === 'product'"
          @update="handleProductUpdate"
          @import="handleProductImport"
          @generate-titles="handleGenerateTitles"
          @toggle="togglePanel('product')"
        />
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
