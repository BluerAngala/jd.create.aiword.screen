//! 京东直播相关功能模块

use log::info;
use serde::{Deserialize, Serialize};

use crate::cookie::Cookie;

// ============ 通用响应结构 ============

/// 京东作者信息响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JdAuthorResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub author_info: Option<JdAuthorInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JdAuthorInfo {
    pub name: String,
    pub pic: String,
    pub pin: String,
}

/// 验证京东登录状态结果
#[derive(Debug, Serialize, Deserialize)]
pub struct JdLoginResult {
    pub is_logged_in: bool,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

// ============ 直播间相关 ============

/// 最近使用的直播间
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecentLiveRoom {
    pub live_id: Option<String>,
    pub title: Option<String>,
    pub cover_url: Option<String>,
    pub status: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// 最近使用直播间响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentUsedIndexResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub data: Option<RecentUsedIndexData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentUsedIndexData {
    pub live_list: Option<Vec<RecentLiveRoom>>,
}

/// 创建直播间请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLiveRequest {
    pub title: String,
    pub index_image: String,                    // 封面图（4:3）
    pub resize_index_image: String,             // 封面图（2:1）
    pub square_index_image: String,             // 封面图（1:1）
    pub portrait_index_image: String,           // 封面图（3:4）
    #[serde(rename = "type")]
    pub live_type: i32,                         // 直播类型，固定 69
    pub publish_time: String,                   // 发布时间
    pub screen: i32,                            // 横竖屏，0=竖屏
    pub test: i32,                              // 是否测试，0=否
    pub location_detail: Option<String>,        // 位置详情
    pub can_explain: i32,                       // 是否可讲解，1=是
    pub pre_video_type: i32,                    // 预告视频类型，0=无
    pub desc: String,                           // 描述
    pub welcome: String,                        // 欢迎语
    pub channel_num: String,                    // 频道号
    pub pc_version: i32,                        // PC 版本，固定 1
}

/// 创建直播间响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLiveResponse {
    pub success: bool,
    pub code: i32,
    pub subcode: Option<i32>,
    pub success_msg: Option<String>,
    pub error_msg: Option<String>,
    pub live_id: Option<i64>,
    pub dd_msg: Option<String>,
}

// ============ 商品相关 ============

/// 上传商品请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSkuRequest {
    pub live_id: String,
    pub sku_id: String,
}

/// 添加商品到购物袋请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSkuRequest {
    pub live_id: String,
    pub sku_ids: Vec<String>,
}

/// 商品操作响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkuOperationResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
}

// ============ 实时数据相关 ============

/// 直播实时数据
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveGeneralData {
    pub online_count: Option<i64>,
    pub total_watch_count: Option<i64>,
    pub like_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub share_count: Option<i64>,
    pub order_count: Option<i64>,
    pub order_amount: Option<f64>,
}

/// 实时数据响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralDataResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub data: Option<LiveGeneralData>,
}

// ============ 讲解相关 ============

/// 讲解操作请求
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainRequest {
    pub live_id: String,
    pub sku_id: String,
}

/// 讲解操作响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplainResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
}

// ============ H5 页面相关 ============

/// H5 页面响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct H5Response {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub url: Option<String>,
}

// ============ HTTP 客户端辅助函数 ============

/// 将 Cookie 数组转换为请求头格式
fn cookies_to_string(cookies: &[Cookie]) -> String {
    cookies
        .iter()
        .map(|c| format!("{}={}", c.name, c.value))
        .collect::<Vec<_>>()
        .join("; ")
}

/// 构建通用请求头
fn build_headers(cookie_str: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(value) = cookie_str.parse() {
        headers.insert(reqwest::header::COOKIE, value);
    }
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        "https://drlives.jd.com/".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        "application/json, text/plain, */*".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json".parse().unwrap(),
    );
    headers
}

// ============ Tauri Commands ============

