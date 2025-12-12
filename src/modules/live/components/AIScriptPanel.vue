<script setup lang="ts">
/**
 * AI 话术面板组件
 * 显示 AI 生成的直播话术内容，支持商品讲解控制
 */
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import { emit as tauriEmit, listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { AIScript } from '../types'

interface Props {
  scripts: AIScript[]
  currentIndex: number
  totalProducts?: number // 直播间商品总数
  isExplaining?: boolean // 是否正在讲解
  canExplain?: boolean // 是否可以讲解（直播间已创建）
  isCountdownRunning?: boolean // 倒计时是否正在运行
  autoExplainEnabled?: boolean // 是否开启自动讲解
  canSwitchNext?: boolean // 是否可以切换下一条（讲解满 63 秒）
}

interface Emits {
  (e: 'prev'): void
  (e: 'next'): void
  (e: 'openSettings'): void
  (e: 'startExplain', productId: string): void
  (e: 'endExplain', productId: string): void
}

// 话术投屏状态
const isScriptScreening = ref(false)

// 窗口关闭事件监听器
let unlistenClose: UnlistenFn | null = null

const props = withDefaults(defineProps<Props>(), {
  totalProducts: 0,
  isExplaining: false,
  canExplain: false,
  isCountdownRunning: false,
  autoExplainEnabled: false,
  canSwitchNext: true,
})
const emit = defineEmits<Emits>()

// 字体大小（rem）
const fontSize = ref(1)
const minFontSize = 0.75
const maxFontSize = 2

function increaseFontSize() {
  if (fontSize.value < maxFontSize) {
    fontSize.value = Math.min(maxFontSize, fontSize.value + 0.125)
  }
}

function decreaseFontSize() {
  if (fontSize.value > minFontSize) {
    fontSize.value = Math.max(minFontSize, fontSize.value - 0.125)
  }
}

// 当前话术
const currentScript = computed(() => props.scripts[props.currentIndex] || null)

// 当前话术字数
const wordCount = computed(() => currentScript.value?.content?.length || 0)

// 是否可以切换上一条（不受讲解时间限制）
const canPrev = computed(() => props.currentIndex > 0)
// 是否可以切换下一条（开启自动讲解时，需要满足 63 秒限制）
const canNext = computed(() => {
  if (props.currentIndex >= props.scripts.length - 1) return false
  // 如果开启了自动讲解且正在讲解，需要满足时间限制
  if (props.autoExplainEnabled && props.isExplaining && !props.canSwitchNext) return false
  return true
})

// 进度显示：当前索引/话术总数
const progress = computed(() => {
  if (props.scripts.length === 0) return '0/0'
  return `${props.currentIndex + 1}/${props.scripts.length}`
})

// 话术/商品数量显示
const scriptProductCount = computed(() => {
  if (props.totalProducts === 0) return ''
  return `${props.scripts.length}/${props.totalProducts}`
})

// 处理上一条（只切换，不执行讲解操作）
function handlePrev() {
  emit('prev')
}

// 处理下一条（如果开启自动讲解，结束当前讲解并通知父组件）
function handleNext() {
  const currentProductId = currentScript.value?.productId

  // 如果开启了自动讲解且正在讲解，先结束当前讲解
  if (props.autoExplainEnabled && currentProductId && props.isExplaining) {
    emit('endExplain', currentProductId)
  }

  emit('next')
}

// 开始讲解当前商品
function handleStartFirstExplain() {
  const productId = currentScript.value?.productId
  if (productId) {
    emit('startExplain', productId)
  }
}

// 停止讲解当前商品
function handleStopExplain() {
  const productId = currentScript.value?.productId
  if (productId) {
    emit('endExplain', productId)
  }
}

// 开启话术投屏
async function startScriptScreen() {
  try {
    await invoke('create_screen_window', {
      label: 'screen-script',
      title: '话术投屏',
      width: 500,
      height: 400,
      transparent: false,
      alwaysOnTop: true,
      decorations: false,
      resizable: true,
      backgroundColor: '#000000',
      extraParams: '',
    })
    isScriptScreening.value = true
    // 窗口创建后延迟发送数据（等待窗口加载完成）
    setTimeout(() => {
      tauriEmit('script-sync-to-screen', {
        scripts: props.scripts,
        index: props.currentIndex,
      })
    }, 300)
  } catch (error) {
    console.error('开启话术投屏失败:', error)
  }
}

// 关闭话术投屏
async function stopScriptScreen() {
  try {
    await invoke('close_screen_window', { label: 'screen-script' })
  } catch {
    // 忽略
  }
  isScriptScreening.value = false
}

// 监听话术变化，同步到投屏窗口
watch(
  [() => props.currentIndex, () => props.scripts],
  () => {
    if (isScriptScreening.value) {
      tauriEmit('script-sync-to-screen', {
        scripts: props.scripts,
        index: props.currentIndex,
      })
    }
  },
  { deep: true },
)

// 投屏窗口操作同步监听器
let unlistenSync: UnlistenFn | null = null
let unlistenReady: UnlistenFn | null = null

onMounted(async () => {
  // 监听话术投屏窗口关闭事件
  unlistenClose = await listen('screen-script-closed', () => {
    isScriptScreening.value = false
  })

  // 监听投屏窗口的操作同步
  unlistenSync = await listen<{ index: number }>('script-sync-to-main', (event) => {
    const newIndex = event.payload.index
    if (newIndex > props.currentIndex) {
      emit('next')
    } else if (newIndex < props.currentIndex) {
      emit('prev')
    }
  })

  // 监听投屏窗口准备就绪事件，立即发送数据
  unlistenReady = await listen('script-screen-ready', () => {
    if (isScriptScreening.value) {
      tauriEmit('script-sync-to-screen', {
        scripts: props.scripts,
        index: props.currentIndex,
      })
    }
  })
})

onUnmounted(() => {
  if (unlistenClose) unlistenClose()
  if (unlistenSync) unlistenSync()
  if (unlistenReady) unlistenReady()
})
</script>

<template>
  <div class="card bg-base-100 shadow-sm h-full">
    <div class="card-body p-4 flex flex-col">
      <!-- 标题栏 -->
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2">
          <h3 class="card-title text-sm">
            <Icon icon="mdi:robot" class="text-lg" />
            AI 话术提词
          </h3>
          <!-- 话术/商品数量 -->
          <span v-if="scriptProductCount" class="badge badge-ghost badge-sm">
            {{ scriptProductCount }}
          </span>
          <!-- 话术投屏按钮 -->
          <button
            class="btn btn-xs"
            :class="isScriptScreening ? 'btn-error' : 'btn-success'"
            @click="isScriptScreening ? stopScriptScreen() : startScriptScreen()"
          >
            <Icon :icon="isScriptScreening ? 'mdi:cast-off' : 'mdi:cast'" class="text-sm" />
            {{ isScriptScreening ? '停止' : '投屏' }}
          </button>
        </div>
        <div class="flex items-center gap-1">
          <button
            class="btn btn-outline btn-xs"
            :disabled="fontSize <= minFontSize"
            @click="decreaseFontSize"
          >
            <Icon icon="mdi:format-font-size-decrease" class="text-sm" />
            缩小
          </button>
          <button
            class="btn btn-outline btn-xs"
            :disabled="fontSize >= maxFontSize"
            @click="increaseFontSize"
          >
            <Icon icon="mdi:format-font-size-increase" class="text-sm" />
            放大
          </button>
          <button class="btn btn-outline btn-xs" @click="emit('openSettings')">
            <Icon icon="mdi:cog" class="text-sm" />
            AI 设置
          </button>
          <span class="text-xs text-base-content/60 ml-1">{{ progress }}</span>
        </div>
      </div>

      <!-- 空状态 -->
      <div
        v-if="scripts.length === 0"
        class="flex-1 flex flex-col items-center justify-center text-base-content/60 text-center"
      >
        <Icon icon="mdi:text-box-remove-outline" class="text-4xl mb-2" />
        <p class="text-sm">暂无话术</p>
        <p class="text-xs">请先选择或新建直播间</p>
      </div>

      <!-- 话术内容 -->
      <div v-else class="flex-1 flex flex-col">
        <!-- 商品信息提示 -->
        <div v-if="currentScript?.productId" class="text-sm font-bold text-base-content mb-1">
          <Icon icon="mdi:package-variant" class="inline" />
          商品 ID: {{ currentScript.productId }}
          <span v-if="isExplaining" class="badge badge-success badge-xs ml-1">讲解中</span>
        </div>

        <div class="flex-1 overflow-y-auto bg-base-200/50 rounded-lg p-3 mb-3 relative">
          <p class="leading-relaxed whitespace-pre-wrap" :style="{ fontSize: fontSize + 'rem' }">
            {{ currentScript?.content || '' }}
          </p>
          <!-- 字数统计 -->
          <span class="absolute bottom-2 right-2 text-xs text-base-content/40">{{ wordCount }} 字</span>
        </div>

        <!-- 切换按钮 - 始终显示上下一条 -->
        <div class="flex justify-center items-center gap-2">
          <button class="btn btn-sm btn-outline" :disabled="!canPrev" @click="handlePrev">
            <Icon icon="mdi:chevron-left" />
            上一条
          </button>

          <!-- 只有开启自动讲解时才显示讲解按钮 -->
          <template v-if="autoExplainEnabled">
            <!-- 未开始讲解：显示开始讲解按钮 -->
            <button
              v-if="!isExplaining"
              class="btn btn-sm btn-success"
              :disabled="!canExplain || !currentScript?.productId"
              @click="handleStartFirstExplain"
            >
              <Icon icon="mdi:microphone" />
              开始讲解
            </button>

            <!-- 讲解中：显示停止讲解按钮 -->
            <button
              v-else
              class="btn btn-sm btn-error"
              @click="handleStopExplain"
            >
              <Icon icon="mdi:microphone-off" />
              停止讲解
            </button>
          </template>

          <button class="btn btn-sm btn-outline" :disabled="!canNext" @click="handleNext">
            下一条
            <Icon icon="mdi:chevron-right" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
