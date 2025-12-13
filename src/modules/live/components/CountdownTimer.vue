<script setup lang="ts">
/**
 * 倒计时组件
 * 显示距离直播开始的剩余时间，支持设置讲解时间和休息时间
 */
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import { emit as tauriEmit, listen, type UnlistenFn } from '@tauri-apps/api/event'

interface Props {
  targetTime: Date | null
  isRunning: boolean
  isPaused?: boolean // 是否暂停中
  pausedRemaining?: number | null // 暂停时的剩余秒数
  phase?: 'prepare' | 'explain' | 'rest' // 当前阶段
  explainDuration?: number // 讲解时长（秒）
  restDuration?: number // 休息时长（秒）
}

interface Emits {
  (e: 'complete'): void
  (e: 'update:explainDuration', value: number): void
  (e: 'update:restDuration', value: number): void
  (e: 'start'): void // 开始计时
  (e: 'pause'): void // 暂停计时
  (e: 'stop'): void // 结束计时
}

// 倒计时投屏状态
const isCountdownScreening = ref(false)

// 窗口关闭事件监听器
let unlistenClose: UnlistenFn | null = null
let unlistenReady: UnlistenFn | null = null

const props = withDefaults(defineProps<Props>(), {
  isPaused: false,
  pausedRemaining: null,
  phase: 'explain',
  explainDuration: 70,
  restDuration: 10,
})
const emit = defineEmits<Emits>()

const now = ref(new Date())
let timer: ReturnType<typeof setInterval> | null = null

// 本地编辑状态
const localExplainDuration = ref(props.explainDuration)
const localRestDuration = ref(props.restDuration)

// 同步 props 变化
watch(
  () => props.explainDuration,
  (val) => {
    localExplainDuration.value = val
  }
)
watch(
  () => props.restDuration,
  (val) => {
    localRestDuration.value = val
  }
)

// 更新讲解时长
function updateExplainDuration(value: number) {
  const val = Math.max(1, value) // 最小1秒，无上限
  localExplainDuration.value = val
  emit('update:explainDuration', val)
}

// 更新休息时长
function updateRestDuration(value: number) {
  const val = Math.max(1, value) // 最小1秒，无上限
  localRestDuration.value = val
  emit('update:restDuration', val)
}

// 计算剩余秒数
const remainingSeconds = computed(() => {
  // 暂停状态下显示暂停时的剩余秒数
  if (props.isPaused && props.pausedRemaining !== null) {
    return props.pausedRemaining
  }
  if (!props.targetTime) return 0
  const diff = Math.floor((props.targetTime.getTime() - now.value.getTime()) / 1000)
  return Math.max(0, diff)
})

