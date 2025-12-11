<script setup lang="ts">
/**
 * 倒计时组件
 * 显示距离直播开始的剩余时间
 */
import { ref, computed, watch, onUnmounted } from 'vue'
import { Icon } from '@iconify/vue'

interface Props {
  targetTime: Date | null
  isRunning: boolean
}

interface Emits {
  (e: 'complete'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const now = ref(new Date())
let timer: ReturnType<typeof setInterval> | null = null

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
    <div class="card-body p-4 items-center text-center">
      <h3 class="card-title text-sm mb-2">
        <Icon icon="mdi:timer-outline" class="text-lg" />
        倒计时
      </h3>

      <!-- 未开始状态 -->
      <div v-if="!isRunning && !targetTime" class="py-4 text-base-content/60 flex flex-col items-center justify-center">
        <Icon icon="mdi:clock-outline" class="text-4xl mb-2" />
        <p class="text-sm">等待开始</p>
      </div>

      <!-- 倒计时显示 -->
      <div v-else class="py-2">
        <div
          class="font-mono text-4xl font-bold"
          :class="isComplete ? 'text-success' : 'text-primary'"
        >
          {{ displayTime }}
        </div>
        <p v-if="isComplete" class="text-success text-sm mt-2">
          <Icon icon="mdi:check-circle" class="inline" />
          倒计时结束
        </p>
        <p v-else-if="isRunning" class="text-base-content/60 text-xs mt-2">距离直播开始</p>
      </div>
    </div>
  </div>
</template>
