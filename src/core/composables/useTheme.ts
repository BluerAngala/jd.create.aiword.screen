import { ref, onMounted } from 'vue'

// 可用主题列表（商务风格）
export const themes = ['light', 'dark', 'business']

const currentTheme = ref('light')

export function useTheme() {
  // 切换主题
  function setTheme(theme: string) {
    currentTheme.value = theme
    document.documentElement.setAttribute('data-theme', theme)
    localStorage.setItem('theme', theme)
  }

  // 初始化主题
  function initTheme() {
    const saved = localStorage.getItem('theme') || 'light'
    setTheme(saved)
  }

  onMounted(() => {
    initTheme()
  })

  return {
    themes,
    currentTheme,
    setTheme,
    initTheme,
  }
}
