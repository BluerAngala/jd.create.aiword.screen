<script setup lang="ts">
/**
 * 图片配置组件 - 只设置宽度和高度
 */
import { Icon } from '@iconify/vue'
import type { ImageSettings } from '../types'

interface Props {
  config: ImageSettings
  accordionName?: string
}

interface Emits {
  (e: 'update', config: ImageSettings): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

function updateSize(key: 'width' | 'height', value: number) {
  const newValue = Math.max(1, Number.isFinite(value) ? value : 1)
  emit('update', { ...props.config, [key]: newValue })
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm">
    <input type="radio" :name="accordionName" />
    <div class="collapse-title py-2 px-3 min-h-0 flex items-center gap-2">
      <Icon icon="mdi:image" class="text-lg" />
      <span class="text-sm font-medium">图片设置</span>
    </div>
    <div class="collapse-content px-3 pb-2">
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
