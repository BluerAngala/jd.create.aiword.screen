//! 投屏窗口功能模块

use tauri::{Emitter, Manager};

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
) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    // 如果窗口已存在，先关闭
    if let Some(existing) = app.get_webview_window(&label) {
        let _ = existing.close();
    }

    // 构建窗口 URL（使用 hash 路由兼容性更好）
    let url = format!("/#/screen-content?{}", extra_params.unwrap_or_default());

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
pub async fn close_screen_window(app: tauri::AppHandle, label: String) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&label) {
        window.close().map_err(|e| e.to_string())?;
        // 通知主窗口投屏已关闭
        let _ = app.emit("screen-window-closed", ());
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
