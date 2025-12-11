import { ref, onMounted } from 'vue'

// 可用主题列表
export const themes = [
  'light', 'dark', 'cupcake', 'bumblebee', 'emerald', 'corporate',
  'synthwave', 'retro', 'cyberpunk', 'valentine', 'halloween', 'garden',
  'forest', 'aqua', 'lofi', 'pastel', 'fantasy', 'wireframe', 'black',
  'luxury', 'dracula', 'cmyk', 'autumn', 'business', 'acid', 'lemonade',
  'night', 'coffee', 'winter', 'dim', 'nord', 'sunset',
]

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
