<script setup lang="ts">
/**
 * 执行日志组件 - 支持折叠
 */
import { ref, watch, nextTick } from 'vue'
import { Icon } from '@iconify/vue'
import type { LogEntry } from '../types'

interface Props {
  logs: LogEntry[]
  maxHeight?: string
  expanded?: boolean
}

interface Emits {
  (e: 'toggle'): void
}

const props = withDefaults(defineProps<Props>(), {
  maxHeight: '100px',
  expanded: false,
})
const emit = defineEmits<Emits>()
const logContainer = ref<HTMLElement | null>(null)

const levelConfig = {
  info: { icon: 'mdi:information', class: 'text-info' },
  warn: { icon: 'mdi:alert', class: 'text-warning' },
  error: { icon: 'mdi:alert-circle', class: 'text-error' },
  success: { icon: 'mdi:check-circle', class: 'text-success' },
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit', second: '2-digit' })
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
    </div>
    <div v-show="expanded" class="collapse-content px-3 pb-2">
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
