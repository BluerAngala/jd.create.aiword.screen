/**
 * 京东直播 API 封装
 */

import { invoke } from '@tauri-apps/api/core'
import type {
  Cookie,
  JdLoginResult,
  RecentLiveRoom,
  CreateLiveRequest,
  LiveGeneralData,
  SkuInfo,
  AddSkuResult,
} from '../types'

/**
 * 验证京东登录状态
 */
export async function verifyJdLogin(cookies: Cookie[]): Promise<JdLoginResult> {
  return invoke<JdLoginResult>('verify_jd_login', { cookies })
}

/**
 * 获取最近使用的直播间列表
 */
export async function getRecentLiveRooms(cookies: Cookie[]): Promise<RecentLiveRoom[]> {
  return invoke<RecentLiveRoom[]>('get_recent_live_rooms', { cookies })
}

/**
 * 创建直播间
 */
export async function createLiveRoom(
  cookies: Cookie[],
  request: CreateLiveRequest
): Promise<number> {
  return invoke<number>('create_live_room', { cookies, request })
}

/**
 * 上传商品到直播间
 */
export async function uploadSku(cookies: Cookie[], liveId: string, skuId: string): Promise<void> {
  return invoke<void>('upload_sku', { cookies, liveId, skuId })
}

/**
 * 添加商品到购物袋
 */
export async function addSkuToBag(
  cookies: Cookie[],
  liveId: string,
  skuIds: string[]
): Promise<void> {
  return invoke<void>('add_sku_to_bag', { cookies, liveId, skuIds })
}

/**
 * 获取直播实时数据
 */
export async function getLiveGeneralData(
  cookies: Cookie[],
  liveId: string
): Promise<LiveGeneralData> {
  return invoke<LiveGeneralData>('get_live_general_data', { cookies, liveId })
}

/**
 * 获取 H5 页面 URL
 */
export async function getH5Url(cookies: Cookie[], liveId: string): Promise<string> {
  return invoke<string>('get_h5_url', { cookies, liveId })
}

/**
 * 开始讲解商品
 */
export async function startExplain(
  cookies: Cookie[],
  liveId: string,
  skuId: string
): Promise<void> {
  return invoke<void>('start_explain', { cookies, liveId, skuId })
}

/**
 * 结束讲解商品
 */
export async function endExplain(cookies: Cookie[], liveId: string, skuId: string): Promise<void> {
  return invoke<void>('end_explain', { cookies, liveId, skuId })
}

/**
 * 通过上传文件获取商品详情
 */
export async function getSkuInfoByFile(
  cookies: Cookie[],
  liveId: number,
  skuIds: string[]
): Promise<SkuInfo[]> {
  return invoke<SkuInfo[]>('get_sku_info_by_file', { cookies, liveId, skuIds })
}

/**
 * 批量添加商品到购物袋
 */
export async function addSkuToBagBatch(
  cookies: Cookie[],
  liveId: number,
  skuList: SkuInfo[]
): Promise<AddSkuResult> {
  return invoke<AddSkuResult>('add_sku_to_bag_batch', { cookies, liveId, skuList })
}
