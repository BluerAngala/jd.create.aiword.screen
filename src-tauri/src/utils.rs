//! 通用工具函数模块

use log::info;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use tauri::Manager;

use crate::cookie::Cookie;

/// 获取机器码（基于系统信息生成唯一标识）
#[tauri::command]
pub fn get_machine_code() -> String {
    let mut hasher = DefaultHasher::new();

    // 获取主机名
    if let Ok(hostname) = hostname::get() {
        hostname.to_string_lossy().hash(&mut hasher);
    }

    // 获取用户名
    if let Ok(username) = std::env::var("USERNAME").or_else(|_| std::env::var("USER")) {
        username.hash(&mut hasher);
    }

    // 获取系统信息
    std::env::consts::OS.hash(&mut hasher);
    std::env::consts::ARCH.hash(&mut hasher);

    // 获取 home 目录路径作为额外标识
    if let Some(home) = dirs::home_dir() {
        home.to_string_lossy().hash(&mut hasher);
    }

    let hash = hasher.finish();
    format!("{:016x}", hash)
}

/// 获取程序运行目录
#[tauri::command]
pub fn get_app_dir() -> Result<String, String> {
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "无法获取程序目录".to_string())?;

    Ok(exe_dir.to_string_lossy().to_string())
}

/// 保存 Cookie 到本地文件
#[tauri::command]
pub async fn save_cookies_to_file(
    app: tauri::AppHandle,
    cookies: Vec<Cookie>,
    filename: String,
) -> Result<String, String> {
    use std::fs;

    // 获取程序运行根目录
    let app_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("获取程序目录失败: {}", e))?;

    // 使用可执行文件所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(app_dir);

    // 创建 cookies 文件夹
    let cookies_dir = exe_dir.join("cookies");
    if !cookies_dir.exists() {
        fs::create_dir_all(&cookies_dir).map_err(|e| format!("创建 cookies 目录失败: {}", e))?;
    }

    let file_path = cookies_dir.join(&filename);

    // 将 Cookie 转换为 JSON 格式
    let json_content =
        serde_json::to_string_pretty(&cookies).map_err(|e| format!("序列化 Cookie 失败: {}", e))?;

    // 写入文件
    fs::write(&file_path, &json_content).map_err(|e| format!("写入文件失败: {}", e))?;

    info!("Cookie 已保存到: {:?}", file_path);
    Ok(file_path.to_string_lossy().to_string())
}

// 示例：带结构体的 Command
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub age: u32,
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    info!("greet 被调用，参数: {}", name);
    format!("你好，{}！来自 Rust 后端的问候。", name)
}

#[tauri::command]
pub fn get_user_info() -> UserInfo {
    UserInfo {
        name: "测试用户".to_string(),
        age: 18,
    }
}

// 示例：异步 Command
#[tauri::command]
pub async fn async_operation(delay_ms: u64) -> Result<String, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    Ok(format!("异步操作完成，延迟 {} 毫秒", delay_ms))
}

/// 通用 HTTP POST 请求（用于绕过 CORS 限制）
#[tauri::command]
pub async fn http_post(url: String, body: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    Ok(text)
}

/// 保存直播场次数据到文件
#[tauri::command]
pub async fn save_live_sessions(sessions_json: String) -> Result<String, String> {
    use std::fs;

    // 获取可执行文件所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "无法获取程序目录".to_string())?;

    // 创建 data 文件夹
    let data_dir = exe_dir.join("data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| format!("创建 data 目录失败: {}", e))?;
    }

    let file_path = data_dir.join("live_sessions.json");

    // 写入文件
    fs::write(&file_path, &sessions_json).map_err(|e| format!("写入文件失败: {}", e))?;

    info!("直播场次数据已保存到: {:?}", file_path);
    Ok(file_path.to_string_lossy().to_string())
}

/// 从文件加载直播场次数据
#[tauri::command]
pub async fn load_live_sessions() -> Result<String, String> {
    use std::fs;

    // 获取可执行文件所在目录
    let exe_dir = std::env::current_exe()
        .map_err(|e| format!("获取可执行文件路径失败: {}", e))?
        .parent()
        .map(|p| p.to_path_buf())
        .ok_or_else(|| "无法获取程序目录".to_string())?;

    let file_path = exe_dir.join("data").join("live_sessions.json");

    // 如果文件不存在，返回空数组
    if !file_path.exists() {
        return Ok("[]".to_string());
    }

    // 读取文件
    let content = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;

    info!("直播场次数据已从文件加载: {:?}", file_path);
    Ok(content)
}
