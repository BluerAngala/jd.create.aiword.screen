<script setup lang="ts">
/**
 * AI 设置弹窗组件
 * 配置 SiliconFlow API 秘钥、模型、提示词
 */
import { ref, watch } from 'vue'
import { Icon } from '@iconify/vue'
import { DEFAULT_AI_MODEL, API_ENDPOINTS, SILICON_FLOW_REGISTER_URL } from '@/config/constants'

interface AISettings {
  model: string
  apiKey: string
  prompt: string
}

interface SiliconFlowModel {
  id: string
  object: string
}

interface Props {
  visible: boolean
  settings: AISettings
}

interface Emits {
  (e: 'update:visible', value: boolean): void
  (e: 'save', settings: AISettings): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localSettings = ref<AISettings>({ ...props.settings })
const models = ref<SiliconFlowModel[]>([])
const isLoadingModels = ref(false)
const modelError = ref('')

// 监听 settings 变化
watch(
  () => props.settings,
  (newSettings) => {
    localSettings.value = { ...newSettings }
  },
)

// 监听弹窗打开时加载模型
watch(
  () => props.visible,
  (visible) => {
    if (visible && localSettings.value.apiKey) {
      fetchModels()
    }
  },
)

// 监听 API Key 变化，自动获取模型
let debounceTimer: ReturnType<typeof setTimeout> | null = null
watch(
  () => localSettings.value.apiKey,
  (newKey) => {
    if (debounceTimer) clearTimeout(debounceTimer)
    if (newKey && newKey.length > 10) {
      debounceTimer = setTimeout(() => fetchModels(), 500)
    }
  },
)

// 获取模型列表
async function fetchModels() {
  if (!localSettings.value.apiKey) return

  isLoadingModels.value = true
  modelError.value = ''

  try {
    const response = await fetch(`${API_ENDPOINTS.MODELS}?type=text`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${localSettings.value.apiKey}`,
      },
    })

    if (!response.ok) {
      throw new Error(`请求失败: ${response.status}`)
    }

    const data = await response.json()
    models.value = data.data || []

    // 如果当前模型不在列表中，使用默认模型
    if (!localSettings.value.model || !models.value.find((m) => m.id === localSettings.value.model)) {
      localSettings.value.model = DEFAULT_AI_MODEL
    }
  } catch (error) {
    modelError.value = error instanceof Error ? error.message : '获取模型列表失败'
    models.value = []
  } finally {
    isLoadingModels.value = false
  }
}

function handleSave() {
  emit('save', { ...localSettings.value })
  emit('update:visible', false)
}

function handleClose() {
  localSettings.value = { ...props.settings }
  emit('update:visible', false)
}
</script>

<template>
  <dialog class="modal" :class="{ 'modal-open': visible }">
    <div class="modal-box max-w-md">
      <h3 class="font-bold text-lg flex items-center gap-2 mb-4">
        <Icon icon="mdi:robot-outline" class="text-xl" />
        AI 设置
      </h3>

      <div class="space-y-4">
        <!-- API 秘钥 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">SiliconFlow API 秘钥</span>
          </label>
          <input
            v-model="localSettings.apiKey"
            type="password"
            placeholder="请输入 API Key"
            class="input input-bordered w-full"
          />
          <label class="label">
            <span class="label-text-alt text-base-content/50">
              <a
                :href="SILICON_FLOW_REGISTER_URL"
                target="_blank"
                class="link link-primary"
              >
                点击获取 API 秘钥
              </a>
            </span>
          </label>
        </div>

        <!-- AI 模型 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">AI 模型</span>
            <span v-if="isLoadingModels" class="label-text-alt">
              <Icon icon="mdi:loading" class="animate-spin" />
              加载中...
            </span>
          </label>
          <select v-model="localSettings.model" class="select select-bordered w-full">
            <option v-if="models.length === 0" :value="DEFAULT_AI_MODEL">{{ DEFAULT_AI_MODEL }}</option>
            <option v-for="model in models" :key="model.id" :value="model.id">
              {{ model.id }}
            </option>
          </select>
          <label v-if="modelError" class="label">
            <span class="label-text-alt text-error">{{ modelError }}</span>
          </label>
        </div>

        <!-- 提示词 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">提示词</span>
          </label>
          <textarea
            v-model="localSettings.prompt"
            class="textarea textarea-bordered w-full h-32"
            placeholder="请输入 AI 生成话术的提示词..."
          ></textarea>
          <label class="label">
            <span class="label-text-alt text-base-content/50">用于指导 AI 生成直播话术的风格和内容</span>
          </label>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" @click="handleClose">取消</button>
        <button class="btn btn-primary" @click="handleSave">保存</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop" @click="handleClose">
      <button>关闭</button>
    </form>
  </dialog>
</template>
