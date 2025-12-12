<script setup lang="ts">
/**
 * 直播间标题设置弹窗
 * 支持 AI 生成指定数量的直播间标题，配置持久化
 */
import { ref, watch, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { useLiveStore } from '../stores/live'
import {
  STORAGE_KEYS,
  API_ENDPOINTS,
  AI_PROMPTS,
  AI_REQUEST_DEFAULTS,
} from '@/config/constants'

interface Props {
  visible: boolean
}

interface Emits {
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const store = useLiveStore()

// 标题数量
const titleCount = ref(10)
// 生成的标题列表
const titles = ref<string[]>([])
// 是否正在生成
const isGenerating = ref(false)
// 提示词
const prompt = ref(AI_PROMPTS.TITLE_GENERATION)
// 是否显示提示词设置
const showPromptSettings = ref(false)
// 错误信息
const errorMsg = ref('')

// 加载保存的设置
function loadSettings() {
  try {
    const saved = localStorage.getItem(STORAGE_KEYS.TITLE_SETTINGS)
    if (saved) {
      const data = JSON.parse(saved)
      titleCount.value = data.titleCount || 5
      titles.value = data.titles || []
      prompt.value = data.prompt || AI_PROMPTS.TITLE_GENERATION
    }
  } catch {
    // 忽略
  }
}

// 保存设置
function saveSettings() {
  localStorage.setItem(
    STORAGE_KEYS.TITLE_SETTINGS,
    JSON.stringify({
      titleCount: titleCount.value,
      titles: titles.value,
      prompt: prompt.value,
    }),
  )
}

// 监听弹窗打开
watch(
  () => props.visible,
  (visible) => {
    if (visible) {
      loadSettings()
    }
  },
)

// AI 生成标题
async function generateTitles() {
  if (titleCount.value <= 0) return

  const apiKey = store.aiScriptSettings.apiKey
  const model = store.aiScriptSettings.model

  if (!apiKey) {
    errorMsg.value = '请先在 AI 设置中配置 API 秘钥'
    return
  }

  isGenerating.value = true
  errorMsg.value = ''

  try {
    const response = await fetch(API_ENDPOINTS.CHAT_COMPLETIONS, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        model: model,
        messages: [
          {
            role: 'user',
            content: `${prompt.value}\n\n请生成 ${titleCount.value} 个直播间标题，每行一个标题，不要序号，不要其他解释。`,
          },
        ],
        temperature: AI_REQUEST_DEFAULTS.TITLE_TEMPERATURE,
        max_tokens: AI_REQUEST_DEFAULTS.TITLE_MAX_TOKENS,
      }),
    })

    if (!response.ok) {
      throw new Error(`请求失败: ${response.status}`)
    }

    const data = await response.json()
    const content = data.choices?.[0]?.message?.content || ''

    // 解析标题（按行分割，过滤空行，去除序号和标点）
    const generatedTitles = content
      .split('\n')
      .map((line: string) => {
        // 去除序号（如 "1." "1、" "1:" 等）
        let cleaned = line.trim().replace(/^[\d]+[.、:：)\]】]\s*/, '')
        // 去除引号
        cleaned = cleaned.replace(/^["'"「『【]|["'"」』】]$/g, '')
        // 去除末尾标点
        cleaned = cleaned.replace(/[。！!？?，,；;：:]+$/g, '')
        return cleaned.trim()
      })
      .filter((line: string) => line.length > 0)
      .slice(0, titleCount.value)

    titles.value = generatedTitles
    saveSettings()
  } catch (error) {
    errorMsg.value = error instanceof Error ? error.message : '生成失败'
  } finally {
    isGenerating.value = false
  }
}

// 关闭弹窗
function close() {
  emit('close')
}

// 复制单个标题
async function copyTitle(title: string) {
  await navigator.clipboard.writeText(title)
}

// 复制全部标题
async function copyAllTitles() {
  await navigator.clipboard.writeText(titles.value.join('\n'))
}

// 删除单个标题
function removeTitle(index: number) {
  titles.value.splice(index, 1)
  saveSettings()
}

// 清空所有标题
function clearTitles() {
  titles.value = []
  saveSettings()
}

