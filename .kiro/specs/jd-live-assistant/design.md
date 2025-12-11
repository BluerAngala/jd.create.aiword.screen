# 设计文档

## 概述

京东直播助手是一个基于 Tauri + Vue 3 + DaisyUI 的桌面应用。本设计文档聚焦于前端 UI 实现，后端 API 调用暂不涉及。

## 架构

```
src/modules/live/
├── components/           # 直播模块组件
│   ├── AnnouncementBar.vue      # 公告栏
│   ├── SettingsModal.vue        # 设置弹窗
│   ├── BrowserList.vue          # 浏览器列表
│   ├── ProductConfig.vue        # 商品信息配置
│   ├── ImageConfig.vue          # 图片设置
│   ├── ExecutionLog.vue         # 执行日志
│   ├── LiveParams.vue           # 直播参数
│   ├── CountdownTimer.vue       # 倒计时
│   └── AIScriptPanel.vue        # AI 话术面板
├── stores/
│   └── live.ts                  # 直播状态管理
├── types/
│   └── index.ts                 # 类型定义
├── views/
│   └── LiveAssistant.vue        # 主页面
└── index.ts                     # 模块导出
```

## 组件与接口

### 1. AnnouncementBar 公告栏组件

```typescript
// Props
interface AnnouncementBarProps {
  content: string        // 公告内容
}

// Emits
// 无
```

### 2. SettingsModal 设置弹窗组件

```typescript
// Props
interface SettingsModalProps {
  visible: boolean       // 弹窗可见性
}

// Emits
interface SettingsModalEmits {
  'update:visible': (value: boolean) => void
  'save': (settings: AppSettings) => void
}

// 设置数据结构
interface AppSettings {
  theme: string          // 主题名称
  loginInfo: {
    apiKey?: string      // API 密钥
  }
  aiConfig: {
    enabled: boolean     // AI 功能开关
    model?: string       // AI 模型
    apiEndpoint?: string // API 端点
  }
}
```

### 3. BrowserList 浏览器列表组件

```typescript
// Props
interface BrowserListProps {
  browsers: BrowserInfo[]     // 浏览器列表
  selectedId?: string         // 选中的浏览器 ID
}

// Emits
interface BrowserListEmits {
  'select': (browser: BrowserInfo) => void
  'refresh': () => void
}

// 数据结构
interface BrowserInfo {
  id: string
  name: string           // 浏览器名称
  profilePath: string    // 配置文件路径
  jdAccount?: {
    nickname: string     // 京东账号昵称
    isLoggedIn: boolean  // 是否已登录
  }
}
```

### 4. ProductConfig 商品配置组件

```typescript
// Props
interface ProductConfigProps {
  products: ProductItem[]
}

// Emits
interface ProductConfigEmits {
  'update': (products: ProductItem[]) => void
  'import': (file: File) => void
  'generate-titles': (productId: string) => void
}

// 数据结构
interface ProductItem {
  id: string
  name: string           // 商品名称
  quantity: number       // 添加数量
  titles: string[]       // 标题列表（至少 2 个）
  imageUrl?: string      // 商品图片
}
```

### 5. ImageConfig 图片配置组件

```typescript
// Props
interface ImageConfigProps {
  config: ImageSettings
}

// Emits
interface ImageConfigEmits {
  'update': (config: ImageSettings) => void
}

// 数据结构
interface ImageSettings {
  aspectRatio: string    // 宽高比，默认 "1:1"
  position: {
    x: number            // X 坐标
    y: number            // Y 坐标
  }
}
```

### 6. ExecutionLog 执行日志组件

```typescript
// Props
interface ExecutionLogProps {
  logs: LogEntry[]
  maxHeight?: string     // 最大高度，默认 "200px"
}

// 数据结构
interface LogEntry {
  id: string
  timestamp: Date
  level: 'info' | 'warn' | 'error' | 'success'
  message: string
}
```

### 7. LiveParams 直播参数组件

```typescript
// Props
interface LiveParamsProps {
  params: LiveParameters
}

// Emits
interface LiveParamsEmits {
  'update': (params: LiveParameters) => void
}

// 数据结构
interface LiveParameters {
  totalProducts: number      // 当场直播商品总数量
  cartProducts: number       // 添加购物袋的商品数量
  startTime: Date            // 直播开始时间
}
```

### 8. CountdownTimer 倒计时组件

