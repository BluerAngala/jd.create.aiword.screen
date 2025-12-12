<script setup lang="ts">
/**
 * 公告栏组件
 * 显示软件公告信息，支持滚动或静态显示
 */
import { computed } from 'vue'
import { Icon } from '@iconify/vue'

interface Props {
  content: string // 公告内容
  scrolling?: boolean // 是否滚动显示
}

const props = withDefaults(defineProps<Props>(), {
  scrolling: false,
})

const hasContent = computed(() => props.content.trim().length > 0)
</script>

<template>
  <div
    v-if="hasContent"
    class="flex items-center gap-2 px-4 py-2 bg-blue-500 text-white rounded-lg"
  >
    <Icon icon="mdi:bullhorn" class="text-lg shrink-0" />
    <span v-if="!scrolling">{{ content }}</span>
    <div v-else class="overflow-hidden flex-1">
      <div class="animate-marquee whitespace-nowrap">
        {{ content }}
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes marquee {
  0% {
    transform: translateX(100%);
  }
  100% {
    transform: translateX(-100%);
  }
}

.animate-marquee {
  animation: marquee 15s linear infinite;
}
</style>
