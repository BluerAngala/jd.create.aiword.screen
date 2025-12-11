/**
 * 京东直播助手类型定义
 */

// 应用设置
export interface AppSettings {
  theme: string // 主题名称
  loginInfo: {
    apiKey?: string // API 密钥
  }
  aiConfig: {
    enabled: boolean // AI 功能开关
    model?: string // AI 模型
    apiEndpoint?: string // API 端点
  }
}

// AI 话术设置
export interface AIScriptSettings {
  model: string // AI 模型
  apiKey: string // API 秘钥
  prompt: string // 提示词
}

// 浏览器信息
export interface BrowserInfo {
  id: string
  name: string // 浏览器名称
  profilePath: string // 配置文件路径
  jdAccount?: {
    nickname: string // 京东账号昵称
    isLoggedIn: boolean // 是否已登录
  }
}

// 商品项
export interface ProductItem {
  id: string
  name: string // 商品名称
  quantity: number // 添加数量
  titles: string[] // 标题列表（至少 1 个）
  imageUrl?: string // 商品图片
}

// 图片设置
export interface ImageSettings {
  width: number // 图片宽度
  height: number // 图片高度
}

// 日志条目
export interface LogEntry {
  id: string
  timestamp: Date
  level: 'info' | 'warn' | 'error' | 'success'
  message: string
}


// 直播参数
export interface LiveParameters {
  totalProducts: number // 当场直播商品总数量
  cartProducts: number // 添加购物袋的商品数量
  startTime: Date // 直播开始时间
}

// AI 话术
export interface AIScript {
  id: string
  content: string // 话术内容
  productId?: string // 关联商品 ID
}

// 直播状态
export interface LiveState {
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
