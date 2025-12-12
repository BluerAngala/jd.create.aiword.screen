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

// 最小时间（当前时间 + 3 分钟）
const minTimeStr = computed(() => {
  const minDate = new Date(Date.now() + 3 * 60 * 1000)
  return formatDateTimeLocal(minDate)
})

// 最大时间（30 天后）
const maxTimeStr = computed(() => {
  const maxDate = new Date(Date.now() + 30 * 24 * 60 * 60 * 1000)
  return formatDateTimeLocal(maxDate)
})

// 快捷时间选项
const quickTimeOptions = [
  { label: '3分钟后', minutes: 3 },
  { label: '30分钟后', minutes: 30 },
  { label: '1小时后', minutes: 60 },
  { label: '3小时后', minutes: 180 },
  { label: '1天后', minutes: 1440 },
]

// 更新购物袋商品数量
function updateCartProducts(value: number) {
  emit('update', { ...props.params, cartProducts: Math.max(0, value) })
}

// 更新直播时间
function updateStartTime(value: string) {
  const date = new Date(value)
  if (!isNaN(date.getTime())) {
    // 校验时间必须在未来
    const minTime = Date.now() + 3 * 60 * 1000
    if (date.getTime() < minTime) {
      // 如果选择的时间小于最小时间，自动调整为最小时间
      emit('update', { ...props.params, startTime: new Date(minTime) })
    } else {
      emit('update', { ...props.params, startTime: date })
    }
  }
}

// 快捷设置时间
function setQuickTime(minutes: number) {
  const date = new Date(Date.now() + minutes * 60 * 1000)
  emit('update', { ...props.params, startTime: date })
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
        :min="minTimeStr"
        :max="maxTimeStr"
        class="input input-bordered input-sm w-40"
        @input="updateStartTime(($event.target as HTMLInputElement).value)"
      />
      <!-- 快捷时间选择 -->
      <div class="dropdown dropdown-top dropdown-end">
        <div tabindex="0" role="button" class="btn btn-sm btn-ghost">
          <Icon icon="mdi:clock-fast" />
          快捷时间
        </div>
        <ul
          tabindex="0"
          class="dropdown-content menu bg-base-200 rounded-box z-50 w-32 p-1 shadow-lg mb-1"
        >
          <li v-for="opt in quickTimeOptions" :key="opt.minutes">
            <a class="text-sm" @click="setQuickTime(opt.minutes)">{{ opt.label }}</a>
          </li>
        </ul>
      </div>
    </div>

    <!-- 直播间标题弹窗 -->
    <LiveTitleModal :visible="showTitleModal" @close="showTitleModal = false" />
  </div>
</template>
