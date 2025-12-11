<script setup lang="ts">
import { useToast } from '@/core/composables'
import { Icon } from '@iconify/vue'

const { toasts, remove } = useToast()

const iconMap = {
  success: 'mdi:check-circle',
  error: 'mdi:alert-circle',
  info: 'mdi:information',
  warning: 'mdi:alert',
}

const alertClass = {
  success: 'alert-success',
  error: 'alert-error',
  info: 'alert-info',
  warning: 'alert-warning',
}
</script>

<template>
  <div class="toast toast-top toast-center z-50">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        class="alert shadow-lg cursor-pointer min-w-80"
        :class="alertClass[toast.type]"
        @click="remove(toast.id)"
      >
        <Icon :icon="iconMap[toast.type]" class="text-xl" />
        <span>{{ toast.message }}</span>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100px);
}
</style>
