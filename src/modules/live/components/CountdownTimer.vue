<script setup lang="ts">
/**
 * 倒计时组件
 * 显示距离直播开始的剩余时间，支持设置讲解时间和休息时间
 */
import { ref, computed, watch, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'

interface Props {
  targetTime: Date | null
  isRunning: boolean
  explainDuration?: number // 讲解时长（秒）
  restDuration?: number // 休息时长（秒）
}

interface Emits {
  (e: 'complete'): void
  (e: 'update:explainDuration', value: number): void
  (e: 'update:restDuration', value: number): void
}

const props = withDefaults(defineProps<Props>(), {
  explainDuration: 60,
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
  },
)
watch(
  () => props.restDuration,
  (val) => {
    localRestDuration.value = val
  },
)

// 更新讲解时长
function updateExplainDuration(value: number) {
  const val = Math.max(10, Math.min(600, value)) // 10秒 - 10分钟
  localExplainDuration.value = val
  emit('update:explainDuration', val)
}

// 更新休息时长
function updateRestDuration(value: number) {
  const val = Math.max(5, Math.min(120, value)) // 5秒 - 2分钟
  localRestDuration.value = val
  emit('update:restDuration', val)
}

// 计算剩余秒数
const remainingSeconds = computed(() => {
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

// 启动/停止计时器
function startTimer() {
  if (timer) return
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
  { immediate: true },
)

onUnmounted(() => {
  stopTimer()
})
</script>

<template>
  <div class="card bg-base-100 shadow-sm">
    <div class="card-body p-4">
      <div class="flex items-start gap-4">
        <!-- 左侧：讲解时间设置 -->
        <div class="flex flex-col items-center gap-1">
          <span class="text-xs text-base-content/60">讲解时间</span>
          <div class="flex items-center gap-1">
            <input
              type="number"
              :value="localExplainDuration"
              min="10"
              max="600"
              class="input input-bordered input-xs w-16 text-center"
              @input="updateExplainDuration(Number(($event.target as HTMLInputElement).value))"
            />
            <span class="text-xs text-base-content/60">秒</span>
          </div>
        </div>

        <!-- 中间：倒计时 -->
        <div class="flex-1 flex flex-col items-center">
          <h3 class="card-title text-sm mb-2">
            <Icon icon="mdi:timer-outline" class="text-lg" />
            倒计时
          </h3>

          <!-- 未开始状态 -->
          <div
            v-if="!isRunning && !targetTime"
            class="py-2 text-base-content/60 flex flex-col items-center justify-center"
          >
            <Icon icon="mdi:clock-outline" class="text-3xl mb-1" />
            <p class="text-xs">等待开始</p>
          </div>

          <!-- 倒计时显示 -->
          <div v-else class="py-1">
            <div
              class="font-mono text-3xl font-bold"
              :class="isComplete ? 'text-success' : 'text-primary'"
            >
              {{ displayTime }}
            </div>
            <p v-if="isComplete" class="text-success text-xs mt-1 text-center">
              <Icon icon="mdi:check-circle" class="inline" />
              倒计时结束
            </p>
            <p v-else-if="isRunning" class="text-base-content/60 text-xs mt-1 text-center">
              距离直播开始
            </p>
          </div>
        </div>

        <!-- 右侧：休息时间设置 -->
        <div class="flex flex-col items-center gap-1">
          <span class="text-xs text-base-content/60">休息时间</span>
          <div class="flex items-center gap-1">
            <input
              type="number"
              :value="localRestDuration"
              min="5"
              max="120"
              class="input input-bordered input-xs w-16 text-center"
              @input="updateRestDuration(Number(($event.target as HTMLInputElement).value))"
            />
            <span class="text-xs text-base-content/60">秒</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
