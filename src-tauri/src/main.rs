// 防止在 Windows 发布版本中出现额外的控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    jd_create_aiword_screen_lib::run()
}
