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
    pub cover_url: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

/// 创建直播间响应
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateLiveResponse {
    pub success: bool,
    pub code: i32,
    pub error_msg: Option<String>,
    pub live_id: Option<String>,
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


/// 创建直播间
#[tauri::command]
pub async fn create_live_room(
    cookies: Vec<Cookie>,
    request: CreateLiveRequest,
) -> Result<String, String> {
    info!("[创建直播间] 开始创建直播间: {}", request.title);

    let cookie_str = cookies_to_string(&cookies);
    let url = "https://drlives.jd.com/live/live-create";

    let client = reqwest::Client::new();
    let headers = build_headers(&cookie_str);

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
