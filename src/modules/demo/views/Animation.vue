<script setup lang="ts">
import { ref } from 'vue'
import { useMotion } from '@vueuse/motion'
import { useAutoAnimate } from '@formkit/auto-animate/vue'
import { Icon } from '@iconify/vue'

// 列表自动动画
const [listParent] = useAutoAnimate()
void listParent
const items = ref(['项目 1', '项目 2', '项目 3'])
let counter = 4

function addItem() {
  items.value.push(`项目 ${counter++}`)
}

function removeItem(index: number) {
  items.value.splice(index, 1)
}

function shuffleItems() {
  items.value = items.value.sort(() => Math.random() - 0.5)
}

// Motion 动画示例
const boxRef = ref<HTMLElement>()
const { apply } = useMotion(boxRef, {
  initial: { scale: 1, rotate: 0 },
  enter: { scale: 1, rotate: 0 },
})

function bounce() {
  apply({ scale: 1.2 })
  setTimeout(() => apply({ scale: 1 }), 150)
}

function spin() {
  apply({ rotate: 360 })
  setTimeout(() => apply({ rotate: 0 }), 500)
}

// 卡片动画
const cards = ref([
  { id: 1, title: '卡片 1', icon: 'mdi:rocket-launch' },
  { id: 2, title: '卡片 2', icon: 'mdi:lightning-bolt' },
  { id: 3, title: '卡片 3', icon: 'mdi:star' },
])
const [cardsParent] = useAutoAnimate()
void cardsParent
</script>

<template>
  <div class="space-y-8">
    <h2 class="text-2xl font-bold">动画示例</h2>

    <!-- Motion 动画 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">
          <Icon icon="mdi:animation" class="text-primary" />
          Motion 动画
        </h3>
        <p class="text-base-content/70">使用 @vueuse/motion 实现流畅动画</p>

        <div class="flex items-center gap-8 mt-4">
          <div
            ref="boxRef"
            class="w-24 h-24 bg-gradient-to-br from-primary to-secondary rounded-xl flex items-center justify-center cursor-pointer"
            @click="bounce"
          >
            <Icon icon="mdi:cursor-default-click" class="text-3xl text-white" />
          </div>

          <div class="flex flex-col gap-2">
            <button class="btn btn-primary btn-sm" @click="bounce">
              <Icon icon="mdi:arrow-expand" />
              弹跳
            </button>
            <button class="btn btn-secondary btn-sm" @click="spin">
              <Icon icon="mdi:rotate-right" />
              旋转
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 列表自动动画 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">
          <Icon icon="mdi:format-list-bulleted" class="text-secondary" />
          列表自动动画
        </h3>
        <p class="text-base-content/70">使用 auto-animate 自动添加列表动画</p>

        <div class="flex gap-2 mt-4">
          <button class="btn btn-primary btn-sm" @click="addItem">
            <Icon icon="mdi:plus" />
            添加
          </button>
          <button class="btn btn-secondary btn-sm" @click="shuffleItems">
            <Icon icon="mdi:shuffle" />
            打乱
          </button>
        </div>

        <ul ref="listParent" class="mt-4 space-y-2">
          <li
            v-for="(item, index) in items"
            :key="item"
            class="flex items-center justify-between p-3 bg-base-200 rounded-lg"
          >
            <span>{{ item }}</span>
            <button class="btn btn-ghost btn-xs btn-circle" @click="removeItem(index)">
              <Icon icon="mdi:close" />
            </button>
          </li>
        </ul>
      </div>
    </div>

    <!-- 卡片网格动画 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">
          <Icon icon="mdi:view-grid" class="text-accent" />
          卡片网格
        </h3>

        <div ref="cardsParent" class="grid grid-cols-3 gap-4 mt-4">
          <div
            v-for="card in cards"
            :key="card.id"
            v-motion
            :initial="{ opacity: 0, y: 20 }"
            :enter="{ opacity: 1, y: 0 }"
            :hovered="{ scale: 1.05 }"
            class="card bg-base-200 cursor-pointer"
          >
            <div class="card-body items-center text-center p-4">
              <Icon :icon="card.icon" class="text-4xl text-primary" />
              <span class="font-medium">{{ card.title }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
