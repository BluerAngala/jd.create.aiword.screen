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
  x?: number // 窗口 x 坐标
  y?: number // 窗口 y 坐标
  borderImage?: string // 边框图片本地路径
}

// 投屏配置方案
export interface ScreenPreset {
  id: string // 唯一标识
  name: string // 方案名称
  config: ImageSettings // 配置内容
  createdAt: string // 创建时间
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
  title: string // 直播间标题
  indexImage: string // 封面图（4:3）
  resizeIndexImage: string // 封面图（2:1）
  squareIndexImage: string // 封面图（1:1）
  portraitIndexImage: string // 封面图（3:4）
  type: number // 直播类型，固定 69
  publishTime: string // 发布时间，格式：YYYY-MM-DD HH:mm:ss
  screen: number // 横竖屏，0=竖屏
  test: number // 是否测试，0=否
  locationDetail: string | null // 位置详情
  canExplain: number // 是否可讲解，1=是
  preVideoType: number // 预告视频类型，0=无
  desc: string // 描述
  welcome: string // 欢迎语
  channelNum: string // 频道号
  pcVersion: number // PC 版本，固定 1
}

// 创建直播间响应
export interface CreateLiveResponse {
  success: boolean
  code: number
  subcode?: number
  successMsg?: string
  errorMsg?: string
  liveId?: number
  ddMsg?: string
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

// ============ 商品详情相关（购物袋功能）============

// 商品详情（从京东接口返回的完整数据）
export interface SkuInfo {
  sku: string
  title?: string
  img?: string
  description?: string
  threeCategory?: string
  twoCategory?: string
  oneCategory?: string
  brandId?: string
  shopId?: string
  vendorId?: string
  price?: string
  cpsPrice?: string
  cpsRate?: string
  cpsShareButton?: boolean
  brokerageRatio?: string
  marketPrice?: string
  spu?: string
  shopName?: string
  skuStatus?: string
  isCanUseDQ?: string
  isCanUseJQ?: string
  wxsp?: string
  colType?: string
  dxType?: string
  msbybt?: number
  priceStar?: string
  ttdj?: string
  viewStatus?: string
  idxGoods?: string
  indexSkuMaxPrice?: string
  virtualBundles?: boolean
  csTime?: string
  ceTime?: string
  couponLimitStatus?: string
  customizeType?: string
  couponSwitch?: string
  couponLimitCount?: string
  address?: string
  storeName?: string
  storeId?: string
  stockState?: number
  id?: string
  liveId?: string
  type?: number
  secKillFlag?: number
  tag?: number
  sort?: string
  top?: number
  explainBegin?: number
  promotionId?: string
  promotionPrice?: string
  activityId?: string
  source?: number
  startTime?: string
  endTime?: string
  status?: number
  liveOnly?: string
  purchased?: string
  denomination?: string
  quota?: string
  limit?: string
  promotionExtra?: string
  finished?: string
  benefit?: string
  url?: string
  explainStatus?: string
  copyUrl?: string
  wjShopId?: string
  salerId?: string
  couponKey?: string
  couponType?: string
  couponInfo?: string
  couponOpTime?: string
  skuMakeUp?: string
  skuSkinTest?: string
  specialKeyFlag?: string
  createTime?: string
  explainEnd?: number
  ext?: string
  flashLiveAnnouncement?: boolean
  skuReserveNum?: string
  skuReserveNumWx?: string
  skuReserveNumApp?: string
  errorTips?: string
  batchId?: number
  flashPlayFlag?: string
  promotionStart?: string
  promotionEnd?: string
  flashPlayStatus?: string
  opId?: string
  crId?: string
  buId?: string
  shopMode?: string
  bpin?: string
  promoteStatus?: string
  groupIds?: string
  groupNames?: string
  groupId?: string
  groupName?: string
  promotionStatus?: string
  promotionStock?: string
  incrementPrice?: string
  bidCount?: string
  saleTotal?: string
  restChannelStock?: string
  sentChannelStock?: string
  highQualityStartTime?: string
  highQualityEndTime?: string
  applyPrice?: string
  highSkuActStatus?: string
  addSource?: string
  highSkuTablePrimaryKey?: string
  applyId?: string
  chooseTime?: string
  priceHighFlag?: string
  tipReason?: string
  promoBatchId?: string
  promoSwitchType?: string
  promoStockType?: string
  prepareStockNum?: string
  adCouponGuideWord?: string
  syncSkuCount?: string
  stockActId?: string
  wan8StartTime?: string
  wan8EndTime?: string
  businessOpportunityFlag?: string
  authorId?: string
  forbiddenReason?: string
  underAuction?: boolean
  canUploadSkuVideo?: boolean
  videoPlayUrl?: string
  videoSource?: string
  canChangeLimitPrice?: boolean
  offlineOrderNums?: string
  uploadVideoUrl?: string
  uploadVideoFailedReason?: string
  isFlashSale?: string
  fsInvalidDesc?: string
  fsValidThreshold?: string
  fsArrivalPrice?: string
}

// 添加商品结果
export interface AddSkuResult {
  success: boolean
  success_count: number
  error_msg?: string
}

// ============ 直播商品数据（持久化）============

// 直播商品（简化版，用于 AI 话术、投屏、讲解控制）
export interface LiveProduct {
  sku: string // 商品 ID
  title: string // 商品名称
  img: string // 商品图片
  price?: string // 价格
  shopName?: string // 店铺名称
}

// 直播场次数据
export interface LiveSession {
  id: string // 唯一标识（使用 liveId）
  liveId: number // 直播间 ID
  title: string // 直播间标题
  browserName: string // 使用的浏览器名称
  accountName: string // 京东账号昵称
  startTime: string // 直播开始时间
  createdAt: string // 创建时间
  products: LiveProduct[] // 直播商品列表
  scripts?: AIScript[] // AI 话术列表
}
