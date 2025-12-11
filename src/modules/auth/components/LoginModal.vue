<script setup lang="ts">
import { ref } from 'vue'
import { useAuthStore } from '../stores/auth'
import { Icon } from '@iconify/vue'
import { openUrl } from '@tauri-apps/plugin-opener'

const authStore = useAuthStore()

const cardCode = ref('')
const userId = ref('')
const showPassword = ref(false)

// 充值续费链接
const RECHARGE_URL = 'https://env-00jxu65bfie3-static.normal.cloudstatic.cn/admin/index.html#/'

async function handleLogin() {
  if (!cardCode.value.trim() || !userId.value.trim()) {
    return
  }
  await authStore.login(cardCode.value.trim(), userId.value.trim())
}

function openRecharge() {
  openUrl(RECHARGE_URL)
}
</script>

<template>
  <!-- 登录弹窗（不可关闭） -->
  <dialog class="modal modal-open">
    <div class="modal-box bg-base-100 border border-base-300 shadow-2xl w-96">
      <!-- 标题 -->
      <div class="flex items-center gap-3 pb-4 border-b border-base-300">
        <div class="p-2 bg-neutral text-neutral-content rounded-lg">
          <Icon icon="mdi:shield-key" class="text-xl" />
        </div>
        <div>
          <h3 class="font-semibold text-lg">授权验证</h3>
          <p class="text-xs text-base-content/60">请输入卡密信息登录</p>
        </div>
      </div>

      <form class="mt-5 space-y-4" @submit.prevent="handleLogin">
        <!-- 用户 ID -->
        <div class="form-control">
          <label class="label py-1">
            <span class="label-text text-sm font-medium">用户 ID</span>
          </label>
          <input
            v-model="userId"
            type="text"
            placeholder="请输入购买时的用户 ID"
            class="input input-bordered w-full focus:input-neutral"
            :disabled="authStore.isLoading"
          />
        </div>

        <!-- 卡密 -->
        <div class="form-control">
          <label class="label py-1">
            <span class="label-text text-sm font-medium">卡密</span>
          </label>
          <div class="relative">
            <input
              v-model="cardCode"
              :type="showPassword ? 'text' : 'password'"
              placeholder="请输入卡密码"
              class="input input-bordered w-full pr-10 focus:input-neutral"
              :disabled="authStore.isLoading"
            />
            <button
              type="button"
              class="absolute right-3 top-1/2 -translate-y-1/2 text-base-content/50 hover:text-base-content"
              @click="showPassword = !showPassword"
            >
              <Icon :icon="showPassword ? 'mdi:eye-off' : 'mdi:eye'" class="text-lg" />
            </button>
          </div>
        </div>

        <!-- 错误提示 -->
        <div v-if="authStore.error" class="alert alert-error text-sm py-2">
          <Icon icon="mdi:alert-circle-outline" />
          <span>{{ authStore.error }}</span>
        </div>

        <!-- 按钮组 -->
        <div class="flex gap-2 pt-2">
          <button type="button" class="btn btn-ghost" @click="openRecharge">
            <Icon icon="mdi:open-in-new" />
            充值续费
          </button>
          <button
            type="submit"
            class="btn btn-neutral flex-1"
            :disabled="authStore.isLoading || !cardCode.trim() || !userId.trim()"
          >
            <span v-if="authStore.isLoading" class="loading loading-spinner loading-sm"></span>
            <template v-else>
              <Icon icon="mdi:login" />
              登录
            </template>
          </button>
        </div>
      </form>
    </div>
    <!-- 背景遮罩 -->
    <div class="modal-backdrop bg-neutral/60"></div>
  </dialog>
</template>