```typescript
// Props
interface CountdownTimerProps {
  targetTime: Date | null    // 目标时间
  isRunning: boolean         // 是否运行中
}

// Emits
interface CountdownTimerEmits {
  'complete': () => void     // 倒计时结束
}
```

### 9. AIScriptPanel AI 话术面板组件

```typescript
// Props
interface AIScriptPanelProps {
  scripts: AIScript[]
  currentIndex: number       // 当前显示的话术索引
}

// 数据结构
interface AIScript {
  id: string
  content: string            // 话术内容
  productId?: string         // 关联商品 ID
}
```

## 数据模型

### LiveStore 状态管理

```typescript
interface LiveState {
  // 设置
  settings: AppSettings
  
  // 浏览器
  browsers: BrowserInfo[]
  selectedBrowserId: string | null
  
  // 商品
  products: ProductItem[]
  
  // 图片配置
  imageConfig: ImageSettings
  
  // 直播参数
  liveParams: LiveParameters
  
  // 执行状态
  logs: LogEntry[]
  isLiveRoomCreated: boolean
  isLiveStarted: boolean
  countdown: {
    targetTime: Date | null
    isRunning: boolean
  }
  
  // AI 话术
  aiScripts: AIScript[]
  currentScriptIndex: number
}
```



## 正确性属性

*属性是指在系统所有有效执行中都应保持为真的特征或行为——本质上是关于系统应该做什么的形式化陈述。属性是人类可读规范与机器可验证正确性保证之间的桥梁。*

### Property 1: 设置持久化 round-trip

*对于任意* 有效的设置对象，保存后再读取应该得到等价的设置对象。

**验证: 需求 2.5**

### Property 2: 商品数量必须为正整数

*对于任意* 商品配置，商品数量必须大于 0 且为整数。

**验证: 需求 4.3**

### Property 3: 商品标题不能为空

*对于任意* 有效的商品配置，标题数组长度必须至少为 1。

**验证: 需求 4.4**

### Property 4: 图片坐标必须为有效数值

*对于任意* 图片配置，位置坐标 x 和 y 必须为有限数值。

**验证: 需求 5.2**

### Property 5: 日志追加一致性

*对于任意* 日志条目序列，添加新日志后，日志列表长度应增加 1，且新日志应出现在列表中。

**验证: 需求 6.1**

### Property 6: 开始直播按钮状态依赖

*对于任意* 应用状态，当直播间未创建时，开始直播按钮必须处于禁用状态。

**验证: 需求 9.1**

### Property 7: 倒计时计算正确性

*对于任意* 目标时间和当前时间，倒计时显示的剩余秒数应等于目标时间减去当前时间的差值（向下取整到秒）。

**验证: 需求 9.4**

## 错误处理

### 浏览器相关错误

| 错误场景 | 处理方式 |
|---------|---------|
| 未检测到 Chrome 浏览器 | 显示提示信息，引导用户安装 Chrome |
| Cookie 获取失败 | 显示错误日志，提示用户检查浏览器登录状态 |
| 账号未登录京东 | 显示未登录状态，禁用相关功能 |

### 商品配置错误

| 错误场景 | 处理方式 |
|---------|---------|
| 商品文件格式错误 | 显示解析错误信息，拒绝导入 |
| 商品数量为空或非法 | 表单验证提示，阻止保存 |
| 标题为空 | 表单验证提示，要求至少填写一个标题 |

### 直播操作错误

| 错误场景 | 处理方式 |
|---------|---------|
| 创建直播间失败 | 显示错误日志，保持开始直播按钮禁用 |
| 直播执行中断 | 记录错误日志，显示中断原因 |

## 测试策略

### 单元测试

使用 Vitest 进行单元测试：

- 测试各组件的 Props 和 Emits 行为
- 测试 Store 的状态变更逻辑
- 测试工具函数（如倒计时计算、数据验证）

### 属性测试

使用 fast-check 进行属性测试：

- 每个属性测试运行至少 100 次迭代
- 测试文件使用注释标记对应的正确性属性
- 格式：`**Feature: jd-live-assistant, Property {number}: {property_text}**`

### 测试覆盖范围

| 组件/模块 | 单元测试 | 属性测试 |
|----------|---------|---------|
| SettingsModal | 渲染、交互 | Property 1 |
| ProductConfig | 渲染、验证 | Property 2, 3 |
| ImageConfig | 渲染、验证 | Property 4 |
| ExecutionLog | 渲染、追加 | Property 5 |
| LiveParams | 渲染、状态 | Property 6 |
| CountdownTimer | 渲染、计算 | Property 7 |
