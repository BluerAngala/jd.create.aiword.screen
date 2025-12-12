<script setup lang="ts">
/**
 * 图片配置组件 - 设置宽度、高度、初始位置，支持投屏功能
 */
import { Icon } from '@iconify/vue'
import type { ImageSettings } from '../types'

interface Props {
  config: ImageSettings
  expanded?: boolean
  canScreen?: boolean
  isScreening?: boolean
}

interface Emits {
  (e: 'update', config: ImageSettings): void
  (e: 'toggle'): void
  (e: 'startScreen'): void
  (e: 'stopScreen'): void
}

const props = withDefaults(defineProps<Props>(), {
  canScreen: false,
  isScreening: false,
})
const emit = defineEmits<Emits>()

const positions = [
  { value: 'top-left', label: '左上' },
  { value: 'top-right', label: '右上' },
  { value: 'bottom-left', label: '左下' },
  { value: 'bottom-right', label: '右下' },
] as const

function updateConfig<K extends keyof ImageSettings>(key: K, value: ImageSettings[K]) {
  emit('update', { ...props.config, [key]: key === 'position' ? value : Math.max(1, value as number) })
}

function toggleScreen() {
  if (props.isScreening) {
    emit('stopScreen')
  } else {
    emit('startScreen')
  }
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div
      class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer"
      @click="emit('toggle')"
    >
      <Icon icon="mdi:numeric-3-circle" class="text-xl" />
      <span class="text-sm font-medium">图片设置</span>
      <button
        class="btn btn-xs ml-auto mr-4"
        :class="isScreening ? 'btn-error' : 'btn-success'"
        :disabled="!canScreen && !isScreening"
        @click.stop="toggleScreen"
      >
        <Icon :icon="isScreening ? 'mdi:cast-off' : 'mdi:cast'" class="text-sm" />
        {{ isScreening ? '停止投屏' : '开启投屏' }}
      </button>
    </div>
    <div v-if="expanded" class="px-3 pb-3 space-y-3">
      <!-- 尺寸设置 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-sm">宽度</span>
          <input
            type="number"
            :value="config.width"
            min="1"
            class="input input-bordered input-sm w-24 text-center"
            @input="updateConfig('width', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
        <div class="flex items-center gap-2">
          <span class="text-sm">高度</span>
          <input
            type="number"
            :value="config.height"
            min="1"
            class="input input-bordered input-sm w-24 text-center"
            @input="updateConfig('height', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
      </div>
      <!-- 初始位置 -->
      <div class="flex items-center gap-3">
        <span class="text-sm">初始位置</span>
        <div class="flex gap-2">
          <button
            v-for="pos in positions"
            :key="pos.value"
            class="btn btn-sm"
            :class="config.position === pos.value ? 'btn-primary' : 'btn-ghost'"
            @click="updateConfig('position', pos.value)"
          >
            {{ pos.label }}
          </button>
        </div>
      </div>
      <!-- 提示 -->
      <div class="bg-base-200 rounded-lg p-3 text-xs text-base-content/70 space-y-1">
        <div class="flex items-center gap-1.5">
          <Icon icon="mdi:lightbulb-outline" class="text-warning" />
          <span>点击「开启投屏」显示图片窗口</span>
        </div>
        <div class="flex items-center gap-1.5">
          <Icon icon="mdi:lightbulb-outline" class="text-warning" />
          <span>左侧 AI 话术的上下一条按钮会自动切换商品图片</span>
        </div>
      </div>
    </div>
  </div>
</template>
