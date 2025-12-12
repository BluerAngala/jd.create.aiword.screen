<script setup lang="ts">
/**
 * 设置弹窗组件
 * 提供登录信息配置、AI 配置选项
 */
import { ref, watch } from 'vue'
import { Icon } from '@iconify/vue'
import type { AppSettings } from '../types'

interface Props {
  visible: boolean
  settings: AppSettings
}

interface Emits {
  (e: 'update:visible', value: boolean): void
  (e: 'save', settings: AppSettings): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 本地表单数据
const localSettings = ref<AppSettings>({ ...props.settings })

// 监听 props 变化，同步到本地
watch(
  () => props.settings,
  (newVal) => {
    localSettings.value = { ...newVal }
  },
  { deep: true }
)

// 关闭弹窗
function closeModal() {
  emit('update:visible', false)
}

// 保存设置
function handleSave() {
  emit('save', { ...localSettings.value })
  closeModal()
}
</script>

<template>
  <dialog class="modal" :class="{ 'modal-open': visible }">
    <div class="modal-box w-11/12 max-w-lg">
      <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" @click="closeModal">
        <Icon icon="mdi:close" class="text-lg" />
      </button>

      <h3 class="font-bold text-lg mb-4">
        <Icon icon="mdi:cog" class="inline mr-2" />
        软件设置
      </h3>

      <div class="space-y-6">
        <!-- 登录信息配置 -->
        <div class="form-control">
          <label class="label">
            <span class="label-text font-medium">API 密钥</span>
          </label>
          <input
            v-model="localSettings.loginInfo.apiKey"
            type="password"
            placeholder="请输入 API 密钥"
            class="input input-bordered w-full"
          />
        </div>

        <!-- AI 配置 -->
        <div class="space-y-4">
          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-4">
              <input
                v-model="localSettings.aiConfig.enabled"
                type="checkbox"
                class="toggle toggle-primary"
              />
              <span class="label-text font-medium">启用 AI 功能</span>
            </label>
          </div>

          <div v-if="localSettings.aiConfig.enabled" class="space-y-4 pl-4">
            <div class="form-control">
              <label class="label">
                <span class="label-text">AI 模型</span>
              </label>
              <input
                v-model="localSettings.aiConfig.model"
                type="text"
                placeholder="例如：gpt-4"
                class="input input-bordered w-full"
              />
            </div>

            <div class="form-control">
              <label class="label">
                <span class="label-text">API 端点</span>
              </label>
              <input
                v-model="localSettings.aiConfig.apiEndpoint"
                type="text"
                placeholder="例如：https://api.openai.com/v1"
                class="input input-bordered w-full"
              />
            </div>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button class="btn btn-ghost" @click="closeModal">取消</button>
        <button class="btn btn-primary" @click="handleSave">
          <Icon icon="mdi:content-save" class="mr-1" />
          保存
        </button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop">
      <button @click="closeModal">关闭</button>
    </form>
  </dialog>
</template>