/// 验证京东登录状态（通过后端发起请求）
#[tauri::command]
pub async fn verify_jd_login(cookies: Vec<Cookie>) -> Result<JdLoginResult, String> {
    info!("[验证登录] 开始验证京东登录状态");
    info!("[验证登录] 收到 {} 个 Cookie", cookies.len());

    let cookie_str = cookies_to_string(&cookies);
    info!("[验证登录] Cookie 字符串长度: {} 字符", cookie_str.len());

    let url = "https://drlives.jd.com/console/homePage/newGetAuthorInfo";
    info!("[验证登录] 请求 URL: {}", url);

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| {
            let err_msg = format!("请求失败: {}", e);
            info!("[验证登录] {}", err_msg);
            err_msg
        })?;

    info!("[验证登录] 响应状态码: {}", response.status());

    let response_text = response.text().await.map_err(|e| {
        let err_msg = format!("读取响应失败: {}", e);
        info!("[验证登录] {}", err_msg);
        err_msg
    })?;

    info!("[验证登录] 响应内容: {}", response_text);

    let data: JdAuthorResponse = serde_json::from_str(&response_text).map_err(|e| {
        let err_msg = format!("解析响应失败: {}", e);
        info!("[验证登录] {}", err_msg);
        err_msg
    })?;

    if data.success {
        if let Some(author) = data.author_info {
            info!("[验证登录] 登录成功，用户: {}", author.name);
            return Ok(JdLoginResult {
                is_logged_in: true,
                nickname: Some(author.name),
                avatar: Some(author.pic),
            });
        }
    }

    info!("[验证登录] 未登录: {:?}", data.error_msg);
    Ok(JdLoginResult {
        is_logged_in: false,
        nickname: None,
        avatar: None,
    })
}

/// 获取最近使用的直播间列表
#[tauri::command]
pub async fn get_recent_live_rooms(cookies: Vec<Cookie>) -> Result<Vec<RecentLiveRoom>, String> {
    info!("[最近直播间] 开始获取最近使用的直播间");

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live/pc/recentUsedIndex";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[最近直播间] 响应: {}", response_text);

    let data: RecentUsedIndexResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        if let Some(d) = data.data {
            return Ok(d.live_list.unwrap_or_default());
        }
    }

    Err(data.error_msg.unwrap_or_else(|| "获取失败".to_string()))
}


/// 构建创建直播间专用请求头
fn build_create_live_headers(cookie_str: &str) -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(value) = cookie_str.parse() {
        headers.insert(reqwest::header::COOKIE, value);
    }
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36 Edg/134.0.0.0"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        "https://jlive.jd.com/".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    headers
}

/// 创建直播间
#[tauri::command]
pub async fn create_live_room(
    cookies: Vec<Cookie>,
    request: CreateLiveRequest,
) -> Result<i64, String> {
    info!("[创建直播间] 开始创建直播间: {}", request.title);
    info!("[创建直播间] 发布时间: {}", request.publish_time);

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live/live-create";

    let client = reqwest::Client::new();
    let headers = build_create_live_headers(&cookie_str);

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[创建直播间] 响应: {}", response_text);

    let data: CreateLiveResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        if let Some(live_id) = data.live_id {
            info!("[创建直播间] 创建成功，直播间 ID: {}", live_id);
            return Ok(live_id);
        }
    }

    Err(data.error_msg.unwrap_or_else(|| "创建失败".to_string()))
}

/// 上传商品到直播间
#[tauri::command]
pub async fn upload_sku(
    cookies: Vec<Cookie>,
    live_id: String,
    sku_id: String,
) -> Result<(), String> {
    info!("[上传商品] 直播间: {}, 商品: {}", live_id, sku_id);

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live-shopping-bag/sku/uploadSku";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let request = UploadSkuRequest { live_id, sku_id };

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[上传商品] 响应: {}", response_text);

    let data: SkuOperationResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        return Ok(());
    }

    Err(data.error_msg.unwrap_or_else(|| "上传失败".to_string()))
}

