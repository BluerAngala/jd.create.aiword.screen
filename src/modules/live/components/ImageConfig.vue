<script setup lang="ts">
/**
 * 图片配置组件 - 设置宽度和高度，支持投屏功能
 */
import { Icon } from '@iconify/vue'
import type { ImageSettings } from '../types'

interface Props {
  config: ImageSettings
  expanded?: boolean
  canScreen?: boolean // 是否可以投屏（有商品数据）
  isScreening?: boolean // 是否正在投屏
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

function updateSize(key: 'width' | 'height', value: number) {
  const newValue = Math.max(1, Number.isFinite(value) ? value : 1)
  emit('update', { ...props.config, [key]: newValue })
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
      <Icon icon="mdi:image" class="text-lg" />
      <span class="text-sm font-medium">图片设置</span>
      <!-- 投屏按钮（在标题栏） -->
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
    <div v-if="expanded" class="px-3 pb-2">
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2">
          <span class="text-xs text-base-content/60">宽度:</span>
          <input
            type="number"
            :value="config.width"
            min="1"
            class="input input-bordered input-xs w-20 text-center"
            @input="updateSize('width', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
        <div class="flex items-center gap-2">
          <span class="text-xs text-base-content/60">高度:</span>
          <input
            type="number"
            :value="config.height"
            min="1"
            class="input input-bordered input-xs w-20 text-center"
            @input="updateSize('height', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
      </div>
    </div>
  </div>
</template>
