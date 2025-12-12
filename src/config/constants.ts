/**
 * 应用常量配置
 * 统一管理 AI 配置、API 端点、本地存储 Key、提示词等
 */

// ============ API 配置 ============

/** SiliconFlow API 基础地址 */
export const SILICON_FLOW_API_BASE = 'https://api.siliconflow.cn/v1'

/** SiliconFlow API 端点 */
export const API_ENDPOINTS = {
  /** 聊天补全 */
  CHAT_COMPLETIONS: `${SILICON_FLOW_API_BASE}/chat/completions`,
  /** 模型列表 */
  MODELS: `${SILICON_FLOW_API_BASE}/models`,
} as const

/** SiliconFlow 注册链接（带邀请码） */
export const SILICON_FLOW_REGISTER_URL = 'https://cloud.siliconflow.cn/i/WFoChvZf'

// ============ AI 模型配置 ============

/** 默认 AI 模型 */
export const DEFAULT_AI_MODEL = 'Qwen/Qwen2-7B-Instruct'

/** AI 请求默认参数 */
export const AI_REQUEST_DEFAULTS = {
  /** 最大 token 数 */
  MAX_TOKENS: 2000,
  /** 温度（创造性） */
  TEMPERATURE: 0.7,
  /** 标题生成温度（更高创造性） */
  TITLE_TEMPERATURE: 0.7,
  /** 标题生成最大 token */
  TITLE_MAX_TOKENS: 1000,
} as const

// ============ AI 提示词配置 ============

export const AI_PROMPTS = {
  /** 话术生成系统提示词 */
  SCRIPT_SYSTEM: '你是一个专业的直播带货主播，请根据商品信息生成吸引人的直播话术。',

  /** 话术生成用户提示词模板（使用 {title}、{price}、{shopName} 占位符） */
  SCRIPT_USER_TEMPLATE: `这是商品的基本信息：
商品名称：{title}
商品价格：{price}
店铺名称：{shopName}

我要做直播商品讲解，你帮我写一个商品的口播词，400字左右，生成我直接可以用的，不用写框架，不要使用表情和颜文字，写成一段就行，别留那么多空行，使用中文。不要使用最多，第一，必须，顶级，国家级等极限词，不要使用美白，抗皱违禁词。类似这样，

家人们，今天给大家介绍一款超棒的手机 —— 荣耀 Play9T！
外观上，荣耀 Play9T 简直就是 "颜值担当"。它的机身线条流畅，轻薄又便携，拿在手里十分舒适。时尚的配色方案，每一种都独具魅力，不管你是走简约风还是个性风，都能选到合心意的颜色。
性能方面，荣耀 Play9T 也毫不逊色。搭载了强劲的处理器，运行速度超快，多任务处理轻松自如。无论是刷短视频、玩游戏，还是日常办公，它都能高效完成，让你告别卡顿和等待的烦恼。
它的拍照功能同样出色。高清的摄像头，能够捕捉每一个精彩瞬间。白天拍摄，色彩鲜艳、画面清晰；夜晚拍摄，也能有效减少噪点，拍出质感满满的照片。有了它，你随时都能记录生活中的美好。
再说说续航，荣耀 Play9T 配备了大容量电池，续航能力超强。就算你一整天都在使用手机，也不用担心电量不足。而且它还支持快速充电，短时间就能补充大量电量，让你没有后顾之忧。
这么一款外观美、性能强、拍照好、续航久的荣耀 Play9T，你还在等什么呢？赶紧入手一台吧！`,

  /** 直播间标题生成提示词 */
  TITLE_GENERATION: `生成京东直播间通用标题，要求：
1.每个标题不超过15个字 
2.简洁有力突出卖点 
3.不要标点符号 
4.不要序号 
5.不要有具体的商品类型或者名称 
6.不要使用敏感词如：诱惑、最、第一、绝对、秒杀、疯狂、限时等夸大或营销敏感词汇`,

  /** 直播间欢迎语 */
  LIVE_WELCOME: '欢迎来到我的直播间，喜欢我可以点一下关注哦~',

  /** 默认话术（无商品时） */
  DEFAULT_SCRIPT: '欢迎来到直播间！今天给大家带来超值好物推荐~',

  /** 话术模板列表（AI 生成失败时的备选） */
  SCRIPT_TEMPLATES: [
    '这款商品非常推荐给大家！品质有保障，价格也很实惠。喜欢的朋友们可以点击下方链接下单哦~',
    '家人们看过来！这款商品是我们精心挑选的，性价比超高！数量有限，先到先得~',
    '宝子们，这款商品真的太棒了！我自己也在用，强烈推荐给大家！赶紧下单吧~',
  ],
} as const

// ============ 本地存储 Key ============

export const STORAGE_KEYS = {
  /** 应用设置 */
  SETTINGS: 'jd-live-assistant-settings',
  /** AI 设置 */
  AI_SETTINGS: 'jd-live-assistant-ai-settings',
  /** 商品文件列表 */
  PRODUCT_FILES: 'jd-live-assistant-product-files',
  /** 直播间标题设置 */
  TITLE_SETTINGS: 'jd-live-title-settings',
} as const

// ============ 直播配置 ============

export const LIVE_CONFIG = {
  /** 默认直播类型 */
  TYPE: 69,
  /** 默认横竖屏（0=竖屏） */
  SCREEN: 0,
  /** 默认是否测试（0=否） */
  TEST: 0,
  /** 默认是否可讲解（1=是） */
  CAN_EXPLAIN: 1,
  /** 默认预告视频类型（0=无） */
  PRE_VIDEO_TYPE: 0,
  /** 默认 PC 版本 */
  PC_VERSION: 1,
  /** 最小讲解时长（毫秒） */
  MIN_EXPLAIN_DURATION: 63000,
  /** 讲解操作间隔（毫秒） */
  EXPLAIN_ACTION_DELAY: 1500,
  /** 默认讲解时长（秒） */
  DEFAULT_EXPLAIN_DURATION: 70 as number,
  /** 默认休息时长（秒） */
  DEFAULT_REST_DURATION: 10 as number,
  /** 最大日志条数 */
  MAX_LOGS: 500,
  /** 最大保存直播场次数 */
  MAX_SESSIONS: 50,
  /** 直播间过期时间（分钟） */
  SESSION_EXPIRE_MINUTES: 30,
  /** 最小开播时间（分钟后） */
  MIN_START_TIME_MINUTES: 3,
  /** 最大开播时间（天后） */
  MAX_START_TIME_DAYS: 30,
} as const

// ============ 默认值配置 ============

export const DEFAULTS = {
  /** 默认图片宽度 */
  IMAGE_WIDTH: 300,
  /** 默认图片高度 */
  IMAGE_HEIGHT: 300,
  /** 默认直播商品总数 */
  TOTAL_PRODUCTS: 10,
  /** 默认购物袋商品数量 */
  CART_PRODUCTS: 150,
  /** 批量添加商品最大数量 */
  MAX_BATCH_SIZE: 150,
  /** 商品文件默认使用数量 */
  FILE_USE_COUNT: 999,
} as const

// ============ 工具函数 ============

/**
 * 生成话术用户提示词
 * @param title 商品标题
 * @param price 商品价格
 * @param shopName 店铺名称
 */
export function generateScriptUserPrompt(
  title: string,
  price: string = '优惠价',
  shopName: string = '',
): string {
  return AI_PROMPTS.SCRIPT_USER_TEMPLATE
    .replace('{title}', title)
    .replace('{price}', price)
    .replace('{shopName}', shopName)
}
