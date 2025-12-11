<script setup lang="ts">
import { ref, reactive } from 'vue'
import { Icon } from '@iconify/vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogger } from '@/core/composables'

const logger = useLogger('ScreenWindow')

// 窗口配置接口
interface WindowConfig {
  id: string
  title: string
  width: number
  height: number
  transparent: boolean
  alwaysOnTop: boolean
  decorations: boolean
  resizable: boolean
  contentType: string
}

// 内容类型选项
const contentTypes = [
  { value: 'clock', label: '时钟', icon: 'mdi:clock' },
  { value: 'countdown', label: '倒计时', icon: 'mdi:timer' },
  { value: 'marquee', label: '滚动文字', icon: 'mdi:text' },
  { value: 'image', label: '图片', icon: 'mdi:image' },
  { value: 'particles', label: '粒子', icon: 'mdi:blur' },
  { value: 'solid', label: '纯色', icon: 'mdi:palette' },
  { value: 'gradient', label: '渐变', icon: 'mdi:gradient-horizontal' },
]

// 当前配置
const config = reactive({
  title: '投屏窗口',
  width: 400,
  height: 200,
  transparent: true,
  alwaysOnTop: true,
  decorations: false,
  resizable: true,
  contentType: 'clock',
})

// 样式配置
const styleConfig = reactive({
  textColor: '#ffffff',
  backgroundColor: '#000000',
  fontSize: 48,
  countdownMinutes: 5,
  marqueeText: '欢迎来到直播间！关注主播不迷路～',
  marqueeSpeed: 50,
  gradientFrom: '#667eea',
  gradientTo: '#764ba2',
  gradientAngle: 135,
  imageUrl: 'https://picsum.photos/800/600',
  imageFit: 'cover',
  particleColor: '#ffffff',
  particleCount: 50,
  particleBgColor: '#1a1a2e',
})

// 已创建的窗口列表
const createdWindows = ref<WindowConfig[]>([])
const loading = ref(false)

// 快速预设
const presets = [
  { name: '时钟', type: 'clock', width: 350, height: 120, transparent: true },
  { name: '倒计时', type: 'countdown', width: 300, height: 150, transparent: true },
  { name: '滚动公告', type: 'marquee', width: 800, height: 60, transparent: true },
  { name: '风景图', type: 'image', width: 800, height: 600, transparent: false },
  { name: '粒子背景', type: 'particles', width: 800, height: 600, transparent: false },
  { name: '绿幕', type: 'solid', width: 800, height: 600, transparent: false },
  { name: '渐变背景', type: 'gradient', width: 800, height: 600, transparent: false },
]

function applyPreset(preset: (typeof presets)[0]) {
  config.title = preset.name
  config.contentType = preset.type
  config.width = preset.width
  config.height = preset.height
  config.transparent = preset.transparent
  config.decorations = false
}

// 构建 URL 参数
function buildParams(): string {
  const p = new URLSearchParams()
  p.set('type', config.contentType)
  p.set('color', styleConfig.textColor)
  p.set('bg', styleConfig.backgroundColor)
  p.set('size', String(styleConfig.fontSize))
  p.set('transparent', String(config.transparent))

  if (config.contentType === 'countdown') {
    p.set('minutes', String(styleConfig.countdownMinutes))
  } else if (config.contentType === 'marquee') {
    p.set('text', styleConfig.marqueeText)
    p.set('speed', String(styleConfig.marqueeSpeed))
  } else if (config.contentType === 'gradient') {
    p.set('from', styleConfig.gradientFrom)
    p.set('to', styleConfig.gradientTo)
    p.set('angle', String(styleConfig.gradientAngle))
  } else if (config.contentType === 'image') {
    p.set('url', styleConfig.imageUrl)
    p.set('fit', styleConfig.imageFit)
  } else if (config.contentType === 'particles') {
    p.set('pcolor', styleConfig.particleColor)
    p.set('pcount', String(styleConfig.particleCount))
    p.set('pbg', styleConfig.particleBgColor)
  }

  return p.toString()
}

