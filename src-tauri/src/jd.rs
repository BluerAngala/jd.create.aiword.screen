//! 京东相关功能模块

use log::info;
use serde::{Deserialize, Serialize};

use crate::cookie::Cookie;

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

/// 验证京东登录状态（通过后端发起请求）
#[tauri::command]
pub async fn verify_jd_login(cookies: Vec<Cookie>) -> Result<JdLoginResult, String> {
    info!("[验证登录] 开始验证京东登录状态");
    info!("[验证登录] 收到 {} 个 Cookie", cookies.len());

    // 将 Cookie 数组转换为请求头格式
    let cookie_str: String = cookies
        .iter()
        .map(|c| format!("{}={}", c.name, c.value))
        .collect::<Vec<_>>()
        .join("; ");

    info!("[验证登录] Cookie 字符串长度: {} 字符", cookie_str.len());
    info!("[验证登录] Cookie 内容: {}", cookie_str);

    let url = "https://drlives.jd.com/console/homePage/newGetAuthorInfo";
    info!("[验证登录] 请求 URL: {}", url);

    let client = reqwest::Client::new();
    info!("[验证登录] 发起 GET 请求...");

    let response = client
        .get(url)
        .header("Cookie", &cookie_str)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
        .header("Referer", "https://drlives.jd.com/")
        .header("Accept", "application/json, text/plain, */*")
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

    info!(
        "[验证登录] 解析结果: success={}, code={}, error_msg={:?}",
        data.success, data.code, data.error_msg
    );

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
