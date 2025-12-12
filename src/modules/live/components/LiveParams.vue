<script setup lang="ts">
/**
 * 直播参数组件
 * 设置商品总数量、购物袋数量、直播时间
 */
import { computed, ref } from 'vue'
import { Icon } from '@iconify/vue'
import type { LiveParameters } from '../types'
import LiveTitleModal from './LiveTitleModal.vue'

interface Props {
  params: LiveParameters
}

interface Emits {
  (e: 'update', params: LiveParameters): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 标题弹窗显示状态
const showTitleModal = ref(false)

// 格式化日期时间为 input datetime-local 格式
function formatDateTimeLocal(date: Date): string {
  const pad = (n: number) => n.toString().padStart(2, '0')
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}T${pad(date.getHours())}:${pad(date.getMinutes())}`
}

// 当前时间字符串
const startTimeStr = computed(() => formatDateTimeLocal(props.params.startTime))

// 更新购物袋商品数量
function updateCartProducts(value: number) {
  emit('update', { ...props.params, cartProducts: Math.max(0, value) })
}

// 更新直播时间
function updateStartTime(value: string) {
  const date = new Date(value)
  if (!isNaN(date.getTime())) {
    emit('update', { ...props.params, startTime: date })
  }
}
</script>

<template>
  <div class="flex flex-wrap items-center gap-4">
    <!-- 设置直播间标题 -->
    <button class="btn btn-sm btn-neutral" @click="showTitleModal = true">
      <Icon icon="mdi:format-title" />
      设置直播间标题
    </button>

    <!-- 购物袋商品数量 -->
    <div class="flex items-center gap-2">
      <Icon icon="mdi:cart" class="text-base-content/60" />
      <span class="text-sm">购物袋数量</span>
      <input
        type="number"
        :value="params.cartProducts"
        min="0"
        class="input input-bordered input-sm w-20 text-center"
        @input="updateCartProducts(Number(($event.target as HTMLInputElement).value))"
      />
    </div>

    <!-- 直播时间 -->
    <div class="flex items-center gap-2">
      <Icon icon="mdi:clock-outline" class="text-base-content/60" />
      <span class="text-sm whitespace-nowrap">直播时间</span>
      <input
        type="datetime-local"
        :value="startTimeStr"
        class="input input-bordered input-sm w-48"
        @input="updateStartTime(($event.target as HTMLInputElement).value)"
      />
    </div>

    <!-- 直播间标题弹窗 -->
    <LiveTitleModal :visible="showTitleModal" @close="showTitleModal = false" />
  </div>
</template>
