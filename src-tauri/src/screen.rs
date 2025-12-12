//! 投屏窗口功能模块

use tauri::{Emitter, Manager};

/// 窗口状态信息
#[derive(serde::Serialize)]
pub struct WindowState {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 获取窗口当前位置和尺寸（逻辑像素）
#[tauri::command]
pub async fn get_window_state(
    app: tauri::AppHandle,
    label: String,
) -> Result<Option<WindowState>, String> {
    if let Some(window) = app.get_webview_window(&label) {
        let scale = app
            .primary_monitor()
            .ok()
            .flatten()
            .map(|m| m.scale_factor())
            .unwrap_or(1.0);

        let pos = window.outer_position().ok();
        let size = window.inner_size().ok();

        if let (Some(p), Some(s)) = (pos, size) {
            return Ok(Some(WindowState {
                x: p.x as f64 / scale,
                y: p.y as f64 / scale,
                width: s.width as f64 / scale,
                height: s.height as f64 / scale,
            }));
        }
    }
    Ok(None)
}

/// 创建投屏窗口（独立窗口，OBS 可捕获）
#[tauri::command]
pub async fn create_screen_window(
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
    x: Option<f64>,
    y: Option<f64>,
) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // 根据 label 决定路由路径
    let route = match label.as_str() {
        "screen-countdown" => "screen-countdown",
        "screen-script" => "screen-script",
        _ => "screen-content",
    };

    // 构建窗口 URL（使用 hash 路由兼容性更好）
    let url = format!("/#/{}?{}", route, extra_params.unwrap_or_default());

    // 如果窗口已存在，先关闭
    if let Some(existing) = app.get_webview_window(&label) {
        let _ = existing.close();
    }

    // 创建独立窗口
    let mut builder = WebviewWindowBuilder::new(&app, &label, WebviewUrl::App(url.into()))
        .title(&title)
        .inner_size(width, height)
        .transparent(transparent)
        .always_on_top(always_on_top)
        .decorations(decorations)
        .resizable(resizable)
        .skip_taskbar(false)
        .visible(true)
        .focused(true);

    // 如果提供了坐标则设置位置
    if let (Some(px), Some(py)) = (x, y) {
        builder = builder.position(px, py);
    }

    builder.build().map_err(|e| e.to_string())?;

    Ok(())
}

/// 关闭投屏窗口
#[tauri::command]
pub async fn close_screen_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
        // 通知主窗口投屏已关闭（发送带标签的事件）
        let _ = app.emit(&format!("{}-closed", label), ());
    }
    Ok(())
}

/// 开始拖动指定窗口
#[tauri::command]
pub async fn start_dragging_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&label) {
        window.start_dragging().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 读取本地图片文件并返回 base64 编码
#[tauri::command]
pub async fn read_image_as_base64(path: String) -> Result<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use std::fs;

    // 读取文件
    let data = fs::read(&path).map_err(|e| format!("读取文件失败: {}", e))?;

    // 根据扩展名确定 MIME 类型
    let mime = match path.to_lowercase() {
        p if p.ends_with(".png") => "image/png",
        p if p.ends_with(".jpg") || p.ends_with(".jpeg") => "image/jpeg",
        p if p.ends_with(".webp") => "image/webp",
        p if p.ends_with(".gif") => "image/gif",
        _ => "image/png",
    };

    // 编码为 base64 data URL
    let base64_str = STANDARD.encode(&data);
    Ok(format!("data:{};base64,{}", mime, base64_str))
}
