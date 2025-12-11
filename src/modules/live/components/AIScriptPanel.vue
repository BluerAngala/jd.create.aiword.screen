<script setup lang="ts">
/**
 * AI 话术面板组件
 * 显示 AI 生成的直播话术内容
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import type { AIScript } from '../types'

interface Props {
  scripts: AIScript[]
  currentIndex: number
}

interface Emits {
  (e: 'prev'): void
  (e: 'next'): void
  (e: 'openSettings'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 字体大小（rem）
const fontSize = ref(1)
const minFontSize = 0.75
const maxFontSize = 2

function increaseFontSize() {
  if (fontSize.value < maxFontSize) {
    fontSize.value = Math.min(maxFontSize, fontSize.value + 0.125)
  }
}

function decreaseFontSize() {
  if (fontSize.value > minFontSize) {
    fontSize.value = Math.max(minFontSize, fontSize.value - 0.125)
  }
}

// 当前话术
const currentScript = computed(() => props.scripts[props.currentIndex] || null)

// 当前话术字数
const wordCount = computed(() => currentScript.value?.content?.length || 0)

// 是否可以切换
const canPrev = computed(() => props.currentIndex > 0)
const canNext = computed(() => props.currentIndex < props.scripts.length - 1)

// 进度显示
const progress = computed(() => {
  if (props.scripts.length === 0) return '0/0'
  return `${props.currentIndex + 1}/${props.scripts.length}`
})
</script>

<template>
  <div class="card bg-base-100 shadow-sm h-full">
    <div class="card-body p-4 flex flex-col">
      <div class="flex items-center justify-between mb-2">
        <h3 class="card-title text-sm">
          <Icon icon="mdi:robot" class="text-lg" />
          AI 话术提词
        </h3>
        <div class="flex items-center gap-1">
          <button
            class="btn btn-outline btn-xs"
            :disabled="fontSize <= minFontSize"
            @click="decreaseFontSize"
          >
            <Icon icon="mdi:format-font-size-decrease" class="text-sm" />
            缩小
          </button>
          <button
            class="btn btn-outline btn-xs"
            :disabled="fontSize >= maxFontSize"
            @click="increaseFontSize"
          >
            <Icon icon="mdi:format-font-size-increase" class="text-sm" />
            放大
          </button>
          <button class="btn btn-outline btn-xs" @click="emit('openSettings')">
            <Icon icon="mdi:cog" class="text-sm" />
            设置
          </button>
          <span class="text-xs text-base-content/60 ml-1">{{ progress }}</span>
        </div>
      </div>

      <!-- 空状态 -->
      <div
        v-if="scripts.length === 0"
        class="flex-1 flex flex-col items-center justify-center text-base-content/60 text-center"
      >
        <Icon icon="mdi:text-box-remove-outline" class="text-4xl mb-2" />
        <p class="text-sm">暂无话术</p>
        <p class="text-xs">开始直播后将显示 AI 话术</p>
      </div>

      <!-- 话术内容 -->
      <div v-else class="flex-1 flex flex-col">
        <div class="flex-1 overflow-y-auto bg-base-200/50 rounded-lg p-3 mb-3 relative">
          <p class="leading-relaxed whitespace-pre-wrap" :style="{ fontSize: fontSize + 'rem' }">
            {{ currentScript?.content || '' }}
          </p>
          <!-- 字数统计 -->
          <span class="absolute bottom-2 right-2 text-xs text-base-content/40">{{ wordCount }} 字</span>
        </div>

        <!-- 切换按钮 -->
        <div class="flex justify-center gap-2">
          <button class="btn btn-sm btn-outline" :disabled="!canPrev" @click="emit('prev')">
            <Icon icon="mdi:chevron-left" />
            上一条
          </button>
          <button class="btn btn-sm btn-outline" :disabled="!canNext" @click="emit('next')">
            下一条
            <Icon icon="mdi:chevron-right" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
