use log::info;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod cookie;

use cookie::{read_chrome_cookies_cdp, Cookie};

// 示例：Tauri Command
#[tauri::command]
fn greet(name: &str) -> String {
    info!("greet 被调用，参数: {}", name);
    format!("你好，{}！来自 Rust 后端的问候。", name)
}

/// 获取机器码（基于系统信息生成唯一标识）
#[tauri::command]
fn get_machine_code() -> String {
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

/// 读取 Chrome Cookie 命令（使用 CDP 协议）
#[tauri::command]
async fn read_chrome_cookies(domain: String, profile: Option<String>) -> Result<Vec<Cookie>, String> {
    read_chrome_cookies_cdp(&domain, profile.as_deref())
        .await
        .map_err(|e| e.to_string())
}

// 示例：带结构体的 Command
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub name: String,
    pub age: u32,
}

#[tauri::command]
fn get_user_info() -> UserInfo {
    UserInfo {
        name: "测试用户".to_string(),
        age: 18,
    }
}

// 示例：异步 Command
#[tauri::command]
async fn async_operation(delay_ms: u64) -> Result<String, String> {
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
    Ok(format!("异步操作完成，延迟 {} 毫秒", delay_ms))
}

/// 创建投屏窗口（独立窗口，OBS 可捕获）
#[tauri::command]
async fn create_screen_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    width: f64,
    height: f64,
    transparent: bool,
    always_on_top: bool,
    decorations: bool,
    resizable: bool,
    _background_color: String,
    extra_params: Option<String>,
) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // 构建窗口 URL
    let url = format!("/screen-content?{}", extra_params.unwrap_or_default());

    // 创建独立窗口
    WebviewWindowBuilder::new(&app, &label, WebviewUrl::App(url.into()))
        .title(&title)
        .inner_size(width, height)
        .transparent(transparent)
        .always_on_top(always_on_top)
        .decorations(decorations)
        .resizable(resizable)
        .skip_taskbar(false)
        .visible(true)
        .focused(true)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// 关闭投屏窗口
#[tauri::command]
async fn close_screen_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    use tauri::Manager;
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 禁用 WebView2 GPU 加速，让 OBS 可以正常捕获窗口内容
    // SAFETY: 在程序启动时设置环境变量，此时只有主线程运行
    unsafe {
        std::env::set_var("WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS", "--disable-gpu");
    }

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_info,
            async_operation,
            read_chrome_cookies,
            create_screen_window,
            close_screen_window,
            get_machine_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    info!("应用启动完成");
}
