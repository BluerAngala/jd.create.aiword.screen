<script setup lang="ts">
/**
 * 商品配置组件 - 支持 xlsx 文件导入
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import * as XLSX from 'xlsx'
import type { ProductItem, ProductFileResult } from '../types'
import { useLiveStore } from '../stores/live'

interface Props {
  products: ProductItem[]
  expanded?: boolean
}

interface Emits {
  (e: 'update', products: ProductItem[]): void
  (e: 'import', file: File): void
  (e: 'generate-titles', productId: string): void
  (e: 'toggle'): void
}

const props = withDefaults(defineProps<Props>(), {
  expanded: false,
})
const emit = defineEmits<Emits>()

const liveStore = useLiveStore()
const fileInput = ref<HTMLInputElement | null>(null)
const expandedProductId = ref<string | null>(null)
const parseResult = ref<ProductFileResult | null>(null)

// 计算去重后的商品数量
const uniqueProductCount = computed(() => {
  return parseResult.value?.uniqueCount ?? props.products.length
})

function triggerFileSelect() {
  fileInput.value?.click()
}

/**
 * 解析 xlsx 文件
 */
async function parseXlsxFile(file: File): Promise<ProductFileResult> {
  return new Promise((resolve) => {
    const reader = new FileReader()

    reader.onload = (e) => {
      try {
        const data = new Uint8Array(e.target?.result as ArrayBuffer)
        const workbook = XLSX.read(data, { type: 'array' })
        const firstSheetName = workbook.SheetNames[0]
        if (!firstSheetName) {
          resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: '文件为空' })
          return
        }
        const worksheet = workbook.Sheets[firstSheetName]
        if (!worksheet) {
          resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: '工作表为空' })
          return
        }
        const jsonData = XLSX.utils.sheet_to_json<unknown[]>(worksheet, { header: 1 })

        if (jsonData.length === 0) {
          resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: '文件为空' })
          return
        }

        const headers = jsonData[0] as unknown[]
        if (!headers || headers.length === 0 || String(headers[0]).toUpperCase() !== 'ID') {
          resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: '第一列标题必须是 ID' })
          return
        }

        const allIds: string[] = []
        for (let i = 1; i < jsonData.length; i++) {
          const row = jsonData[i] as unknown[]
          if (row && row[0] !== undefined && row[0] !== null && row[0] !== '') {
            allIds.push(String(row[0]).trim())
          }
        }

        const uniqueIds = [...new Set(allIds.filter((id) => id.length > 0))]
        resolve({ success: true, productIds: uniqueIds, totalCount: allIds.length, uniqueCount: uniqueIds.length })
      } catch (error) {
        resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: `解析失败: ${error instanceof Error ? error.message : '未知错误'}` })
      }
    }

    reader.onerror = () => {
      resolve({ success: false, productIds: [], totalCount: 0, uniqueCount: 0, error: '文件读取失败' })
    }

    reader.readAsArrayBuffer(file)
  })
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  if (!file.name.endsWith('.xlsx') && !file.name.endsWith('.xls')) {
    liveStore.addLog('error', `文件格式错误: 仅支持 xlsx 格式`)
    target.value = ''
    return
  }

  liveStore.addLog('info', `开始解析文件: ${file.name}`)
  const result = await parseXlsxFile(file)
  parseResult.value = result

  if (!result.success) {
    liveStore.addLog('error', `解析失败: ${result.error}`)
    target.value = ''
    return
  }

  liveStore.addLog('success', `解析成功: 共 ${result.totalCount} 条数据，去重后 ${result.uniqueCount} 条`)
  const products: ProductItem[] = result.productIds.map((id) => ({ id, name: `商品 ${id}`, quantity: 1, titles: [''] }))
  emit('update', products)
  target.value = ''
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
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer" @click="emit('toggle')">
      <Icon icon="mdi:package-variant" class="text-lg" />
      <span class="text-sm font-medium flex-1">商品配置</span>
      <span v-if="uniqueProductCount > 0" class="badge badge-primary badge-sm">{{ uniqueProductCount }}</span>
      <button class="btn btn-primary btn-xs" @click.stop="triggerFileSelect">
        <Icon icon="mdi:upload" />
        导入
      </button>
      <input ref="fileInput" type="file" accept=".xlsx,.xls" class="hidden" @change="handleFileChange" />
    </div>
    <div v-if="expanded" class="px-3 pb-2">
      <div v-if="products.length === 0" class="text-center py-2 text-base-content/60 text-xs">
        暂无商品，请导入 xlsx 商品文件（第一列标题必须是 ID）
      </div>
      <div v-else class="space-y-1 max-h-40 overflow-y-auto">
        <div v-for="product in products" :key="product.id" class="border border-base-300 rounded">
          <div class="flex items-center gap-2 p-2 cursor-pointer text-sm" @click="toggleProduct(product.id)">
            <Icon icon="mdi:chevron-right" :class="{ 'rotate-90': expandedProductId === product.id }" />
            <span class="flex-1 truncate">{{ product.id }}</span>
            <input type="number" :value="product.quantity" min="1" class="input input-bordered input-xs w-14 text-center" @click.stop @input="updateQuantity(product.id, Number(($event.target as HTMLInputElement).value))" />
          </div>
          <div v-if="expandedProductId === product.id" class="p-2 pt-0 space-y-1">
            <div class="flex items-center justify-between">
              <span class="text-xs">标题</span>
              <div class="flex gap-1">
                <button class="btn btn-ghost btn-xs" @click="handleGenerateTitles(product.id)"><Icon icon="mdi:robot" /></button>
                <button class="btn btn-ghost btn-xs" @click="addTitle(product.id)"><Icon icon="mdi:plus" /></button>
              </div>
            </div>
            <div v-for="(title, index) in product.titles" :key="index" class="flex gap-1">
              <input type="text" :value="title" class="input input-bordered input-xs flex-1" @input="updateTitle(product.id, index, ($event.target as HTMLInputElement).value)" />
              <button v-if="product.titles.length > 1" class="btn btn-ghost btn-xs text-error" @click="removeTitle(product.id, index)"><Icon icon="mdi:close" /></button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
