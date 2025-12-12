<script setup lang="ts">
/**
 * 倒计时投屏页面
 * 独立窗口显示倒计时，供 OBS 捕获
 */
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// 窗口标签
const WINDOW_LABEL = 'screen-countdown'

// 目标时间
const targetTime = ref<Date | null>(null)

// 剩余时间（秒）
const remainingSeconds = ref(0)

// 定时器
let timer: number | null = null

// 右键菜单状态
const showContextMenu = ref(false)
const menuPosition = ref({ x: 0, y: 0 })
let menuTimer: number | null = null

// 格式化时间显示
const formattedTime = computed(() => {
  const total = remainingSeconds.value
  if (total <= 0) return '00:00:00'
  const hours = Math.floor(total / 3600)
  const minutes = Math.floor((total % 3600) / 60)
  const seconds = total % 60
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
})

// 是否已结束
const isFinished = computed(() => remainingSeconds.value <= 0 && targetTime.value !== null)

onMounted(() => {
  // 从 hash URL 参数获取目标时间
  const hash = window.location.hash
  const queryIndex = hash.indexOf('?')
  if (queryIndex !== -1) {
    const params = new URLSearchParams(hash.slice(queryIndex))
    const timeStr = params.get('targetTime')
    if (timeStr) {
      targetTime.value = new Date(decodeURIComponent(timeStr))
      startTimer()
    }
  }

  // 点击其他地方关闭菜单
  document.addEventListener('click', () => {
    showContextMenu.value = false
  })
})

onUnmounted(() => {
  if (timer) {
    clearInterval(timer)
  }
})

function startTimer() {
  updateRemaining()
  timer = window.setInterval(updateRemaining, 1000)
}

function updateRemaining() {
  if (!targetTime.value) return
  const diff = Math.max(0, Math.floor((targetTime.value.getTime() - Date.now()) / 1000))
  remainingSeconds.value = diff
  if (diff <= 0 && timer) {
    clearInterval(timer)
  }
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

// 关闭窗口
async function closeWindow() {
  await invoke('close_screen_window', { label: WINDOW_LABEL })
}

// 开始拖动窗口
async function startDrag(e: MouseEvent) {
  if (e.button !== 0) return
  e.preventDefault()
  await invoke('start_dragging_window', { label: WINDOW_LABEL })
}
</script>

<template>
  <div
    class="w-full h-screen bg-white flex items-center justify-center overflow-hidden select-none cursor-move"
    @mousedown="startDrag"
    @contextmenu="handleContextMenu"
  >
    <!-- 倒计时显示 -->
    <div class="text-center w-full px-4">
      <div
        class="font-mono font-bold countdown-text"
        :class="isFinished ? 'text-green-500' : 'text-red-500'"
      >
        {{ formattedTime }}
      </div>
      <p v-if="isFinished" class="text-green-500 subtitle-text mt-2">直播开始！</p>
      <p v-else-if="!targetTime" class="text-gray-400 subtitle-text">等待开始...</p>
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
/* 倒计时文字自适应窗口宽度 */
.countdown-text {
  font-size: clamp(2rem, 18vw, 12rem);
  line-height: 1;
}

/* 副标题自适应 */
.subtitle-text {
  font-size: clamp(0.875rem, 4vw, 2rem);
}
</style>
