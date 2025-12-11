<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, computed } from 'vue'
import { useRoute } from 'vue-router'
import { getCurrentWindow, type Window } from '@tauri-apps/api/window'

const route = useRoute()

// 缓存窗口实例
let appWindow: Window | null = null

onMounted(async () => {
  appWindow = getCurrentWindow()
})

// 拖拽窗口 - 使用 pointerdown 事件更可靠
async function startDrag(e: PointerEvent) {
  if (e.button !== 0) return
  try {
    if (appWindow) {
      await appWindow.startDragging()
    }
  } catch {
    // 忽略错误
  }
}

// 右键菜单
const contextMenu = reactive({ show: false, x: 0, y: 0 })

function showContextMenu(e: MouseEvent) {
  e.preventDefault()
  contextMenu.show = true
  contextMenu.x = e.clientX
  contextMenu.y = e.clientY
}

function hideContextMenu() {
  contextMenu.show = false
}

async function closeCurrentWindow() {
  hideContextMenu()
  if (appWindow) {
    await appWindow.close()
  }
}

// 从 URL 参数获取配置
const contentType = computed(() => (route.query.type as string) || 'clock')
const textColor = computed(() => (route.query.color as string) || '#ffffff')
const backgroundColor = computed(() => (route.query.bg as string) || '#000000')
const fontSize = computed(() => parseInt((route.query.size as string) || '48'))
const isTransparent = computed(() => route.query.transparent === 'true')

// 倒计时
const countdownMinutes = computed(() => parseInt((route.query.minutes as string) || '5'))

// 滚动文字
const marqueeText = computed(() => (route.query.text as string) || '欢迎来到直播间！')
const marqueeSpeed = computed(() => parseInt((route.query.speed as string) || '50'))

// 渐变
const gradientFrom = computed(() => (route.query.from as string) || '#667eea')
const gradientTo = computed(() => (route.query.to as string) || '#764ba2')
const gradientAngle = computed(() => parseInt((route.query.angle as string) || '135'))

// 图片
const imageUrl = computed(() => (route.query.url as string) || '')
const imageFit = computed(() => (route.query.fit as string) || 'cover')

// 粒子
const particleColor = computed(() => (route.query.pcolor as string) || '#ffffff')
const particleCount = computed(() => parseInt((route.query.pcount as string) || '50'))
const particleBgColor = computed(() => (route.query.pbg as string) || '#1a1a2e')

// 粒子数据
interface Particle {
  x: number
  y: number
  size: number
  speedX: number
  speedY: number
  opacity: number
}
const particles = ref<Particle[]>([])

// 时钟数据
const currentTime = ref('')
const currentDate = ref('')

// 倒计时数据
const countdown = ref(0)
const countdownEnded = ref(false)
const countdownDisplay = computed(() => {
  const mins = Math.floor(countdown.value / 60)
  const secs = countdown.value % 60
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
})

let timer: number | null = null
let animationId: number | null = null

onMounted(() => {
  if (contentType.value === 'clock') {
    const update = () => {
      const now = new Date()
      currentTime.value = now.toLocaleTimeString('zh-CN')
      currentDate.value = now.toLocaleDateString('zh-CN', {
        month: 'long',
        day: 'numeric',
        weekday: 'short',
      })
    }
    update()
    timer = window.setInterval(update, 1000)
  } else if (contentType.value === 'countdown') {
    countdown.value = countdownMinutes.value * 60
    timer = window.setInterval(() => {
      if (countdown.value > 0) {
        countdown.value--
      } else {
        countdownEnded.value = true
      }
    }, 1000)
  } else if (contentType.value === 'particles') {
    for (let i = 0; i < particleCount.value; i++) {
      particles.value.push({
        x: Math.random() * 100,
        y: Math.random() * 100,
        size: Math.random() * 4 + 2,
        speedX: (Math.random() - 0.5) * 0.3,
        speedY: -Math.random() * 0.5 - 0.2,
        opacity: Math.random() * 0.6 + 0.4,
      })
    }
    const animate = () => {
      particles.value.forEach((p) => {
        p.x += p.speedX
        p.y += p.speedY
        if (p.y < -5) {
          p.y = 105
          p.x = Math.random() * 100
        }
        if (p.x < -5) p.x = 105
        if (p.x > 105) p.x = -5
      })
      animationId = window.requestAnimationFrame(animate)
    }
    animate()
  }
})

