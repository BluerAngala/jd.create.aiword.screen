<script setup lang="ts">
/**
 * 商品配置组件 - 支持折叠
 */
import { ref } from 'vue'
import { Icon } from '@iconify/vue'
import type { ProductItem } from '../types'

interface Props {
  products: ProductItem[]
  accordionName?: string
}

interface Emits {
  (e: 'update', products: ProductItem[]): void
  (e: 'import', file: File): void
  (e: 'generate-titles', productId: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const fileInput = ref<HTMLInputElement | null>(null)
const expandedProductId = ref<string | null>(null)

function triggerFileSelect() {
  fileInput.value?.click()
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    emit('import', file)
    target.value = ''
  }
}

function updateQuantity(id: string, quantity: number) {
  const updated = props.products.map((p) => (p.id === id ? { ...p, quantity: Math.max(1, quantity) } : p))
  emit('update', updated)
}

function updateTitle(id: string, index: number, value: string) {
  const updated = props.products.map((p) => {
    if (p.id === id) {
      const titles = [...p.titles]
      titles[index] = value
      return { ...p, titles }
    }
    return p
  })
  emit('update', updated)
}

function addTitle(id: string) {
  const updated = props.products.map((p) => (p.id === id ? { ...p, titles: [...p.titles, ''] } : p))
  emit('update', updated)
}

function removeTitle(id: string, index: number) {
  const updated = props.products.map((p) => {
    if (p.id === id && p.titles.length > 1) {
      return { ...p, titles: p.titles.filter((_, i) => i !== index) }
    }
    return p
  })
  emit('update', updated)
}

function toggleProduct(id: string) {
  expandedProductId.value = expandedProductId.value === id ? null : id
}

function handleGenerateTitles(id: string) {
  emit('generate-titles', id)
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm">
    <input type="radio" :name="accordionName" />
    <div class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2">
      <Icon icon="mdi:package-variant" class="text-lg" />
      <span class="text-sm font-medium flex-1">商品配置</span>
      <button class="btn btn-primary btn-xs" @click.stop="triggerFileSelect">
        <Icon icon="mdi:upload" />
        导入
      </button>
      <input ref="fileInput" type="file" accept=".json,.csv,.xlsx" class="hidden" @change="handleFileChange" />
    </div>
    <div class="collapse-content px-3 pb-2">
      <div v-if="products.length === 0" class="text-center py-2 text-base-content/60 text-xs">
        暂无商品，请导入商品文件
      </div>
      <div v-else class="space-y-1 max-h-40 overflow-y-auto">
        <div v-for="product in products" :key="product.id" class="border border-base-300 rounded">
          <div class="flex items-center gap-2 p-2 cursor-pointer text-sm" @click="toggleProduct(product.id)">
            <Icon icon="mdi:chevron-right" :class="{ 'rotate-90': expandedProductId === product.id }" />
            <span class="flex-1 truncate">{{ product.name }}</span>
            <input
              type="number"
              :value="product.quantity"
              min="1"
              class="input input-bordered input-xs w-14 text-center"
              @click.stop
              @input="updateQuantity(product.id, Number(($event.target as HTMLInputElement).value))"
            />
          </div>
          <div v-if="expandedProductId === product.id" class="p-2 pt-0 space-y-1">
            <div class="flex items-center justify-between">
              <span class="text-xs">标题</span>
              <div class="flex gap-1">
                <button class="btn btn-ghost btn-xs" @click="handleGenerateTitles(product.id)">
                  <Icon icon="mdi:robot" />
                </button>
                <button class="btn btn-ghost btn-xs" @click="addTitle(product.id)">
                  <Icon icon="mdi:plus" />
                </button>
              </div>
            </div>
            <div v-for="(title, index) in product.titles" :key="index" class="flex gap-1">
              <input
                type="text"
                :value="title"
                class="input input-bordered input-xs flex-1"
                @input="updateTitle(product.id, index, ($event.target as HTMLInputElement).value)"
              />
              <button v-if="product.titles.length > 1" class="btn btn-ghost btn-xs text-error" @click="removeTitle(product.id, index)">
                <Icon icon="mdi:close" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
