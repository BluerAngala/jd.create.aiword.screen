<script setup lang="ts">
/**
 * 投屏内容页面
 * 独立窗口显示商品图片，供 OBS 捕获
 * 无边框窗口，右键菜单关闭，支持拖动
 */
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 窗口标签（固定值）
const WINDOW_LABEL = 'screen-window'

// 图片 URL（从 URL 参数获取）
const imageUrl = ref<string>('')

// 加载状态
const loading = ref(true)
const error = ref(false)

// 右键菜单状态
const showContextMenu = ref(false)
const menuPosition = ref({ x: 0, y: 0 })
let menuTimer: number | null = null

onMounted(() => {
  // 从 hash URL 参数获取图片地址（格式：/#/screen-content?imageUrl=xxx）
  const hash = window.location.hash
  const queryIndex = hash.indexOf('?')
  if (queryIndex !== -1) {
    const params = new URLSearchParams(hash.slice(queryIndex))
    const url = params.get('imageUrl')
    if (url) {
      imageUrl.value = decodeURIComponent(url)
    }
  }

  // 点击其他地方关闭菜单
  document.addEventListener('click', () => {
    showContextMenu.value = false
  })
})

function handleImageLoad() {
  loading.value = false
}

function handleImageError() {
  loading.value = false
  error.value = true
}

// 显示右键菜单
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

// 关闭窗口（调用后端）
async function closeWindow() {
  await invoke('close_screen_window', { label: WINDOW_LABEL })
}

// 开始拖动窗口（调用后端）
async function startDrag(e: MouseEvent) {
  if (e.button !== 0) return // 只响应左键
  e.preventDefault()
  await invoke('start_dragging_window', { label: WINDOW_LABEL })
}
</script>

<template>
  <div
    class="w-full h-screen bg-black flex items-center justify-center overflow-hidden select-none cursor-move"
    @mousedown="startDrag"
    @contextmenu="handleContextMenu"
  >
    <!-- 加载中 -->
    <div v-if="loading && !error" class="text-white text-center">
      <span class="loading loading-spinner loading-lg"></span>
      <p class="mt-2">加载中...</p>
    </div>

    <!-- 加载失败 -->
    <div v-if="error" class="text-white text-center">
      <p class="text-xl">图片加载失败</p>
      <p class="text-sm opacity-60 mt-2">{{ imageUrl }}</p>
    </div>

    <!-- 商品图片 -->
    <img
      v-if="imageUrl"
      :src="imageUrl"
      alt="商品图片"
      class="max-w-full max-h-full object-contain pointer-events-none"
      :class="{ hidden: loading || error }"
      draggable="false"
      @load="handleImageLoad"
      @error="handleImageError"
    />

    <!-- 无图片 -->
    <div v-if="!imageUrl && !loading" class="text-white text-center">
      <p class="text-xl">暂无图片</p>
    </div>

    <!-- 右键菜单 -->
    <div
      v-if="showContextMenu"
      class="fixed bg-base-100 shadow-lg rounded-lg py-1 min-w-32 z-50"
      :style="{ left: menuPosition.x + 'px', top: menuPosition.y + 'px' }"
      @click.stop
      @mousedown.stop
    >
      <button
        class="w-full px-4 py-2 text-left text-sm hover:bg-base-200 flex items-center gap-2"
        @click.stop="closeWindow"
      >
        <span class="text-error">✕</span>
        关闭窗口
      </button>
    </div>
  </div>
</template>

<style scoped>
/* 确保页面无边距无滚动条 */
:deep(html),
:deep(body) {
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>
