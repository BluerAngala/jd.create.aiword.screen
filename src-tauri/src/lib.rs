use serde::{Deserialize, Serialize};

// 示例：Tauri Command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好，{}！来自 Rust 后端的问候。", name)
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_user_info,
            async_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
