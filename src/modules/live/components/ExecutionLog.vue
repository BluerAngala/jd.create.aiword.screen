<script setup lang="ts">
/**
 * 执行日志组件 - 支持折叠、清空、复制
 */
import { ref, watch, nextTick } from 'vue'
import { Icon } from '@iconify/vue'
import { useClipboard } from '@vueuse/core'
import type { LogEntry } from '../types'

interface Props {
  logs: LogEntry[]
  maxHeight?: string
  expanded?: boolean
}

interface Emits {
  (e: 'toggle'): void
  (e: 'clear'): void
}

const props = withDefaults(defineProps<Props>(), {
  maxHeight: '100px',
  expanded: false,
})
const emit = defineEmits<Emits>()
const logContainer = ref<HTMLElement | null>(null)
const { copy, copied } = useClipboard()

const levelConfig = {
  info: { icon: 'mdi:information', class: 'text-base-content' },
  warn: { icon: 'mdi:alert', class: 'text-base-content' },
  error: { icon: 'mdi:alert-circle', class: 'text-error' },
  success: { icon: 'mdi:check-circle', class: 'text-success' },
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
}

// 复制所有日志
function copyLogs() {
  const text = props.logs
    .map((log) => `[${formatTime(log.timestamp)}] [${log.level.toUpperCase()}] ${log.message}`)
    .join('\n')
  copy(text)
}

// 清空日志
function clearLogs() {
  emit('clear')
}

watch(
  () => props.logs.length,
  async () => {
    await nextTick()
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  },
)
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer" @click="emit('toggle')">
      <Icon icon="mdi:text-box-outline" class="text-lg" />
      <span class="text-sm font-medium flex-1">执行日志</span>
      <span class="text-xs text-base-content/50">{{ logs.length }} 条</span>
      <!-- 复制按钮 -->
      <button
        class="btn btn-ghost btn-xs"
        :class="{ 'text-success': copied }"
        :disabled="logs.length === 0"
        title="复制日志"
        @click.stop="copyLogs"
      >
        <Icon :icon="copied ? 'mdi:check' : 'mdi:content-copy'" />
        {{ copied ? '已复制' : '复制' }}
      </button>
      <!-- 清空按钮 -->
      <button
        class="btn btn-ghost btn-xs"
        :disabled="logs.length === 0"
        title="清空日志"
        @click.stop="clearLogs"
      >
        <Icon icon="mdi:delete-outline" />
        清空
      </button>
    </div>
    <div v-if="expanded" class="px-3 pb-2">
      <div v-if="logs.length === 0" class="text-center py-2 text-base-content/60 text-xs">
        暂无日志
      </div>
      <div v-else ref="logContainer" class="overflow-y-auto font-mono text-xs space-y-0.5 bg-base-200/50 rounded p-2" :style="{ maxHeight }">
        <div v-for="log in logs" :key="log.id" class="flex items-start gap-1" :class="levelConfig[log.level].class">
          <Icon :icon="levelConfig[log.level].icon" class="shrink-0 text-xs" />
          <span class="text-base-content/50">[{{ formatTime(log.timestamp) }}]</span>
          <span class="flex-1 break-all">{{ log.message }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
