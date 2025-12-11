<script setup lang="ts">
import { ref } from 'vue'
import { useTheme } from '@/core/composables'

const { themes, currentTheme, setTheme } = useTheme()
const showModal = ref(false)

function selectTheme(theme: string) {
  setTheme(theme)
  showModal.value = false
}
</script>

<template>
  <button class="btn btn-ghost" @click="showModal = true">🎨 主题: {{ currentTheme }}</button>

  <!-- 主题选择弹窗 -->
  <dialog class="modal" :class="{ 'modal-open': showModal }">
    <div class="modal-box w-11/12 max-w-3xl">
      <h3 class="font-bold text-lg mb-4">选择主题</h3>
      <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 gap-3">
        <button
          v-for="theme in themes"
          :key="theme"
          class="overflow-hidden rounded-lg border-2 transition-all hover:scale-105"
          :class="currentTheme === theme ? 'border-primary' : 'border-base-300'"
          :data-theme="theme"
          @click="selectTheme(theme)"
        >
          <div class="bg-base-100 p-2">
            <div class="flex gap-1 mb-1">
              <div class="w-2 h-2 rounded-full bg-primary"></div>
              <div class="w-2 h-2 rounded-full bg-secondary"></div>
              <div class="w-2 h-2 rounded-full bg-accent"></div>
            </div>
            <div class="text-xs text-base-content truncate">{{ theme }}</div>
          </div>
        </button>
      </div>
      <div class="modal-action">
        <button class="btn" @click="showModal = false">关闭</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="showModal = false">关闭</button>
    </form>
  </dialog>
</template>
