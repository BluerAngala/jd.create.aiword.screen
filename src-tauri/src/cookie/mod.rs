// Chrome Cookie 读取模块 - 使用 CDP 协议
mod reader;

pub use reader::{get_chrome_profiles, read_chrome_cookies_cdp, ChromeProfile};

use serde::{Deserialize, Serialize};

/// Cookie 数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: Option<i64>,
    pub is_secure: bool,
    pub is_http_only: bool,
}

/// Cookie 读取错误类型
#[derive(Debug, Serialize, Deserialize)]
pub enum CookieError {
    /// Chrome 未安装
    ChromeNotFound,
    /// 浏览器启动失败
    BrowserLaunchFailed(String),
    /// 没有找到 Cookie
    NoCookies,
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for CookieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CookieError::ChromeNotFound => write!(f, "未检测到 Chrome 浏览器"),
            CookieError::BrowserLaunchFailed(msg) => write!(f, "浏览器启动失败: {}", msg),
            CookieError::NoCookies => write!(f, "该域名下没有 Cookie"),
            CookieError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for CookieError {}
