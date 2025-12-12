//! Tauri 应用入口模块

use log::info;

// 功能模块
mod cookie;
mod jd;
mod screen;
mod utils;

// 重新导出供其他模块使用
pub use cookie::{get_chrome_profiles, read_chrome_cookies_cdp, Cookie};

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
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // utils 模块
            utils::greet,
            utils::get_user_info,
            utils::async_operation,
            utils::get_machine_code,
            utils::get_app_dir,
            utils::save_cookies_to_file,
            utils::http_post,
            // cookie 模块
            cookie::get_browser_profiles,
            cookie::read_chrome_cookies,
            // jd 模块
            jd::verify_jd_login,
            jd::get_recent_live_rooms,
            jd::create_live_room,
            jd::upload_sku,
            jd::add_sku_to_bag,
            jd::get_live_general_data,
            jd::get_h5_url,
            jd::start_explain,
            jd::end_explain,
            jd::get_cover_images,
            // screen 模块
            screen::create_screen_window,
            screen::close_screen_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    info!("应用启动完成");
}