/// 添加商品到购物袋
#[tauri::command]
pub async fn add_sku_to_bag(
    cookies: Vec<Cookie>,
    live_id: String,
    sku_ids: Vec<String>,
) -> Result<(), String> {
    info!("[添加商品] 直播间: {}, 商品数量: {}", live_id, sku_ids.len());

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live-shopping-bag/sku/add";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let request = AddSkuRequest { live_id, sku_ids };

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[添加商品] 响应: {}", response_text);

    let data: SkuOperationResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        return Ok(());
    }

    Err(data.error_msg.unwrap_or_else(|| "添加失败".to_string()))
}

/// 获取直播实时数据
#[tauri::command]
pub async fn get_live_general_data(
    cookies: Vec<Cookie>,
    live_id: String,
) -> Result<LiveGeneralData, String> {
    info!("[实时数据] 获取直播间 {} 的实时数据", live_id);

    let cookie_str = cookies_to_string(&cookies);
    let url = format!(
        "https://drlives.jd.com/liveRealTimeGeneralData/generalData?liveId={}",
        live_id
    );

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[实时数据] 响应: {}", response_text);

    let data: GeneralDataResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        if let Some(d) = data.data {
            return Ok(d);
        }
    }

    Err(data.error_msg.unwrap_or_else(|| "获取失败".to_string()))
}

/// 获取 H5 页面 URL
#[tauri::command]
pub async fn get_h5_url(cookies: Vec<Cookie>, live_id: String) -> Result<String, String> {
    info!("[H5页面] 获取直播间 {} 的 H5 页面", live_id);

    let cookie_str = cookies_to_string(&cookies);
    let url = format!("https://drlives.jd.com/h5?liveId={}", live_id);

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let response = client
        .get(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[H5页面] 响应: {}", response_text);

    let data: H5Response =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        if let Some(url) = data.url {
            return Ok(url);
        }
    }

    Err(data.error_msg.unwrap_or_else(|| "获取失败".to_string()))
}

/// 开始讲解商品
#[tauri::command]
pub async fn start_explain(
    cookies: Vec<Cookie>,
    live_id: String,
    sku_id: String,
) -> Result<(), String> {
    info!("[开始讲解] 直播间: {}, 商品: {}", live_id, sku_id);

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live/pc/explainBegin";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let request = ExplainRequest { live_id, sku_id };

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[开始讲解] 响应: {}", response_text);

    let data: ExplainResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        return Ok(());
    }

    Err(data.error_msg.unwrap_or_else(|| "开始讲解失败".to_string()))
}

/// 结束讲解商品
#[tauri::command]
pub async fn end_explain(
    cookies: Vec<Cookie>,
    live_id: String,
    sku_id: String,
) -> Result<(), String> {
    info!("[结束讲解] 直播间: {}, 商品: {}", live_id, sku_id);

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live/pc/explainEnd";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

    let request = ExplainRequest { live_id, sku_id };

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[结束讲解] 响应: {}", response_text);

    let data: ExplainResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        return Ok(());
    }

    Err(data.error_msg.unwrap_or_else(|| "结束讲解失败".to_string()))
}

// ============ 封面图片相关 ============

/// 封面图片项
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CoverImage {
    pub four_to_three: Option<String>,
    pub two_to_one: Option<String>,
    pub one_to_one: Option<String>,
    pub three_to_four: Option<String>,
}

/// 封面图片响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverImagesResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub data: Option<Vec<CoverImage>>,
}

/// 获取封面图片列表
#[tauri::command]
pub async fn get_cover_images(cookies: Vec<Cookie>) -> Result<Vec<CoverImage>, String> {
    info!("[封面图片] 开始获取封面图片列表");

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://api.m.jd.com/live_pc_recentUsedIndex?appid=plat-live-operate&functionId=live_pc_recentUsedIndex&PRICE_COLOR_API_TAG=true&use_color_api=true";

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(value) = cookie_str.parse() {
        headers.insert(reqwest::header::COOKIE, value);
    }
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        "https://jlive.jd.com/".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let body = "appid=plat-live-operate&functionId=live_pc_recentUsedIndex&body={}";

    let response = client
        .post(url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[封面图片] 响应: {}", response_text);

    let data: CoverImagesResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        return Ok(data.data.unwrap_or_default());
    }

    Err(data.error_msg.unwrap_or_else(|| "获取封面图片失败".to_string()))
}

