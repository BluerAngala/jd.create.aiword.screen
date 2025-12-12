<script setup lang="ts">
/**
 * AI 话术面板组件
 * 显示 AI 生成的直播话术内容，支持商品讲解控制
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import type { AIScript } from '../types'

interface Props {
  scripts: AIScript[]
  currentIndex: number
  totalProducts?: number // 直播间商品总数
  isExplaining?: boolean // 是否正在讲解
  canExplain?: boolean // 是否可以讲解（直播间已创建）
}

interface Emits {
  (e: 'prev'): void
  (e: 'next'): void
  (e: 'openSettings'): void
  (e: 'startExplain', productId: string): void
  (e: 'endExplain', productId: string): void
}

const props = withDefaults(defineProps<Props>(), {
  totalProducts: 0,
  isExplaining: false,
  canExplain: false,
})
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

// 进度显示：当前索引/话术总数
const progress = computed(() => {
  if (props.scripts.length === 0) return '0/0'
  return `${props.currentIndex + 1}/${props.scripts.length}`
})

// 话术/商品数量显示
const scriptProductCount = computed(() => {
  if (props.totalProducts === 0) return ''
  return `${props.scripts.length}/${props.totalProducts}`
})

// 是否是第一条话术（显示开始讲解按钮）
const isFirstScript = computed(() => props.currentIndex === 0)

// 处理上一条（结束当前讲解，开始上一个商品讲解）
function handlePrev() {
  const currentProductId = currentScript.value?.productId
  if (currentProductId && props.isExplaining) {
    emit('endExplain', currentProductId)
  }
  emit('prev')
  // 延迟开始新商品讲解
  setTimeout(() => {
    const prevScript = props.scripts[props.currentIndex - 1]
    if (prevScript?.productId) {
      emit('startExplain', prevScript.productId)
    }
  }, 100)
}

// 处理下一条（结束当前讲解，开始下一个商品讲解）
function handleNext() {
  const currentProductId = currentScript.value?.productId
  if (currentProductId && props.isExplaining) {
    emit('endExplain', currentProductId)
  }
  emit('next')
  // 延迟开始新商品讲解
  setTimeout(() => {
    const nextScript = props.scripts[props.currentIndex + 1]
    if (nextScript?.productId) {
      emit('startExplain', nextScript.productId)
    }
  }, 100)
}

// 开始讲解第一个商品
function handleStartFirstExplain() {
  const productId = currentScript.value?.productId
  if (productId) {
    emit('startExplain', productId)
  }
}
</script>

<template>
  <div class="card bg-base-100 shadow-sm h-full">
    <div class="card-body p-4 flex flex-col">
      <!-- 标题栏 -->
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2">
          <h3 class="card-title text-sm">
            <Icon icon="mdi:robot" class="text-lg" />
            AI 话术提词
          </h3>
          <!-- 话术/商品数量 -->
          <span v-if="scriptProductCount" class="badge badge-ghost badge-sm">
            {{ scriptProductCount }}
          </span>
        </div>
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
            AI 设置
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
        <!-- 商品信息提示 -->
        <div v-if="currentScript?.productId" class="text-xs text-base-content/60 mb-1">
          <Icon icon="mdi:package-variant" class="inline" />
          商品 ID: {{ currentScript.productId }}
          <span v-if="isExplaining" class="badge badge-success badge-xs ml-1">讲解中</span>
        </div>

        <div class="flex-1 overflow-y-auto bg-base-200/50 rounded-lg p-3 mb-3 relative">
          <p class="leading-relaxed whitespace-pre-wrap" :style="{ fontSize: fontSize + 'rem' }">
            {{ currentScript?.content || '' }}
          </p>
          <!-- 字数统计 -->
          <span class="absolute bottom-2 right-2 text-xs text-base-content/40">{{ wordCount }} 字</span>
        </div>

        <!-- 切换按钮 - 始终显示上下一条 -->
        <div class="flex justify-center items-center gap-2">
          <button class="btn btn-sm btn-outline" :disabled="!canPrev" @click="handlePrev">
            <Icon icon="mdi:chevron-left" />
            上一条
          </button>

          <!-- 第一条话术且未开始讲解：显示开始讲解按钮 -->
          <button
            v-if="isFirstScript && !isExplaining"
            class="btn btn-sm btn-success"
            :disabled="!canExplain || !currentScript?.productId"
            @click="handleStartFirstExplain"
          >
            <Icon icon="mdi:microphone" />
            开始讲解
          </button>

          <button class="btn btn-sm btn-outline" :disabled="!canNext" @click="handleNext">
            下一条
            <Icon icon="mdi:chevron-right" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
