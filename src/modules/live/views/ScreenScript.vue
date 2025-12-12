<script setup lang="ts">
/**
 * AI话术投屏页面
 * 复用 AIScriptPanel 组件，通过事件与主窗口双向同步
 */
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, emit as tauriEmit, type UnlistenFn } from '@tauri-apps/api/event'
import { Icon } from '@iconify/vue'
import type { AIScript } from '../types'

const WINDOW_LABEL = 'screen-script'

// 话术数据
const scripts = ref<AIScript[]>([])
const currentIndex = ref(0)

// 右键菜单
const showContextMenu = ref(false)
const menuPosition = ref({ x: 0, y: 0 })
let menuTimer: number | null = null

// 字体大小
const fontSize = ref(1)
const minFontSize = 0.75
const maxFontSize = 2

// 事件监听器
let unlistenUpdate: UnlistenFn | null = null

// 当前话术
const currentScript = () => scripts.value[currentIndex.value] || null

// 是否可以切换
const canPrev = () => currentIndex.value > 0
const canNext = () => currentIndex.value < scripts.value.length - 1

onMounted(async () => {
  // 从 URL 参数获取初始数据
  const hash = window.location.hash
  const queryIndex = hash.indexOf('?')
  if (queryIndex !== -1) {
    const params = new URLSearchParams(hash.slice(queryIndex))
    const scriptsStr = params.get('scripts')
    const index = params.get('index')
    if (scriptsStr) {
      try {
        scripts.value = JSON.parse(decodeURIComponent(scriptsStr))
      } catch {
        // 忽略
      }
    }
    if (index) {
      currentIndex.value = parseInt(index, 10)
    }
  }

  // 监听主窗口同步事件
  unlistenUpdate = await listen<{ scripts: AIScript[]; index: number }>(
    'script-sync-to-screen',
    (event) => {
      scripts.value = event.payload.scripts
      currentIndex.value = event.payload.index
    },
  )

  document.addEventListener('click', () => {
    showContextMenu.value = false
  })
})

onUnmounted(() => {
  if (unlistenUpdate) unlistenUpdate()
})

// 切换时通知主窗口
function handlePrev() {
  if (!canPrev()) return
  currentIndex.value--
  syncToMain()
}

function handleNext() {
  if (!canNext()) return
  currentIndex.value++
  syncToMain()
}

// 同步到主窗口
function syncToMain() {
  tauriEmit('script-sync-to-main', { index: currentIndex.value })
}

// 字体调整
function increaseFontSize() {
  if (fontSize.value < maxFontSize) fontSize.value += 0.125
}
function decreaseFontSize() {
  if (fontSize.value > minFontSize) fontSize.value -= 0.125
}

// 右键菜单
function handleContextMenu(e: MouseEvent) {
  e.preventDefault()
  menuPosition.value = { x: e.clientX, y: e.clientY }
  showContextMenu.value = true
  // 2秒后自动关闭
  if (menuTimer) clearTimeout(menuTimer)
  menuTimer = window.setTimeout(() => {
    showContextMenu.value = false
  }, 2000)
}

async function closeWindow() {
  await invoke('close_screen_window', { label: WINDOW_LABEL })
}

async function startDrag(e: MouseEvent) {
  if (e.button !== 0) return
  e.preventDefault()
  await invoke('start_dragging_window', { label: WINDOW_LABEL })
}
</script>

<template>
  <div
    class="w-full h-screen bg-base-200 flex flex-col overflow-hidden select-none"
    @contextmenu="handleContextMenu"
  >
    <!-- 顶部工具栏（可拖动） -->
    <div class="flex items-center justify-between p-3 bg-base-100 cursor-move" @mousedown="startDrag">
      <div class="flex items-center gap-2">
        <Icon icon="mdi:robot" class="text-lg" />
        <span class="font-medium">AI 话术提词</span>
        <span v-if="scripts.length" class="badge badge-ghost badge-sm">
          {{ currentIndex + 1 }}/{{ scripts.length }}
        </span>
      </div>
      <div class="flex items-center gap-1">
        <button class="btn btn-outline btn-xs" :disabled="fontSize <= minFontSize" @click.stop="decreaseFontSize">
          <Icon icon="mdi:format-font-size-decrease" class="text-sm" />
          缩小
        </button>
        <button class="btn btn-outline btn-xs" :disabled="fontSize >= maxFontSize" @click.stop="increaseFontSize">
          <Icon icon="mdi:format-font-size-increase" class="text-sm" />
          放大
        </button>
      </div>
    </div>

    <!-- 话术内容 -->
    <div class="flex-1 flex flex-col p-4 min-h-0">
      <!-- 商品信息 -->
      <div v-if="currentScript()?.productId" class="text-xs text-base-content/60 mb-2">
        <Icon icon="mdi:package-variant" class="inline" />
        商品 ID: {{ currentScript()?.productId }}
      </div>

      <!-- 内容区域 -->
      <div class="flex-1 overflow-y-auto bg-base-100 rounded-lg p-4 mb-3 relative">
        <p
          v-if="currentScript()?.content"
          class="leading-relaxed whitespace-pre-wrap"
          :style="{ fontSize: fontSize + 'rem' }"
        >
          {{ currentScript()?.content }}
        </p>
        <div v-else class="text-center text-base-content/60 py-8">
          <Icon icon="mdi:text-box-remove-outline" class="text-4xl mb-2" />
          <p>暂无话术内容</p>
        </div>
        <span v-if="currentScript()?.content" class="absolute bottom-2 right-2 text-xs text-base-content/40">
          {{ currentScript()?.content?.length || 0 }} 字
        </span>
      </div>

      <!-- 切换按钮 -->
      <div class="flex justify-center items-center gap-3">
        <button class="btn btn-sm btn-outline" :disabled="!canPrev()" @click.stop="handlePrev">
          <Icon icon="mdi:chevron-left" />
          上一条
        </button>
        <button class="btn btn-sm btn-outline" :disabled="!canNext()" @click.stop="handleNext">
          下一条
          <Icon icon="mdi:chevron-right" />
        </button>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div
      v-if="showContextMenu"
      class="fixed bg-base-100 shadow-lg rounded-lg py-1 min-w-32 z-50"
      :style="{ left: menuPosition.x + 'px', top: menuPosition.y + 'px' }"
      @click.stop
      @mousedown.stop
    >
      <button class="w-full px-4 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2" @click.stop="closeWindow">
        <span class="text-error">✕</span>
        关闭窗口
      </button>
    </div>
  </div>
</template>