// 保存提示词
function savePrompt() {
  saveSettings()
  showPromptSettings.value = false
}

onMounted(() => {
  loadSettings()
})
</script>

<template>
  <dialog class="modal" :class="{ 'modal-open': visible }">
    <div class="modal-box w-11/12 max-w-2xl">
      <!-- 标题栏 -->
      <div class="flex items-center justify-between mb-4">
        <h3 class="font-bold text-lg">设置直播间标题</h3>
        <button class="btn btn-sm btn-circle btn-ghost" @click="close">
          <Icon icon="mdi:close" class="text-lg" />
        </button>
      </div>

      <!-- 生成设置 -->
      <div class="flex flex-wrap items-center gap-3 mb-4">
        <div class="flex items-center gap-2">
          <span class="text-sm">生成数量</span>
          <input
            v-model.number="titleCount"
            type="number"
            min="1"
            max="20"
            class="input input-bordered input-sm w-20 text-center"
          />
        </div>
        <button
          class="btn btn-primary btn-sm"
          :disabled="isGenerating || titleCount <= 0"
          @click="generateTitles"
        >
          <Icon v-if="isGenerating" icon="mdi:loading" class="animate-spin" />
          <Icon v-else icon="mdi:auto-fix" />
          AI 生成标题
        </button>
        <button class="btn btn-ghost btn-sm" @click="showPromptSettings = !showPromptSettings">
          <Icon icon="mdi:cog" />
          提示词设置
        </button>
        <button v-if="titles.length > 0" class="btn btn-ghost btn-sm" @click="copyAllTitles">
          <Icon icon="mdi:content-copy" />
          复制全部
        </button>
        <button v-if="titles.length > 0" class="btn btn-ghost btn-sm text-error" @click="clearTitles">
          <Icon icon="mdi:delete" />
          清空
        </button>
      </div>

      <!-- 提示词设置 -->
      <div v-if="showPromptSettings" class="mb-4 p-3 bg-base-200 rounded-lg">
        <label class="label">
          <span class="label-text">AI 提示词</span>
        </label>
        <textarea
          v-model="prompt"
          class="textarea textarea-bordered w-full h-24"
          placeholder="输入生成标题的提示词..."
        ></textarea>
        <div class="flex justify-end mt-2">
          <button class="btn btn-sm btn-primary" @click="savePrompt">
            <Icon icon="mdi:check" />
            保存
          </button>
        </div>
      </div>

      <!-- 错误提示 -->
      <div v-if="errorMsg" class="alert alert-error mb-4">
        <Icon icon="mdi:alert-circle" />
        <span>{{ errorMsg }}</span>
      </div>

      <!-- 标题列表 -->
      <div class="bg-base-200 rounded-lg p-4 min-h-48 max-h-96 overflow-y-auto">
        <div
          v-if="titles.length === 0"
          class="flex flex-col items-center justify-center h-40 text-base-content/50"
        >
          <Icon icon="mdi:text-box-outline" class="text-4xl mb-2" />
          <span>点击"AI 生成标题"按钮生成直播间标题</span>
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="(title, index) in titles"
            :key="index"
            class="flex items-center gap-2 bg-base-100 rounded-lg px-3 py-2 group"
          >
            <span class="text-base-content/50 text-sm w-6">{{ index + 1 }}.</span>
            <span class="flex-1" :class="{ 'text-error': title.length > 15 }">
              {{ title }}
              <span v-if="title.length > 15" class="text-xs ml-1">({{ title.length }}字，超出限制)</span>
            </span>
            <button
              class="btn btn-ghost btn-xs opacity-0 group-hover:opacity-100 transition-opacity"
              @click="copyTitle(title)"
            >
              <Icon icon="mdi:content-copy" />
            </button>
            <button
              class="btn btn-ghost btn-xs opacity-0 group-hover:opacity-100 transition-opacity text-error"
              @click="removeTitle(index)"
            >
              <Icon icon="mdi:close" />
            </button>
          </div>
        </div>
      </div>

      <!-- 底部操作 -->
      <div class="modal-action">
        <button class="btn" @click="close">关闭</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop" @click="close">
      <button>close</button>
    </form>
  </dialog>
</template>
