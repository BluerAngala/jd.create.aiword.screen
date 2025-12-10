# Tauri + Vue + DaisyUI 项目模板

一个用于快速开发桌面应用的项目模板，集成了：

- **Tauri 2.x** - 轻量级桌面应用框架
- **Vue 3** - 渐进式前端框架
- **TypeScript** - 类型安全
- **DaisyUI** - 基于 Tailwind 的 UI 组件库
- **Vite** - 快速构建工具

## 快速开始

```bash
# 安装依赖
pnpm install

# 开发模式
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

## 项目结构

```
├── src/                    # Vue 前端
│   ├── components/         # 组件
│   ├── styles/             # 样式
│   └── App.vue             # 根组件
├── src-tauri/              # Rust 后端
│   ├── src/lib.rs          # 主逻辑
│   └── tauri.conf.json     # 配置
└── .kiro/steering/         # Kiro AI 开发指南
```

## 开发指南

查看 `.kiro/steering/tauri-vue-development.md` 获取详细的开发规范和示例代码。

## 使用模板

复制此目录作为新项目的起点，然后修改：

1. `package.json` 中的 `name`
2. `src-tauri/Cargo.toml` 中的 `name`
3. `src-tauri/tauri.conf.json` 中的 `productName` 和 `identifier`