// 格式化显示
const displayTime = computed(() => {
  const total = remainingSeconds.value
  const hours = Math.floor(total / 3600)
  const minutes = Math.floor((total % 3600) / 60)
  const seconds = total % 60

  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`
})

// 是否已结束
const isComplete = computed(() => remainingSeconds.value === 0 && props.isRunning)

// 是否在最后 10 秒（紧急状态）
const isUrgent = computed(() => remainingSeconds.value > 0 && remainingSeconds.value <= 10)

// 启动/停止计时器
function startTimer() {
  if (timer) return
  // 立即更新当前时间，避免显示偏差
  now.value = new Date()
  timer = setInterval(() => {
    now.value = new Date()
    if (remainingSeconds.value === 0 && props.isRunning) {
      emit('complete')
      stopTimer()
    }
  }, 1000)
}

function stopTimer() {
  if (timer) {
    clearInterval(timer)
    timer = null
  }
}

// 监听运行状态
watch(
  () => props.isRunning,
  (running) => {
    if (running) {
      startTimer()
    } else {
      stopTimer()
    }
  },
  { immediate: true }
)

// 同步倒计时数据到投屏窗口
function syncToScreen() {
  if (!isCountdownScreening.value) return
  tauriEmit('countdown-sync-to-screen', {
    targetTime: props.targetTime?.toISOString() ?? null,
    isRunning: props.isRunning,
  })
}

onMounted(async () => {
  // 监听倒计时投屏窗口关闭事件
  unlistenClose = await listen('screen-countdown-closed', () => {
    isCountdownScreening.value = false
  })

  // 监听投屏窗口准备好事件，发送当前状态
  unlistenReady = await listen('countdown-screen-ready', () => {
    syncToScreen()
  })
})

onUnmounted(() => {
  stopTimer()
  if (unlistenClose) {
    unlistenClose()
  }
  if (unlistenReady) {
    unlistenReady()
  }
})

// 监听 props 变化，同步到投屏窗口
watch(
  () => [props.targetTime, props.isRunning],
  () => {
    syncToScreen()
  }
)

// 开启倒计时投屏
async function startCountdownScreen() {
  try {
    const targetTimeStr = props.targetTime ? encodeURIComponent(props.targetTime.toISOString()) : ''
    await invoke('create_screen_window', {
      label: 'screen-countdown',
      title: '倒计时投屏',
      width: 400,
      height: 200,
      transparent: false,
      alwaysOnTop: true,
      decorations: false,
      resizable: true,
      backgroundColor: '#000000',
      extraParams: `targetTime=${targetTimeStr}`,
    })
    isCountdownScreening.value = true
  } catch (error) {
    console.error('开启倒计时投屏失败:', error)
  }
}

// 关闭倒计时投屏
async function stopCountdownScreen() {
  try {
    await invoke('close_screen_window', { label: 'screen-countdown' })
  } catch {
    // 忽略
  }
  isCountdownScreening.value = false
}
</script>

<template>
  <div class="card bg-base-100 shadow-sm">
    <div class="card-body p-4">
      <div class="flex items-start gap-4">
        <!-- 左侧：讲解时间设置 -->
        <div class="flex items-center gap-2">
          <span class="text-sm font-bold text-base-content">讲解时间</span>
          <input
            type="number"
            :value="localExplainDuration"
            min="1"
            class="input input-bordered input-xs w-16 text-center"
            @input="updateExplainDuration(Number(($event.target as HTMLInputElement).value))"
          />
          <span class="text-xs text-base-content/60">秒</span>
        </div>

        <!-- 中间：倒计时 -->
        <div class="flex-1 flex flex-col items-center">
          <div class="flex items-center gap-2 mb-2">
            <h3 class="card-title text-sm">
              <Icon icon="mdi:timer-outline" class="text-lg" />
              倒计时
            </h3>
            <button
              class="btn btn-xs"
              :class="isCountdownScreening ? 'btn-error' : 'btn-success'"
              @click="isCountdownScreening ? stopCountdownScreen() : startCountdownScreen()"
            >
              <Icon :icon="isCountdownScreening ? 'mdi:cast-off' : 'mdi:cast'" class="text-sm" />
              {{ isCountdownScreening ? '停止' : '投屏' }}
            </button>
          </div>

          <!-- 倒计时显示（始终显示，默认归零） -->
          <div class="py-1 relative">
            <!-- 状态标签绝对定位在左边 -->
            <span
              v-if="isRunning || isPaused"
              :class="[
                'badge badge-lg absolute -left-32 top-1/2 -translate-y-1/2',
                phase === 'explain' ? 'badge-primary' : 'badge-success',
              ]"
            >
              {{ phase === 'prepare' ? '准备中' : phase === 'rest' ? '休息中' : '讲解中' }}
            </span>
            <!-- 倒计时数字居中 -->
            <div
              :class="[
                'font-mono text-4xl font-bold transition-all',
                isComplete
                  ? 'text-success'
                  : isUrgent
                    ? 'text-error animate-pulse'
                    : phase === 'prepare' || phase === 'rest'
                      ? 'text-success'
                      : 'text-primary',
              ]"
            >
              {{ displayTime }}
            </div>
            <p v-if="isComplete" class="text-success text-xs mt-1 text-center">
              <Icon icon="mdi:check-circle" class="inline" />
              倒计时结束
            </p>
          </div>

          <!-- 控制按钮 -->
          <div class="flex gap-2 mt-2">
            <!-- 未运行且未暂停：显示开始 -->
            <button
              v-if="!isRunning && !isPaused"
              class="btn btn-xs btn-success"
              @click="emit('start')"
            >
              <Icon icon="mdi:play" class="text-sm" />
              开始
            </button>
            <!-- 暂停中：显示继续和结束 -->
            <template v-else-if="!isRunning && isPaused">
              <button class="btn btn-xs btn-success" @click="emit('start')">
                <Icon icon="mdi:play" class="text-sm" />
                继续
              </button>
              <button class="btn btn-xs btn-error" @click="emit('stop')">
                <Icon icon="mdi:stop" class="text-sm" />
                结束
              </button>
            </template>
            <!-- 运行中：显示暂停和结束 -->
            <template v-else>
              <button class="btn btn-xs btn-warning" @click="emit('pause')">
                <Icon icon="mdi:pause" class="text-sm" />
                暂停
              </button>
              <button class="btn btn-xs btn-error" @click="emit('stop')">
                <Icon icon="mdi:stop" class="text-sm" />
                结束
              </button>
            </template>
          </div>
        </div>

        <!-- 右侧：休息时间设置 -->
        <div class="flex items-center gap-2">
          <span class="text-sm font-bold text-base-content">休息时间</span>
          <input
            type="number"
            :value="localRestDuration"
            min="1"
            class="input input-bordered input-xs w-16 text-center"
            @input="updateRestDuration(Number(($event.target as HTMLInputElement).value))"
          />
          <span class="text-xs text-base-content/60">秒</span>
        </div>
      </div>
    </div>
  </div>
</template>
