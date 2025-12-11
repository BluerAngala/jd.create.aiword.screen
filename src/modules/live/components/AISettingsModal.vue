<script setup lang="ts">
/**
 * AI 设置弹窗组件
 * 配置 AI 模型、秘钥、提示词
 */
import { ref, watch } from 'vue'
import { Icon } from '@iconify/vue'

interface AISettings {
  model: string
  apiKey: string
  prompt: string
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

// 监听 settings 变化
watch(
  () => props.settings,
  (newSettings) => {
    localSettings.value = { ...newSettings }
  },
)

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
        AI 话术设置
      </h3>

      <div class="space-y-4">
        <!-- AI 模型 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">AI 模型</span>
          </label>
          <select v-model="localSettings.model" class="select select-bordered w-full">
            <option value="gpt-3.5-turbo">GPT-3.5 Turbo</option>
            <option value="gpt-4">GPT-4</option>
            <option value="gpt-4-turbo">GPT-4 Turbo</option>
            <option value="claude-3-sonnet">Claude 3 Sonnet</option>
            <option value="claude-3-opus">Claude 3 Opus</option>
            <option value="qwen-turbo">通义千问 Turbo</option>
            <option value="qwen-plus">通义千问 Plus</option>
          </select>
        </div>

        <!-- API 秘钥 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text">API 秘钥</span>
          </label>
          <input
            v-model="localSettings.apiKey"
            type="password"
            placeholder="请输入 API Key"
            class="input input-bordered w-full"
          />
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
