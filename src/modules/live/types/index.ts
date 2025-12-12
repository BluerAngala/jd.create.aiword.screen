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

// 商品文件解析结果
export interface ProductFileResult {
  success: boolean
  productIds: string[] // 去重后的商品 ID 列表
  totalCount: number // 原始数据条数
  uniqueCount: number // 去重后的数量
  error?: string // 错误信息
}

// 商品文件信息
export interface ProductFile {
  id: string // 唯一标识
  name: string // 文件名
  productIds: string[] // 去重后的商品 ID 列表
  totalCount: number // 原始数据条数
  uniqueCount: number // 去重后的数量
  useCount: number // 每场直播使用的商品数量，默认 999
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


// ============ 京东直播 API 类型 ============

// 京东登录验证结果
export interface JdLoginResult {
  is_logged_in: boolean
  nickname?: string
  avatar?: string
}

// 最近使用的直播间
export interface RecentLiveRoom {
  live_id?: string
  title?: string
  cover_url?: string
  status?: number
  start_time?: string
  end_time?: string
}

// 创建直播间请求
export interface CreateLiveRequest {
  title: string
  cover_url?: string
  start_time?: string
  end_time?: string
}

// 直播实时数据
export interface LiveGeneralData {
  online_count?: number
  total_watch_count?: number
  like_count?: number
  comment_count?: number
  share_count?: number
  order_count?: number
  order_amount?: number
}

// Cookie 类型
export interface Cookie {
  name: string
  value: string
  domain: string
  path: string
  expires: number
  http_only: boolean
  secure: boolean
}