onUnmounted(() => {
  if (timer) window.clearInterval(timer)
  if (animationId) window.cancelAnimationFrame(animationId)
})

// 背景样式
const bgStyle = computed(() => {
  if (isTransparent.value) return {}
  if (contentType.value === 'gradient') {
    return {
      background: `linear-gradient(${gradientAngle.value}deg, ${gradientFrom.value}, ${gradientTo.value})`,
    }
  }
  return { backgroundColor: backgroundColor.value }
})

// 文字样式
const textStyle = computed(() => ({
  color: textColor.value,
  fontSize: `${fontSize.value}px`,
}))

// 滚动动画时长
const marqueeDuration = computed(() => `${Math.max(5, 30 - marqueeSpeed.value / 5)}s`)
</script>

<template>
  <div
    class="w-screen h-screen flex items-center justify-center overflow-hidden select-none cursor-move"
    :style="bgStyle"
    @pointerdown="startDrag"
    @contextmenu="showContextMenu"
  >
    <!-- 实时时钟 -->
    <div v-if="contentType === 'clock'" class="text-center">
      <p class="font-bold font-mono drop-shadow-lg" :style="textStyle">
        {{ currentTime }}
      </p>
      <p class="mt-2 opacity-70" :style="{ color: textColor, fontSize: `${fontSize * 0.35}px` }">
        {{ currentDate }}
      </p>
    </div>

    <!-- 倒计时 -->
    <div v-else-if="contentType === 'countdown'" class="text-center">
      <p
        class="font-bold font-mono drop-shadow-lg"
        :class="{ 'animate-pulse': countdownEnded }"
        :style="{
          ...textStyle,
          color: countdownEnded ? '#ef4444' : textColor,
        }"
      >
        {{ countdownDisplay }}
      </p>
      <p
        v-if="countdownEnded"
        class="mt-2 animate-bounce"
        :style="{ color: '#ef4444', fontSize: `${fontSize * 0.4}px` }"
      >
        时间到！
      </p>
    </div>

    <!-- 滚动文字 -->
    <div v-else-if="contentType === 'marquee'" class="w-full whitespace-nowrap">
      <p
        class="inline-block font-bold drop-shadow-lg animate-marquee"
        :style="{ ...textStyle, animationDuration: marqueeDuration }"
      >
        {{ marqueeText }}
      </p>
    </div>

    <!-- 图片展示 -->
    <div v-else-if="contentType === 'image'" class="w-full h-full">
      <img
        v-if="imageUrl"
        :src="imageUrl"
        alt=""
        class="w-full h-full object-cover"
        :class="{
          'object-cover': imageFit === 'cover',
          'object-contain': imageFit === 'contain',
          'object-fill': imageFit === 'fill',
        }"
      />
    </div>

    <!-- 粒子效果 -->
    <div
      v-else-if="contentType === 'particles'"
      class="relative w-full h-full overflow-hidden"
      :style="{ backgroundColor: particleBgColor }"
    >
      <div
        v-for="(p, i) in particles"
        :key="i"
        class="absolute rounded-full"
        :style="{
          left: `${p.x}%`,
          top: `${p.y}%`,
          width: `${p.size}px`,
          height: `${p.size}px`,
          backgroundColor: particleColor,
          opacity: p.opacity,
          boxShadow: `0 0 ${p.size * 2}px ${particleColor}`,
        }"
      ></div>
    </div>

    <!-- 纯色背景 -->
    <div v-else-if="contentType === 'solid'"></div>

    <!-- 渐变背景 -->
    <div v-else-if="contentType === 'gradient'"></div>
  </div>

  <!-- 右键菜单 -->
  <Teleport to="body">
    <div v-if="contextMenu.show" class="fixed inset-0 z-9998" @click="hideContextMenu"></div>
    <div
      v-if="contextMenu.show"
      class="fixed z-9999"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
    >
      <ul class="menu bg-base-100 rounded-box shadow-lg w-32 border border-base-300">
        <li>
          <button class="text-error hover:bg-error hover:text-white" @click="closeCurrentWindow">
            关闭窗口
          </button>
        </li>
      </ul>
    </div>
  </Teleport>
</template>

<style scoped>
@keyframes marquee {
  0% {
    transform: translateX(100vw);
  }
  100% {
    transform: translateX(-100%);
  }
}

.animate-marquee {
  animation: marquee linear infinite;
}
</style>