// ============ 商品详情相关（购物袋功能）============

/// 商品详情（从京东接口返回的完整数据）
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkuInfo {
    pub sku: String,
    pub title: Option<String>,
    pub img: Option<String>,
    pub description: Option<String>,
    pub three_category: Option<String>,
    pub two_category: Option<String>,
    pub one_category: Option<String>,
    pub brand_id: Option<String>,
    pub shop_id: Option<String>,
    pub vendor_id: Option<String>,
    pub price: Option<String>,
    pub cps_price: Option<String>,
    pub cps_rate: Option<String>,
    pub cps_share_button: Option<bool>,
    pub brokerage_ratio: Option<String>,
    pub market_price: Option<String>,
    pub spu: Option<String>,
    pub shop_name: Option<String>,
    pub sku_status: Option<String>,
    pub is_can_use_dq: Option<String>,
    pub is_can_use_jq: Option<String>,
    pub wxsp: Option<String>,
    pub col_type: Option<String>,
    pub dx_type: Option<String>,
    pub msbybt: Option<i32>,
    pub price_star: Option<String>,
    pub ttdj: Option<String>,
    pub view_status: Option<String>,
    pub idx_goods: Option<String>,
    pub index_sku_max_price: Option<String>,
    pub virtual_bundles: Option<bool>,
    pub cs_time: Option<String>,
    pub ce_time: Option<String>,
    pub coupon_limit_status: Option<String>,
    pub customize_type: Option<String>,
    pub coupon_switch: Option<String>,
    pub coupon_limit_count: Option<String>,
    pub address: Option<String>,
    pub store_name: Option<String>,
    pub store_id: Option<String>,
    pub stock_state: Option<i32>,
    pub id: Option<String>,
    pub live_id: Option<String>,
    #[serde(rename = "type")]
    pub sku_type: Option<i32>,
    pub sec_kill_flag: Option<i32>,
    pub tag: Option<i32>,
    pub sort: Option<String>,
    pub top: Option<i32>,
    pub explain_begin: Option<i32>,
    pub promotion_id: Option<String>,
    pub promotion_price: Option<String>,
    pub activity_id: Option<String>,
    pub source: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub status: Option<i32>,
    pub live_only: Option<String>,
    pub purchased: Option<String>,
    pub denomination: Option<String>,
    pub quota: Option<String>,
    pub limit: Option<String>,
    pub promotion_extra: Option<String>,
    pub finished: Option<String>,
    pub benefit: Option<String>,
    pub url: Option<String>,
    pub explain_status: Option<String>,
    pub copy_url: Option<String>,
    pub wj_shop_id: Option<String>,
    pub saler_id: Option<String>,
    pub coupon_key: Option<String>,
    pub coupon_type: Option<String>,
    pub coupon_info: Option<String>,
    pub coupon_op_time: Option<String>,
    pub sku_make_up: Option<String>,
    pub sku_skin_test: Option<String>,
    pub special_key_flag: Option<String>,
    pub create_time: Option<String>,
    pub explain_end: Option<i32>,
    pub ext: Option<String>,
    pub flash_live_announcement: Option<bool>,
    pub sku_reserve_num: Option<String>,
    pub sku_reserve_num_wx: Option<String>,
    pub sku_reserve_num_app: Option<String>,
    pub error_tips: Option<String>,
    pub batch_id: Option<i32>,
    pub flash_play_flag: Option<String>,
    pub promotion_start: Option<String>,
    pub promotion_end: Option<String>,
    pub flash_play_status: Option<String>,
    pub op_id: Option<String>,
    pub cr_id: Option<String>,
    pub bu_id: Option<String>,
    pub shop_mode: Option<String>,
    pub bpin: Option<String>,
    pub promote_status: Option<String>,
    pub group_ids: Option<String>,
    pub group_names: Option<String>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub promotion_status: Option<String>,
    pub promotion_stock: Option<String>,
    pub increment_price: Option<String>,
    pub bid_count: Option<String>,
    pub sale_total: Option<String>,
    pub rest_channel_stock: Option<String>,
    pub sent_channel_stock: Option<String>,
    pub high_quality_start_time: Option<String>,
    pub high_quality_end_time: Option<String>,
    pub apply_price: Option<String>,
    pub high_sku_act_status: Option<String>,
    pub add_source: Option<String>,
    pub high_sku_table_primary_key: Option<String>,
    pub apply_id: Option<String>,
    pub choose_time: Option<String>,
    pub price_high_flag: Option<String>,
    pub tip_reason: Option<String>,
    pub promo_batch_id: Option<String>,
    pub promo_switch_type: Option<String>,
    pub promo_stock_type: Option<String>,
    pub prepare_stock_num: Option<String>,
    pub ad_coupon_guide_word: Option<String>,
    pub sync_sku_count: Option<String>,
    pub stock_act_id: Option<String>,
    pub wan8_start_time: Option<String>,
    pub wan8_end_time: Option<String>,
    pub business_opportunity_flag: Option<String>,
    pub author_id: Option<String>,
    pub forbidden_reason: Option<String>,
    pub under_auction: Option<bool>,
    pub can_upload_sku_video: Option<bool>,
    pub video_play_url: Option<String>,
    pub video_source: Option<String>,
    pub can_change_limit_price: Option<bool>,
    pub offline_order_nums: Option<String>,
    pub upload_video_url: Option<String>,
    pub upload_video_failed_reason: Option<String>,
    pub is_flash_sale: Option<String>,
    pub fs_invalid_desc: Option<String>,
    pub fs_valid_threshold: Option<String>,
    pub fs_arrival_price: Option<String>,
}