async function createWindow() {
  loading.value = true
  try {
    const windowId = `screen_${Date.now()}`
    await invoke('create_screen_window', {
      label: windowId,
      title: config.title,
      width: config.width,
      height: config.height,
      transparent: config.transparent,
      alwaysOnTop: config.alwaysOnTop,
      decorations: config.decorations,
      resizable: config.resizable,
      backgroundColor: styleConfig.backgroundColor,
      extraParams: buildParams(),
    })
    createdWindows.value.push({ id: windowId, ...JSON.parse(JSON.stringify(config)) })
  } catch (e) {
    logger.error(`创建失败: ${e}`)
  } finally {
    loading.value = false
  }
}

async function closeWindow(id: string) {
  try {
    await invoke('close_screen_window', { label: id })
    createdWindows.value = createdWindows.value.filter((w) => w.id !== id)
  } catch (e) {
    logger.error(`关闭失败: ${e}`)
  }
}

async function closeAllWindows() {
  for (const w of [...createdWindows.value]) {
    await closeWindow(w.id)
  }
}
</script>

<template>
  <div class="p-4">
    <h1 class="text-2xl font-bold mb-4 flex items-center gap-2">
      <Icon icon="mdi:monitor-screenshot" class="text-primary" />
      OBS 投屏窗口
    </h1>

    <div class="flex gap-4">
      <!-- 左侧：配置区 -->
      <div class="flex-1 space-y-4">
        <!-- 快速预设 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-2">快速预设</h3>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="p in presets"
                :key="p.name"
                class="btn btn-xs btn-outline"
                @click="applyPreset(p)"
              >
                {{ p.name }}
              </button>
            </div>
          </div>
        </div>

        <!-- 内容类型 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-2">内容类型</h3>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="ct in contentTypes"
                :key="ct.value"
                class="btn btn-sm"
                :class="config.contentType === ct.value ? 'btn-primary' : 'btn-ghost'"
                @click="config.contentType = ct.value"
              >
                <Icon :icon="ct.icon" />
                {{ ct.label }}
              </button>
            </div>
          </div>
        </div>

        <!-- 样式配置 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-3">样式配置</h3>
            <div class="grid grid-cols-2 gap-3">
              <!-- 文字颜色 -->
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">文字颜色</span></label>
                <div class="flex gap-1">
                  <input v-model="styleConfig.textColor" type="color" class="w-8 h-8 rounded" />
                  <input
                    v-model="styleConfig.textColor"
                    type="text"
                    class="input input-bordered input-sm flex-1"
                  />
                </div>
              </div>

              <!-- 字体大小 -->
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">字体大小</span></label>
                <input
                  v-model.number="styleConfig.fontSize"
                  type="number"
                  class="input input-bordered input-sm"
                />
              </div>

              <!-- 背景颜色（非渐变） -->
              <div v-if="config.contentType !== 'gradient'" class="form-control">
                <label class="label py-1"><span class="label-text text-xs">背景颜色</span></label>
                <div class="flex gap-1">
                  <input
                    v-model="styleConfig.backgroundColor"
                    type="color"
                    class="w-8 h-8 rounded"
                  />
                  <input
                    v-model="styleConfig.backgroundColor"
                    type="text"
                    class="input input-bordered input-sm flex-1"
                  />
                </div>
              </div>

              <!-- 倒计时分钟 -->
              <div v-if="config.contentType === 'countdown'" class="form-control">
                <label class="label py-1"
                  ><span class="label-text text-xs">倒计时(分钟)</span></label
                >
                <input
                  v-model.number="styleConfig.countdownMinutes"
                  type="number"
                  min="1"
                  class="input input-bordered input-sm"
                />
              </div>

              <!-- 滚动文字 -->
              <template v-if="config.contentType === 'marquee'">
                <div class="form-control col-span-2">
                  <label class="label py-1"><span class="label-text text-xs">滚动文字</span></label>
                  <input
                    v-model="styleConfig.marqueeText"
                    type="text"
                    class="input input-bordered input-sm"
                  />
                </div>
                <div class="form-control">
                  <label class="label py-1"
                    ><span class="label-text text-xs"
                      >速度: {{ styleConfig.marqueeSpeed }}</span
                    ></label
                  >
                  <input
                    v-model.number="styleConfig.marqueeSpeed"
                    type="range"
                    min="10"
                    max="150"
                    class="range range-xs range-primary"
                  />
                </div>
              </template>

              <!-- 渐变配置 -->
              <template v-if="config.contentType === 'gradient'">
                <div class="form-control">
                  <label class="label py-1"><span class="label-text text-xs">起始颜色</span></label>
                  <div class="flex gap-1">
                    <input
                      v-model="styleConfig.gradientFrom"
                      type="color"
                      class="w-8 h-8 rounded"
                    />
                    <input
                      v-model="styleConfig.gradientFrom"
                      type="text"
                      class="input input-bordered input-sm flex-1"
                    />
                  </div>
                </div>
                <div class="form-control">
                  <label class="label py-1"><span class="label-text text-xs">结束颜色</span></label>
                  <div class="flex gap-1">
                    <input v-model="styleConfig.gradientTo" type="color" class="w-8 h-8 rounded" />
                    <input
                      v-model="styleConfig.gradientTo"
                      type="text"
                      class="input input-bordered input-sm flex-1"
                    />
                  </div>
                </div>
                <div class="form-control">
                  <label class="label py-1"
                    ><span class="label-text text-xs"
                      >角度: {{ styleConfig.gradientAngle }}°</span
                    ></label
                  >
                  <input
                    v-model.number="styleConfig.gradientAngle"
                    type="range"
                    min="0"
                    max="360"
                    class="range range-xs range-primary"
                  />
                </div>
              </template>

              <!-- 图片配置 -->
              <template v-if="config.contentType === 'image'">
                <div class="form-control col-span-2">
                  <label class="label py-1"><span class="label-text text-xs">图片链接</span></label>
                  <input
                    v-model="styleConfig.imageUrl"
                    type="text"
                    class="input input-bordered input-sm"
                    placeholder="https://..."
                  />
                </div>
                <div class="form-control">
                  <label class="label py-1"><span class="label-text text-xs">填充方式</span></label>
                  <select v-model="styleConfig.imageFit" class="select select-bordered select-sm">
                    <option value="cover">覆盖</option>
                    <option value="contain">包含</option>
                    <option value="fill">拉伸</option>
                  </select>
                </div>
              </template>

              <!-- 粒子配置 -->
              <template v-if="config.contentType === 'particles'">
                <div class="form-control">
                  <label class="label py-1"><span class="label-text text-xs">粒子颜色</span></label>
                  <div class="flex gap-1">
                    <input
                      v-model="styleConfig.particleColor"
                      type="color"
                      class="w-8 h-8 rounded"
                    />
                    <input
                      v-model="styleConfig.particleColor"
                      type="text"
                      class="input input-bordered input-sm flex-1"
                    />
                  </div>
                </div>
                <div class="form-control">
                  <label class="label py-1"><span class="label-text text-xs">背景颜色</span></label>
                  <div class="flex gap-1">
                    <input
                      v-model="styleConfig.particleBgColor"
                      type="color"
                      class="w-8 h-8 rounded"
                    />
                    <input
                      v-model="styleConfig.particleBgColor"
                      type="text"
                      class="input input-bordered input-sm flex-1"
                    />
                  </div>
                </div>
                <div class="form-control">
                  <label class="label py-1"
                    ><span class="label-text text-xs"
                      >数量: {{ styleConfig.particleCount }}</span
                    ></label
                  >
                  <input
                    v-model.number="styleConfig.particleCount"
                    type="range"
                    min="10"
                    max="200"
                    class="range range-xs range-primary"
                  />
                </div>
              </template>
            </div>
          </div>
        </div>

        <!-- 窗口配置 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-3">窗口配置</h3>
            <div class="grid grid-cols-2 gap-3">
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">标题</span></label>
                <input v-model="config.title" type="text" class="input input-bordered input-sm" />
              </div>
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">宽度</span></label>
                <input
                  v-model.number="config.width"
                  type="number"
                  class="input input-bordered input-sm"
                />
              </div>
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">高度</span></label>
                <input
                  v-model.number="config.height"
                  type="number"
                  class="input input-bordered input-sm"
                />
              </div>
              <div class="form-control">
                <label class="label py-1"><span class="label-text text-xs">选项</span></label>
                <div class="flex flex-wrap gap-3">
                  <label class="label cursor-pointer gap-1 p-0">
                    <input
                      v-model="config.transparent"
                      type="checkbox"
                      class="checkbox checkbox-xs checkbox-primary"
                    />
                    <span class="label-text text-xs">透明</span>
                  </label>
                  <label class="label cursor-pointer gap-1 p-0">
                    <input
                      v-model="config.alwaysOnTop"
                      type="checkbox"
                      class="checkbox checkbox-xs checkbox-primary"
                    />
                    <span class="label-text text-xs">置顶</span>
                  </label>
                  <label class="label cursor-pointer gap-1 p-0">
                    <input
                      v-model="config.decorations"
                      type="checkbox"
                      class="checkbox checkbox-xs checkbox-primary"
                    />
                    <span class="label-text text-xs">边框</span>
                  </label>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧：操作区 -->
      <div class="w-72 space-y-4">
        <!-- 创建/关闭按钮 -->
        <div class="flex gap-2">
          <button class="btn btn-primary btn-lg flex-1" :disabled="loading" @click="createWindow">
            <span v-if="loading" class="loading loading-spinner"></span>
            <Icon v-else icon="mdi:plus" class="text-xl" />
            创建窗口
          </button>
          <button
            v-if="createdWindows.length > 0"
            class="btn btn-error btn-lg"
            @click="closeAllWindows"
          >
            <Icon icon="mdi:close-circle" class="text-xl" />
            关闭全部
          </button>
        </div>

        <!-- 已创建窗口 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-2">已创建 ({{ createdWindows.length }})</h3>

            <div v-if="createdWindows.length === 0" class="text-center py-8 text-base-content/40">
              <Icon icon="mdi:window-restore" class="text-4xl mb-2" />
              <p class="text-sm">暂无窗口</p>
            </div>

            <div v-else class="space-y-2 max-h-96 overflow-y-auto">
              <div
                v-for="win in createdWindows"
                :key="win.id"
                class="flex items-center justify-between p-2 bg-base-300 rounded"
              >
                <div class="flex-1 min-w-0">
                  <p class="font-medium text-sm truncate">{{ win.title }}</p>
                  <p class="text-xs text-base-content/50">
                    {{ win.width }}×{{ win.height }}
                    <span v-if="win.transparent" class="ml-1">透明</span>
                  </p>
                </div>
                <button class="btn btn-ghost btn-xs text-error" @click="closeWindow(win.id)">
                  <Icon icon="mdi:close" />
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- 使用提示 -->
        <div class="card bg-base-200">
          <div class="card-body p-4">
            <h3 class="font-bold text-sm mb-2">使用提示</h3>
            <ul class="text-xs text-base-content/70 space-y-1">
              <li>• OBS 添加「窗口捕获」源</li>
              <li>• 选择创建的投屏窗口</li>
              <li>• 透明窗口需开启「允许透明」</li>
              <li>• 右键窗口项可关闭</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
