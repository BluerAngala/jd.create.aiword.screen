<script setup lang="ts">
import { open, save, message } from '@tauri-apps/plugin-dialog'
import { useLogger } from '@/core/composables'

const logger = useLogger('Demo')

// 打开文件对话框
async function openFile() {
  logger.info('打开文件选择对话框')
  const file = await open({
    multiple: false,
    filters: [
      { name: '图片', extensions: ['png', 'jpg', 'jpeg', 'gif'] },
      { name: '所有文件', extensions: ['*'] },
    ],
  })
  if (file) {
    logger.info(`选择的文件: ${file}`)
    await message(`你选择了: ${file}`, { title: '文件选择' })
  }
}

// 保存文件对话框
async function saveFile() {
  logger.info('打开保存对话框')
  const path = await save({
    defaultPath: 'untitled.txt',
    filters: [{ name: '文本文件', extensions: ['txt'] }],
  })
  if (path) {
    logger.info(`保存路径: ${path}`)
    await message(`保存路径: ${path}`, { title: '保存文件' })
  }
}

// 显示消息对话框
async function showMessage() {
  logger.info('显示消息对话框')
  await message('这是一个消息对话框示例', {
    title: '提示',
    kind: 'info',
  })
}
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold">功能演示</h2>

    <!-- 对话框示例 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">对话框示例</h3>
        <p class="text-base-content/70">展示 Tauri 原生对话框功能</p>

        <div class="flex gap-2 flex-wrap mt-4">
          <button class="btn btn-primary" @click="openFile">打开文件</button>
          <button class="btn btn-secondary" @click="saveFile">保存文件</button>
          <button class="btn btn-accent" @click="showMessage">消息提示</button>
        </div>
      </div>
    </div>

    <!-- DaisyUI 组件示例 -->
    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">DaisyUI 组件</h3>

        <div class="space-y-4 mt-4">
          <div class="flex gap-2 flex-wrap">
            <button class="btn btn-primary">Primary</button>
            <button class="btn btn-secondary">Secondary</button>
            <button class="btn btn-accent">Accent</button>
            <button class="btn btn-ghost">Ghost</button>
          </div>

          <div class="flex gap-2">
            <div class="badge badge-primary">Primary</div>
            <div class="badge badge-secondary">Secondary</div>
            <div class="badge badge-accent">Accent</div>
          </div>

          <progress class="progress progress-primary w-full" value="70" max="100"></progress>
        </div>
      </div>
    </div>
  </div>
</template>