/// 获取商品详情响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSkuInfoResponse {
    pub success: bool,
    pub success_msg: Option<String>,
    pub error_msg: Option<String>,
    pub code: i32,
    pub subcode: Option<i32>,
    pub data: Option<Vec<SkuInfo>>,
}

/// 添加商品到购物袋请求（新版，使用完整商品详情）
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSkuBatchRequest {
    pub live_id: String,
    pub hide_error_msg: bool,
    pub sku_list: Vec<SkuInfo>,
}

/// 添加商品到购物袋响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddSkuBatchResponse {
    pub success: Option<bool>,
    pub success_msg: Option<String>,
    pub error_msg: Option<String>,
    pub code: Option<i32>,
    pub subcode: Option<i32>,
    // 成功时会返回 skuList（echo back）
    pub sku_list: Option<Vec<SkuInfo>>,
    pub live_id: Option<String>,
    pub hide_error_msg: Option<bool>,
}

/// 添加商品结果
#[derive(Debug, Serialize, Deserialize)]
pub struct AddSkuResult {
    pub success: bool,
    pub success_count: i32,
    pub error_msg: Option<String>,
}

/// 通过上传文件获取商品详情
#[tauri::command]
pub async fn get_sku_info_by_file(
    cookies: Vec<Cookie>,
    live_id: i64,
    sku_ids: Vec<String>,
) -> Result<Vec<SkuInfo>, String> {
    info!("[获取商品详情] 直播间: {}, 商品数量: {}", live_id, sku_ids.len());

    if sku_ids.is_empty() {
        return Ok(vec![]);
    }

    // 1. 生成临时 xlsx 文件
    let temp_dir = std::env::temp_dir();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let file_path = temp_dir.join(format!("jd-upload-{}.xlsx", timestamp));

    info!("[获取商品详情] 生成临时文件: {:?}", file_path);

    // 使用 rust_xlsxwriter 生成 xlsx
    {
        use rust_xlsxwriter::Workbook;
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();

        // 写入表头
        worksheet.write_string(0, 0, "skuId").map_err(|e| format!("写入表头失败: {}", e))?;

        // 写入商品 ID
        for (i, sku_id) in sku_ids.iter().enumerate() {
            worksheet.write_string((i + 1) as u32, 0, sku_id)
                .map_err(|e| format!("写入商品ID失败: {}", e))?;
        }

        workbook.save(&file_path).map_err(|e| format!("保存文件失败: {}", e))?;
    }

    // 2. 读取文件内容
    let file_content = tokio::fs::read(&file_path)
        .await
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 3. 构建 multipart 请求
    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live-shopping-bag/sku/uploadSku";

    let file_name = format!("jd-upload-{}.xlsx", timestamp);
    let file_part = reqwest::multipart::Part::bytes(file_content)
        .file_name(file_name.clone())
        .mime_str("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
        .map_err(|e| format!("创建文件部分失败: {}", e))?;

    let form = reqwest::multipart::Form::new()
        .text("skuFile", "商品上传.xlsx")
        .text("liveId", live_id.to_string())
        .text("type", "undefined")
        .part("file", file_part);

    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    if let Ok(value) = cookie_str.parse() {
        headers.insert(reqwest::header::COOKIE, value);
    }
    headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36"
            .parse()
            .unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        "https://jlive.jd.com/".parse().unwrap(),
    );
    headers.insert(
        reqwest::header::HOST,
        "drlives.jd.com".parse().unwrap(),
    );

    let response = client
        .post(url)
        .headers(headers)
        .multipart(form)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    // 4. 删除临时文件
    let _ = tokio::fs::remove_file(&file_path).await;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[获取商品详情] 响应长度: {} 字符", response_text.len());

    let data: GetSkuInfoResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("解析响应失败: {}", e))?;

    if data.success {
        let sku_list = data.data.unwrap_or_default();
        info!("[获取商品详情] 成功获取 {} 个商品详情", sku_list.len());
        return Ok(sku_list);
    }

    Err(data.error_msg.unwrap_or_else(|| "获取商品详情失败".to_string()))
}

