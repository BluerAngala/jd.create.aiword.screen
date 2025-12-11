import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAppStore = defineStore('app', () => {
  // 应用状态
  const isLoading = ref(false)
  const appVersion = ref('0.1.0')

  // 设置加载状态
  function setLoading(loading: boolean) {
    isLoading.value = loading
  }

  return {
    isLoading,
    appVersion,
    setLoading,
  }
})
