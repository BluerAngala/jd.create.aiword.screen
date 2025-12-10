<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");
const showThemeModal = ref(false);

// 可用主题列表
const themes = [
  "light", "dark", "cupcake", "bumblebee", "emerald", "corporate",
  "synthwave", "retro", "cyberpunk", "valentine", "halloween", "garden",
  "forest", "aqua", "lofi", "pastel", "fantasy", "wireframe", "black",
  "luxury", "dracula", "cmyk", "autumn", "business", "acid", "lemonade",
  "night", "coffee", "winter", "dim", "nord", "sunset"
];

const currentTheme = ref("light");

// 切换主题
function setTheme(theme: string) {
  currentTheme.value = theme;
  document.documentElement.setAttribute("data-theme", theme);
  localStorage.setItem("theme", theme);
}

// 初始化主题
onMounted(() => {
  const saved = localStorage.getItem("theme") || "light";
  setTheme(saved);
});

async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="min-h-screen bg-base-200 p-8">
    <div class="max-w-md mx-auto">
      <h1 class="text-3xl font-bold text-center mb-8">Tauri + Vue + DaisyUI</h1>
      
      <!-- 主题切换按钮 -->
      <div class="flex justify-end mb-4">
        <button class="btn btn-ghost" @click="showThemeModal = true">
          🎨 主题: {{ currentTheme }}
        </button>
      </div>

      <!-- 主题选择弹窗 -->
      <dialog class="modal" :class="{ 'modal-open': showThemeModal }">
        <div class="modal-box w-11/12 max-w-3xl">
          <h3 class="font-bold text-lg mb-4">选择主题</h3>
          <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 gap-3">
            <button
              v-for="theme in themes"
              :key="theme"
              class="overflow-hidden rounded-lg border-2 transition-all hover:scale-105"
              :class="currentTheme === theme ? 'border-primary' : 'border-base-300'"
              :data-theme="theme"
              @click="setTheme(theme); showThemeModal = false"
            >
              <div class="bg-base-100 p-2">
                <div class="flex gap-1 mb-1">
                  <div class="w-2 h-2 rounded-full bg-primary"></div>
                  <div class="w-2 h-2 rounded-full bg-secondary"></div>
                  <div class="w-2 h-2 rounded-full bg-accent"></div>
                </div>
                <div class="text-xs text-base-content truncate">{{ theme }}</div>
              </div>
            </button>
          </div>
          <div class="modal-action">
            <button class="btn" @click="showThemeModal = false">关闭</button>
          </div>
        </div>
        <form method="dialog" class="modal-backdrop">
          <button @click="showThemeModal = false">关闭</button>
        </form>
      </dialog>
      
      <div class="card bg-base-100 shadow-xl">
        <div class="card-body">
          <h2 class="card-title">欢迎使用模板</h2>
          
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
            <button class="btn btn-primary" @click="greet">
              打招呼
            </button>
          </div>
          
          <div v-if="greetMsg" class="alert alert-success mt-4">
            <span>{{ greetMsg }}</span>
          </div>
        </div>
      </div>
      
      <!-- DaisyUI 组件示例 -->
      <div class="mt-8 space-y-4">
        <h3 class="text-xl font-semibold">DaisyUI 组件示例</h3>
        
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
</template>
