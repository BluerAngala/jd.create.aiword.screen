<script setup lang="ts">
/**
 * 图片配置组件 - 设置宽度、高度、边框图，支持投屏功能和方案管理
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useLiveStore } from '../stores/live'
import { useToast } from '@/core/composables'
import type { ImageSettings } from '../types'

interface Props {
  config: ImageSettings
  expanded?: boolean
  canScreen?: boolean
  isScreening?: boolean
}

interface Emits {
  (e: 'update', config: ImageSettings): void
  (e: 'toggle'): void
  (e: 'startScreen'): void
  (e: 'stopScreen'): void
}

const props = withDefaults(defineProps<Props>(), {
  canScreen: false,
  isScreening: false,
})
const emit = defineEmits<Emits>()

const store = useLiveStore()
const toast = useToast()

// 方案管理
const showPresetInput = ref(false)
const newPresetName = ref('')

// 边框图文件名（从路径中提取）
const borderImageName = computed(() => {
  if (!props.config.borderImage) return ''
  const parts = props.config.borderImage.replace(/\\/g, '/').split('/')
  return parts[parts.length - 1] || ''
})

// 当前选中的方案
const currentPreset = computed(() =>
  store.screenPresets.find((p) => p.id === store.selectedPresetId)
)

function updateConfig<K extends keyof ImageSettings>(key: K, value: ImageSettings[K]) {
  emit('update', { ...props.config, [key]: Math.max(1, value as number) })
}

// 保存新方案
async function saveNewPreset() {
  if (!newPresetName.value.trim()) return
  await store.addScreenPreset(newPresetName.value.trim())
  newPresetName.value = ''
  showPresetInput.value = false
  toast.success('方案已保存')
}

// 保存当前方案
async function saveCurrentPreset() {
  if (!store.selectedPresetId) return
  const success = await store.updateScreenPreset(store.selectedPresetId)
  if (success) {
    toast.success('方案已保存')
  }
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div
      class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer"
      @click="emit('toggle')"
    >
      <Icon icon="mdi:numeric-3-circle" class="text-xl" />
      <span class="text-sm font-medium">图片设置</span>
      <button
        class="btn btn-xs ml-auto mr-4"
        :class="isScreening ? 'btn-error' : 'btn-success'"
        :disabled="!canScreen && !isScreening"
        @click.stop="isScreening ? emit('stopScreen') : emit('startScreen')"
      >
        <Icon :icon="isScreening ? 'mdi:cast-off' : 'mdi:cast'" class="text-sm" />
        {{ isScreening ? '停止投屏' : '开启投屏' }}
      </button>
    </div>
    <div v-if="expanded" class="px-3 pb-3 space-y-3">
      <!-- 方案管理 -->
      <div class="flex items-center gap-2 flex-wrap">
        <span class="text-sm">方案</span>
        <select
          :value="store.selectedPresetId"
          class="select select-bordered select-sm w-28"
          @change="store.loadScreenPreset(($event.target as HTMLSelectElement).value || null)"
        >
          <option value="">默认</option>
          <option v-for="preset in store.screenPresets" :key="preset.id" :value="preset.id">
            {{ preset.name }}
          </option>
        </select>
        <!-- 已选方案：保存 + 删除 -->
        <template v-if="currentPreset">
          <button class="btn btn-xs btn-outline" @click="saveCurrentPreset">
            <Icon icon="mdi:content-save" class="text-sm" />
            保存
          </button>
          <button class="btn btn-xs btn-ghost text-error" @click="store.deleteScreenPreset(store.selectedPresetId!)">
            <Icon icon="mdi:delete" />
            删除
          </button>
        </template>
        <!-- 默认：新建方案 -->
        <template v-else>
          <div v-if="showPresetInput" class="flex items-center gap-1">
            <input
              v-model="newPresetName"
              type="text"
              class="input input-bordered input-xs w-20"
              placeholder="方案名"
              @keyup.enter="saveNewPreset"
            />
            <button class="btn btn-xs btn-primary" @click="saveNewPreset">
              <Icon icon="mdi:check" />
            </button>
            <button class="btn btn-xs btn-ghost" @click="showPresetInput = false">
              <Icon icon="mdi:close" />
            </button>
          </div>
          <button v-else class="btn btn-xs btn-outline" @click="showPresetInput = true">
            <Icon icon="mdi:plus" />
            新建方案
          </button>
        </template>
      </div>

      <div class="divider my-1"></div>

      <!-- 尺寸设置 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <span class="text-sm">宽度</span>
          <input
            type="number"
            :value="config.width"
            min="1"
            class="input input-bordered input-sm w-24 text-center"
            @input="updateConfig('width', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
        <div class="flex items-center gap-2">
          <span class="text-sm">高度</span>
          <input
            type="number"
            :value="config.height"
            min="1"
            class="input input-bordered input-sm w-24 text-center"
            @input="updateConfig('height', Number(($event.target as HTMLInputElement).value))"
          />
        </div>
      </div>
      <!-- 边框图片 -->
      <div class="flex items-center gap-3">
        <span class="text-sm">边框背景</span>
        <div class="flex items-center gap-2 flex-1">
          <button
            class="btn btn-sm btn-outline"
            @click="
              async () => {
                const selected = await open({
                  multiple: false,
                  filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
                })
                if (selected) emit('update', { ...config, borderImage: selected as string })
              }
            "
          >
            <Icon icon="mdi:image-plus" class="text-sm" />
            选择
          </button>
          <span v-if="borderImageName" class="text-xs text-base-content/70 truncate max-w-32">
            {{ borderImageName }}
          </span>
          <span v-else class="text-xs text-base-content/50">未设置</span>
          <button
            v-if="config.borderImage"
            class="btn btn-xs btn-ghost text-error"
            @click="emit('update', { ...config, borderImage: undefined })"
          >
            <Icon icon="mdi:close" />
          </button>
        </div>
      </div>

      <div class="divider my-1"></div>

      <!-- 提示 -->
      <div class="bg-base-200 rounded-lg p-3 text-xs text-base-content/70 space-y-1">
        <div class="flex items-center gap-1.5">
          <Icon icon="mdi:lightbulb-outline" class="text-warning" />
          <span>拖动投屏窗口到目标位置，点击「保存」记住位置</span>
        </div>
        <div class="flex items-center gap-1.5">
          <Icon icon="mdi:lightbulb-outline" class="text-warning" />
          <span>左侧 AI 话术的上下一条按钮会自动切换商品图片</span>
        </div>
      </div>
    </div>
  </div>
</template>
