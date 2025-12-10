# Tauri + Vue + DaisyUI 开发指南

## 项目技术栈

- **前端框架**: Vue 3 + TypeScript
- **UI 组件库**: DaisyUI 5.x (基于 Tailwind CSS 4.x)
- **桌面框架**: Tauri 2.x (Rust 后端)
- **构建工具**: Vite
- **包管理器**: pnpm

## CSS 配置 (Tailwind 4 + DaisyUI 5)

在 `src/styles/index.css` 中：

```css
@import "tailwindcss";
@plugin "daisyui";
```

注意：Tailwind 4 不需要 `tailwind.config.js` 和 `postcss.config.js`。

## 项目结构

```
├── src/                    # Vue 前端源码
│   ├── components/         # Vue 组件
│   ├── composables/        # 组合式函数
│   ├── styles/             # 全局样式
│   ├── types/              # TypeScript 类型定义
│   ├── utils/              # 工具函数
│   ├── App.vue             # 根组件
│   └── main.ts             # 入口文件
├── src-tauri/              # Tauri Rust 后端
│   ├── src/
│   │   ├── lib.rs          # 主要逻辑和 Commands
│   │   └── main.rs         # 入口
│   ├── capabilities/       # 权限配置
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── index.html              # HTML 入口
├── package.json            # 前端依赖
├── tailwind.config.js      # Tailwind 配置
└── vite.config.ts          # Vite 配置
```

## 开发命令

```bash
# 安装依赖
pnpm install

# 开发模式（前端 + Tauri）
pnpm tauri dev

# 仅前端开发
pnpm dev

# 构建生产版本
pnpm tauri build
```

## 前端开发规范

### Vue 组件编写

```vue
<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 响应式数据
const data = ref<string>("");

// 调用 Rust 后端
async function callBackend() {
  try {
    const result = await invoke<string>("command_name", { param: "value" });
    data.value = result;
  } catch (error) {
    console.error("调用失败:", error);
  }
}

onMounted(() => {
  callBackend();
});
</script>

<template>
  <div class="p-4">
    <!-- 使用 DaisyUI 组件 -->
    <button class="btn btn-primary" @click="callBackend">
      点击按钮
    </button>
  </div>
</template>
```

### DaisyUI 常用组件

```html
<!-- 按钮 -->
<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary">Secondary</button>
<button class="btn btn-accent">Accent</button>
<button class="btn btn-ghost">Ghost</button>
<button class="btn btn-outline">Outline</button>

<!-- 输入框 -->
<input type="text" class="input input-bordered w-full" placeholder="输入..." />

<!-- 卡片 -->
<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title">标题</h2>
    <p>内容</p>
    <div class="card-actions justify-end">
      <button class="btn btn-primary">操作</button>
    </div>
  </div>
</div>

<!-- 提示框 -->
<div class="alert alert-success">成功消息</div>
<div class="alert alert-error">错误消息</div>
<div class="alert alert-warning">警告消息</div>
<div class="alert alert-info">信息消息</div>

<!-- 加载状态 -->
<span class="loading loading-spinner loading-md"></span>

<!-- 模态框 -->
<dialog id="my_modal" class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">标题</h3>
    <p class="py-4">内容</p>
    <div class="modal-action">
      <form method="dialog">
        <button class="btn">关闭</button>
      </form>
    </div>
  </div>
</dialog>
```

### 主题切换

```typescript
// 切换主题
function setTheme(theme: string) {
  document.documentElement.setAttribute("data-theme", theme);
}

// 可用主题: light, dark, cupcake, cyberpunk, dracula
```

## 后端开发规范 (Rust)

### Tauri Command 编写

```rust
use serde::{Deserialize, Serialize};

// 简单命令
#[tauri::command]
fn simple_command(param: &str) -> String {
    format!("收到参数: {}", param)
}

// 带结构体的命令
#[derive(Debug, Serialize, Deserialize)]
pub struct MyData {
    pub field1: String,
    pub field2: i32,
}

#[tauri::command]
fn struct_command(data: MyData) -> Result<MyData, String> {
    Ok(MyData {
        field1: data.field1.to_uppercase(),
        field2: data.field2 * 2,
    })
}

// 异步命令
#[tauri::command]
async fn async_command() -> Result<String, String> {
    // 异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok("完成".to_string())
}

// 注册命令
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            simple_command,
            struct_command,
            async_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 添加 Rust 依赖

在 `src-tauri/Cargo.toml` 中添加：

```toml
[dependencies]
# 常用依赖
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
anyhow = "1"  # 错误处理
dirs = "5"    # 系统目录
```

### 文件对话框

```rust
// 前端调用
import { open, save } from "@tauri-apps/plugin-dialog";

// 打开文件
const file = await open({
  multiple: false,
  filters: [{ name: "图片", extensions: ["png", "jpg"] }]
});

// 保存文件
const path = await save({
  filters: [{ name: "文本", extensions: ["txt"] }]
});
```

## 注意事项

1. **前后端通信**: 使用 `invoke` 调用 Rust 命令，参数和返回值会自动序列化
2. **错误处理**: Rust 命令返回 `Result<T, String>` 时，前端需要 try-catch
3. **异步操作**: Rust 异步命令需要 `async` 关键字，前端调用时也需要 `await`
4. **权限配置**: 新增插件需要在 `capabilities/default.json` 中添加权限
5. **热重载**: 开发模式下前端支持热重载，Rust 代码修改需要重新编译
