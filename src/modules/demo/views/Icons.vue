<script setup lang="ts">
import { ref } from 'vue'
import { Icon } from '@iconify/vue'

// 常用图标集示例
const iconSets = [
  {
    name: 'Material Design Icons',
    prefix: 'mdi',
    icons: [
      'home',
      'account',
      'cog',
      'bell',
      'heart',
      'star',
      'search',
      'menu',
      'close',
      'check',
      'plus',
      'minus',
      'arrow-left',
      'arrow-right',
      'chevron-down',
      'chevron-up',
      'folder',
      'file',
      'image',
      'video',
      'music',
      'download',
    ],
  },
  {
    name: 'Heroicons',
    prefix: 'heroicons',
    icons: [
      'home',
      'user',
      'cog-6-tooth',
      'bell',
      'heart',
      'star',
      'magnifying-glass',
      'bars-3',
      'x-mark',
      'check',
      'plus',
      'minus',
    ],
  },
  {
    name: 'Lucide',
    prefix: 'lucide',
    icons: [
      'home',
      'user',
      'settings',
      'bell',
      'heart',
      'star',
      'search',
      'menu',
      'x',
      'check',
      'plus',
      'minus',
    ],
  },
]

const selectedIcon = ref('')
const iconSize = ref(24)

function copyIcon(prefix: string, icon: string) {
  const iconName = `${prefix}:${icon}`
  navigator.clipboard.writeText(`<Icon icon="${iconName}" />`)
  selectedIcon.value = iconName
}
</script>

<template>
  <div class="space-y-8">
    <h2 class="text-2xl font-bold">图标库</h2>

    <div class="alert alert-info">
      <Icon icon="mdi:information" class="text-xl" />
      <span>使用 @iconify/vue，支持 200+ 图标集，20万+ 图标。点击图标复制代码。</span>
    </div>

    <!-- 图标大小控制 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">图标大小</h3>
        <div class="flex items-center gap-4">
          <input
            v-model="iconSize"
            type="range"
            min="16"
            max="64"
            class="range range-primary"
          />
          <span class="badge badge-primary">{{ iconSize }}px</span>
        </div>
        <div class="flex gap-4 mt-4">
          <Icon icon="mdi:home" :style="{ fontSize: iconSize + 'px' }" />
          <Icon icon="mdi:account" :style="{ fontSize: iconSize + 'px' }" />
          <Icon icon="mdi:cog" :style="{ fontSize: iconSize + 'px' }" />
        </div>
      </div>
    </div>

    <!-- 图标集展示 -->
    <div v-for="set in iconSets" :key="set.prefix" class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">{{ set.name }}</h3>
        <p class="text-base-content/70">前缀: {{ set.prefix }}</p>

        <div class="grid grid-cols-6 sm:grid-cols-8 md:grid-cols-12 gap-2 mt-4">
          <button
            v-for="icon in set.icons"
            :key="icon"
            class="btn btn-ghost btn-square"
            :class="{ 'btn-active': selectedIcon === `${set.prefix}:${icon}` }"
            :title="`${set.prefix}:${icon}`"
            @click="copyIcon(set.prefix, icon)"
          >
            <Icon :icon="`${set.prefix}:${icon}`" class="text-xl" />
          </button>
        </div>
      </div>
    </div>

    <!-- 使用示例 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">使用示例</h3>

        <div class="mockup-code">
          <pre><code>&lt;script setup&gt;
import { Icon } from '@iconify/vue'
&lt;/script&gt;

&lt;template&gt;
  &lt;Icon icon="mdi:home" /&gt;
  &lt;Icon icon="mdi:account" class="text-2xl text-primary" /&gt;
  &lt;Icon icon="heroicons:heart" :style="{ fontSize: '32px' }" /&gt;
&lt;/template&gt;</code></pre>
        </div>

        <div v-if="selectedIcon" class="alert alert-success mt-4">
          <Icon icon="mdi:check" />
          <span>已复制: {{ selectedIcon }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