/// 批量添加商品到购物袋
#[tauri::command]
pub async fn add_sku_to_bag_batch(
    cookies: Vec<Cookie>,
    live_id: i64,
    sku_list: Vec<SkuInfo>,
) -> Result<AddSkuResult, String> {
    info!("[批量添加商品] 直播间: {}, 商品数量: {}", live_id, sku_list.len());

    if sku_list.is_empty() {
        return Ok(AddSkuResult {
            success: true,
            success_count: 0,
            error_msg: None,
        });
    }

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live-shopping-bag/sku/add";

    let request = AddSkuBatchRequest {
        live_id: live_id.to_string(),
        hide_error_msg: true,
        sku_list: sku_list.clone(),
    };

    let client = reqwest::Client::new();
    let headers = build_create_live_headers(&cookie_str);

    let response = client
        .post(url)
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let response_text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    info!("[批量添加商品] 响应长度: {} 字符", response_text.len());

    // 尝试解析响应
    let data: AddSkuBatchResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("解析响应失败: {}", e))?;

    // 判断是否成功
    // 成功时：返回 skuList（echo back），没有 success 字段或 success=null
    // 失败时：success=false，有 errorMsg
    if let Some(false) = data.success {
        // 明确失败
        return Ok(AddSkuResult {
            success: false,
            success_count: 0,
            error_msg: data.error_msg,
        });
    }

    // 成功（返回了 skuList 或没有明确失败）
    let success_count = data.sku_list.map(|list| list.len() as i32).unwrap_or(sku_list.len() as i32);
    info!("[批量添加商品] 成功添加 {} 个商品", success_count);

    Ok(AddSkuResult {
        success: true,
        success_count,
        error_msg: None,
    })
}
