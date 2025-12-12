<script setup lang="ts">
/**
 * 商品配置组件 - 支持多个 xlsx 文件导入、拖动排序
 */
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'
import * as XLSX from 'xlsx'
import { openPath } from '@tauri-apps/plugin-opener'
import { save } from '@tauri-apps/plugin-dialog'
import { desktopDir } from '@tauri-apps/api/path'
import { writeFile } from '@tauri-apps/plugin-fs'
import type { ProductFile } from '../types'
import { useLiveStore } from '../stores/live'
import { useToast } from '@/core/composables'

interface Props {
  expanded?: boolean
}

interface Emits {
  (e: 'toggle'): void
}

withDefaults(defineProps<Props>(), {
  expanded: false,
})
const emit = defineEmits<Emits>()

const liveStore = useLiveStore()
const toast = useToast()
const fileInput = ref<HTMLInputElement | null>(null)

// 商品文件列表（从 store 获取，已持久化）
const productFiles = computed(() => liveStore.productFiles)

function triggerFileSelect() {
  fileInput.value?.click()
}

/**
 * 生成模板 Excel 文件
 */
function generateTemplateWorkbook(): Uint8Array {
  const workbook = XLSX.utils.book_new()
  const data = [['ID'], ['10001234567'], ['10001234568'], ['10001234569']]
  const worksheet = XLSX.utils.aoa_to_sheet(data)
  worksheet['!cols'] = [{ wch: 15 }]
  XLSX.utils.book_append_sheet(workbook, worksheet, '商品列表')
  return new Uint8Array(XLSX.write(workbook, { type: 'array', bookType: 'xlsx' }))
}

/**
 * 下载模板文件
 */
async function downloadTemplate() {
  try {
    // 获取桌面路径作为默认保存位置
    const desktop = await desktopDir()

    // 打开保存对话框
    const savePath = await save({
      defaultPath: `${desktop}/商品模板.xlsx`,
      filters: [{ name: 'Excel 文件', extensions: ['xlsx'] }],
    })

    if (!savePath) {
      return // 用户取消
    }

    // 生成模板文件并写入
    const templateData = generateTemplateWorkbook()
    await writeFile(savePath, templateData)

    liveStore.addLog('success', `模板已保存到: ${savePath}`)

    // 用系统默认程序打开文件
    await openPath(savePath)
  } catch (error) {
    liveStore.addLog(
      'error',
      `下载模板失败: ${error instanceof Error ? error.message : '未知错误'}`
    )
  }
}

/**
 * 解析 xlsx 文件
 */
async function parseXlsxFile(
  file: File
): Promise<{
  success: boolean
  productIds: string[]
  totalCount: number
  uniqueCount: number
  error?: string
}> {
  return new Promise((resolve) => {
    const reader = new FileReader()

    reader.onload = (e) => {
      try {
        const data = new Uint8Array(e.target?.result as ArrayBuffer)
        // 读取完整工作簿
        const workbook = XLSX.read(data, { type: 'array' })
        const firstSheetName = workbook.SheetNames[0]
        if (!firstSheetName) {
          resolve({
            success: false,
            productIds: [],
            totalCount: 0,
            uniqueCount: 0,
            error: '文件为空',
          })
          return
        }
        const worksheet = workbook.Sheets[firstSheetName]
        if (!worksheet) {
          resolve({
            success: false,
            productIds: [],
            totalCount: 0,
            uniqueCount: 0,
            error: '工作表为空',
          })
          return
        }
        const jsonData = XLSX.utils.sheet_to_json<unknown[]>(worksheet, { header: 1 })

        // 调试：输出解析结果
        console.log('[xlsx解析] 工作表范围:', worksheet['!ref'])
        console.log('[xlsx解析] JSON行数:', jsonData.length)

        if (jsonData.length === 0) {
          resolve({
            success: false,
            productIds: [],
            totalCount: 0,
            uniqueCount: 0,
            error: '文件为空',
          })
          return
        }

        // 检查第一列标题是否为 ID
        const headers = jsonData[0] as unknown[]
        if (!headers || headers.length === 0 || String(headers[0]).toUpperCase() !== 'ID') {
          resolve({
            success: false,
            productIds: [],
            totalCount: 0,
            uniqueCount: 0,
            error: '第一列标题必须是 ID',
          })
          return
        }

        // 提取所有 ID（去除空行）
        const allIds: string[] = []
        for (let i = 1; i < jsonData.length; i++) {
          const row = jsonData[i] as unknown[]
          if (row && row[0] !== undefined && row[0] !== null && row[0] !== '') {
            const id = String(row[0]).trim()
            if (id.length > 0) {
              allIds.push(id)
            }
          }
        }

        // 去重
        const uniqueIds = [...new Set(allIds)]
        resolve({
          success: true,
          productIds: uniqueIds,
          totalCount: allIds.length,
          uniqueCount: uniqueIds.length,
        })
      } catch (error) {
        resolve({
          success: false,
          productIds: [],
          totalCount: 0,
          uniqueCount: 0,
          error: `解析失败: ${error instanceof Error ? error.message : '未知错误'}`,
        })
      }
    }

    reader.onerror = () => {
      resolve({
        success: false,
        productIds: [],
        totalCount: 0,
        uniqueCount: 0,
        error: '文件读取失败',
      })
    }

    reader.readAsArrayBuffer(file)
  })
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return

  for (const file of Array.from(files)) {
    // 检查文件格式
    if (!file.name.endsWith('.xlsx') && !file.name.endsWith('.xls')) {
      liveStore.addLog('error', `文件格式错误: ${file.name}，仅支持 xlsx 格式`)
      continue
    }

    // 检查是否已存在同名文件
    if (productFiles.value.some((f) => f.name === file.name)) {
      liveStore.addLog('warn', `文件已存在: ${file.name}`)
      continue
    }

    liveStore.addLog('info', `开始解析文件: ${file.name}`)
    const result = await parseXlsxFile(file)

    if (!result.success) {
      liveStore.addLog('error', `${file.name} 解析失败: ${result.error}`)
      toast.error(`${file.name} 导入失败: ${result.error}`)
      continue
    }

    // 添加到文件列表（通过 store 持久化）
    const productFile: ProductFile = {
      id: `${Date.now()}-${Math.random().toString(36).slice(2, 9)}`,
      name: file.name,
      productIds: result.productIds,
      totalCount: result.totalCount,
      uniqueCount: result.uniqueCount,
      useCount: 999, // 默认每场直播使用 999 条
    }
    liveStore.addProductFile(productFile)
    liveStore.addLog(
      'success',
      `${file.name}: 共 ${result.totalCount} 条，去重后 ${result.uniqueCount} 条`
    )
  }

  // 清空 input
  target.value = ''
}

