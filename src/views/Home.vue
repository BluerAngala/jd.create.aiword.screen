<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLogger } from '@/core/composables'

const logger = useLogger('Home')
const greetMsg = ref('')
const name = ref('')

async function greet() {
  logger.info(`调用 greet 命令，参数: ${name.value}`)
  try {
    greetMsg.value = await invoke('greet', { name: name.value })
    logger.info(`greet 返回: ${greetMsg.value}`)
  } catch (e) {
    logger.error(`greet 失败: ${e}`)
  }
}
</script>

<template>
  <div class="space-y-6">
    <h2 class="text-2xl font-bold">首页</h2>

    <div class="card bg-base-100 shadow-xl">
      <div class="card-body">
        <h3 class="card-title">欢迎使用模板</h3>

        <div class="form-control w-full">
          <label class="label">
            <span class="label-text">输入你的名字</span>
          </label>
          <input
            v-model="name"
            type="text"
            placeholder="请输入..."
            class="input input-bordered w-full"
            @keyup.enter="greet"
          />
        </div>

        <div class="card-actions justify-end mt-4">
          <button class="btn btn-primary" @click="greet">打招呼</button>
        </div>

        <div v-if="greetMsg" class="alert alert-success mt-4">
          <span>{{ greetMsg }}</span>
        </div>
      </div>
    </div>
  </div>
</template>
