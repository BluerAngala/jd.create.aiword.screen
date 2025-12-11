# Tauri + Vue + DaisyUI 开发指南

## 项目技术栈

- **前端框架**: Vue 3 + TypeScript
- **UI 组件库**: DaisyUI 5.x (基于 Tailwind CSS 4.x)
- **桌面框架**: Tauri 2.x (Rust 后端)
- **构建工具**: Vite + Rolldown
- **包管理器**: pnpm
- **动画库**: @vueuse/motion + auto-animate
- **图标库**: @iconify/vue
- **工具库**: VueUse
- **状态管理**: Pinia
- **路由**: Vue Router

## 项目结构

```
├── src/
│   ├── assets/             # 静态资源
│   ├── config/             # 应用配置
│   │   └── app.config.ts   # 功能开关和导航配置
│   ├── core/               # 核心功能（必须保留）
│   │   ├── components/     # 核心组件
│   │   │   ├── ThemeSelector.vue
│   │   │   └── ToastContainer.vue
│   │   ├── composables/    # 核心组合式函数
│   │   │   ├── useLogger.ts
│   │   │   ├── useTheme.ts
│   │   │   └── useToast.ts
│   │   └── stores/         # 核心状态管理
│   │       └── app.ts
│   ├── modules/            # 可选功能模块
│   │   ├── auth/           # 认证模块（卡密登录）
│   │   ├── cookie/         # Cookie 读取模块
│   │   ├── demo/           # 演示模块（动画、图标、Hooks）
│   │   └── screen/         # 投屏模块
│   ├── router/             # 路由配置
│   ├── styles/             # 全局样式
│   ├── types/              # TypeScript 类型
│   ├── views/              # 核心页面组件
│   ├── App.vue             # 根组件
│   └── main.ts             # 入口文件
├── src-tauri/
│   ├── src/
│   │   ├── lib.rs          # Tauri 命令和插件
│   │   └── main.rs         # 入口
│   ├── capabilities/       # 权限配置
│   ├── Cargo.toml          # Rust 依赖（带模块注释）
│   └── tauri.conf.json     # Tauri 配置
├── .github/workflows/      # CI/CD 工作流
├── package.json
└── vite.config.ts
```

## 开发命令

```bash
pnpm install          # 安装依赖
pnpm tauri dev        # 开发模式
pnpm tauri build      # 构建应用
pnpm lint             # ESLint 检查
pnpm format           # Prettier 格式化
pnpm type-check       # TypeScript 检查
```

## 前端开发规范

### 路径别名

使用 `@/` 代替 `src/`：

```typescript
import { useLogger } from '@/composables/useLogger'
import Home from '@/views/Home.vue'
```

### 日志系统

```typescript
import { useLogger } from '@/core/composables'

const logger = useLogger('组件名')
logger.info('信息')
logger.warn('警告')
logger.error('错误')
```

### 调用 Tauri 后端

```typescript
import { invoke } from '@tauri-apps/api/core'

const result = await invoke<string>('command_name', { param: 'value' })
```

### DaisyUI 组件

```html
<!-- 按钮 -->
<button class="btn btn-primary">Primary</button>
<button class="btn btn-secondary">Secondary</button>

<!-- 卡片 -->
<div class="card bg-base-100 shadow-xl">
  <div class="card-body">
    <h2 class="card-title">标题</h2>
    <p>内容</p>
  </div>
</div>

<!-- 输入框 -->
<input class="input input-bordered w-full" placeholder="输入..." />

<!-- 提示框 -->
<div class="alert alert-success">成功</div>
```

### 图标使用

```vue
<script setup>
import { Icon } from '@iconify/vue'
</script>

<template>
  <Icon icon="mdi:home" />
  <Icon icon="heroicons:heart" class="text-2xl text-red-500" />
  <Icon icon="lucide:settings" />
</template>
```

常用图标集：
- `mdi:` - Material Design Icons
- `heroicons:` - Heroicons
- `lucide:` - Lucide Icons

### 动画效果

#### Motion 动画

```vue
<script setup>
import { useMotion } from '@vueuse/motion'

const boxRef = ref()
const { apply } = useMotion(boxRef, {
  initial: { scale: 1 },
})

const bounce = () => {
  apply({ scale: 1.2 })
  setTimeout(() => apply({ scale: 1 }), 150)
}
</script>

<template>
  <div ref="boxRef" @click="bounce">点击弹跳</div>
</template>
```

#### 指令式动画

```vue
<div
  v-motion
  :initial="{ opacity: 0, y: 20 }"
  :enter="{ opacity: 1, y: 0 }"
  :hovered="{ scale: 1.05 }"
>
  悬停放大
</div>
```

#### 列表自动动画

```vue
<script setup>
import { useAutoAnimate } from '@formkit/auto-animate/vue'
const [parent] = useAutoAnimate()
</script>

<template>
  <ul ref="parent">
    <li v-for="item in items" :key="item.id">{{ item }}</li>
  </ul>
</template>
```

### VueUse 常用函数

```typescript
import {
  useMouse,        // 鼠标位置
  useWindowSize,   // 窗口大小
  useClipboard,    // 剪贴板
  useLocalStorage, // 本地存储
  useOnline,       // 网络状态
  useDark,         // 深色模式
  useElementHover, // 元素悬停
  useBattery,      // 电池状态
} from '@vueuse/core'
```

### 主题切换

```typescript
import { useTheme } from '@/core/composables'

const { currentTheme, setTheme, themes } = useTheme()
setTheme('dark')  // 切换到深色主题
```

## 后端开发规范 (Rust)

### Tauri Command

```rust
use log::info;

#[tauri::command]
fn my_command(param: &str) -> String {
    info!("收到参数: {}", param);
    format!("结果: {}", param)
}

// 异步命令
#[tauri::command]
async fn async_command() -> Result<String, String> {
    Ok("完成".to_string())
}
```

### 注册命令

在 `lib.rs` 的 `run()` 函数中：

```rust
.invoke_handler(tauri::generate_handler![my_command, async_command])
```

### 添加插件权限

在 `src-tauri/capabilities/default.json` 中：

```json
{
  "permissions": [
    "core:default",
    "opener:default",
    "dialog:default",
    "log:default"
  ]
}
```

## 发布流程

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions 会自动打包 Windows、macOS、Linux 版本。

## 注意事项

1. 新增 Tauri 插件需要在 `capabilities/default.json` 添加权限
2. Rust 代码修改需要重新编译，前端支持热重载
3. 使用 `pnpm lint` 确保代码规范
4. 日志会同时输出到控制台和 Tauri 日志系统