function removeFile(id: string) {
  const file = productFiles.value.find((f) => f.id === id)
  if (file) {
    liveStore.removeProductFile(id)
    liveStore.addLog('info', `已移除文件: ${file.name}`)
  }
}

// 上下移动排序
function moveUp(index: number) {
  if (index <= 0) return
  const files = [...productFiles.value]
  const temp = files[index]!
  files[index] = files[index - 1]!
  files[index - 1] = temp
  liveStore.updateProductFiles(files)
}

function moveDown(index: number) {
  if (index >= productFiles.value.length - 1) return
  const files = [...productFiles.value]
  const temp = files[index]!
  files[index] = files[index + 1]!
  files[index + 1] = temp
  liveStore.updateProductFiles(files)
}
</script>

<template>
  <div class="collapse collapse-arrow bg-base-100 shadow-sm" :class="{ 'collapse-open': expanded }">
    <div
      class="collapse-title py-2 px-3 pr-10 min-h-0 flex items-center gap-2 cursor-pointer"
      @click="emit('toggle')"
    >
      <Icon icon="mdi:package-variant" class="text-lg" />
      <span class="text-sm font-medium flex-1">商品配置</span>
    </div>

    <div v-if="expanded" class="px-3 pb-2">
      <!-- 操作按钮 -->
      <div class="flex items-center gap-2 mb-2">
        <span v-if="productFiles.length > 0" class="text-xs text-base-content/60">
          共 {{ productFiles.length }} 个文件
        </span>
        <span class="flex-1"></span>
        <button class="btn btn-ghost btn-sm" @click="downloadTemplate">
          <Icon icon="mdi:download" />
          下载模板
        </button>
        <button class="btn btn-primary btn-sm" @click="triggerFileSelect">
          <Icon icon="mdi:upload" />
          导入文件
        </button>
        <input
          ref="fileInput"
          type="file"
          accept=".xlsx,.xls"
          multiple
          class="hidden"
          @change="handleFileChange"
        />
      </div>

      <!-- 无文件提示 -->
      <div v-if="productFiles.length === 0" class="text-center py-3 text-base-content/60 text-xs">
        暂无商品，请导入 xlsx 商品文件（第一列标题必须是 ID）
      </div>

      <!-- 文件列表 -->
      <div v-else class="space-y-2 max-h-64 overflow-y-auto">
        <div
          v-for="(file, index) in productFiles"
          :key="file.id"
          class="p-2 bg-base-200 rounded transition-all"
        >
          <!-- 第一行：文件信息 -->
          <div class="flex items-center gap-2">
            <!-- 序号 -->
            <span class="badge badge-ghost badge-sm w-6">{{ index + 1 }}</span>

            <!-- 文件名 -->
            <span class="flex-1 text-sm truncate" :title="file.name">{{ file.name }}</span>

            <!-- 上移按钮 -->
            <button
              class="btn btn-ghost btn-xs"
              :class="{ 'btn-disabled opacity-30': index === 0 }"
              :disabled="index === 0"
              @click="moveUp(index)"
            >
              <Icon icon="mdi:arrow-up" /> 上移
            </button>

            <!-- 下移按钮 -->
            <button
              class="btn btn-ghost btn-xs"
              :class="{ 'btn-disabled opacity-30': index === productFiles.length - 1 }"
              :disabled="index === productFiles.length - 1"
              @click="moveDown(index)"
            >
              <Icon icon="mdi:arrow-down" /> 下移
            </button>
          </div>

          <!-- 第二行：数量设置 -->
          <div class="flex items-center justify-between mt-2 ml-8 text-xs">
            <div class="flex items-center gap-4">
              <!-- 有效商品数量 -->
              <div class="flex items-center gap-1 text-base-content/70">
                <span>有效商品：</span>
                <span class="font-medium">{{ file.uniqueCount }} 条</span>
              </div>

              <!-- 每场使用数量 -->
              <div class="flex items-center gap-1">
                <span class="text-base-content/70">每场添加：</span>
                <input
                  :value="file.useCount"
                  type="number"
                  min="1"
                  :max="file.uniqueCount"
                  class="input input-bordered input-xs w-16 text-center"
                  @click.stop
                  @change="
                    (e) =>
                      liveStore.updateProductFileUseCount(
                        file.id,
                        Number((e.target as HTMLInputElement).value)
                      )
                  "
                />
                <span class="text-base-content/70">条</span>
              </div>
            </div>

            <!-- 删除按钮 -->
            <button class="btn btn-ghost btn-xs text-error" @click="removeFile(file.id)">
              <Icon icon="mdi:delete" /> 删除
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
