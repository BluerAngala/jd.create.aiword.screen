// Chrome Cookie 读取器 - 使用 CDP 协议
use crate::cookie::{Cookie, CookieError};
use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::storage::GetCookiesParams;
use futures::StreamExt;
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Chrome 浏览器配置文件信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChromeProfile {
    pub id: String,
    pub name: String,
    pub profile_path: String,
}

/// 获取所有 Chrome 浏览器配置文件列表
pub fn get_chrome_profiles() -> Result<Vec<ChromeProfile>, CookieError> {
    let user_data_dir = get_chrome_user_data_dir()?;
    let mut profiles = Vec::new();

    // 读取 Local State 文件获取配置文件信息
    let local_state_path = user_data_dir.join("Local State");
    if local_state_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&local_state_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(profile_info) = json.get("profile").and_then(|p| p.get("info_cache")) {
                    if let Some(obj) = profile_info.as_object() {
                        for (profile_dir, info) in obj {
                            let name = info
                                .get("name")
                                .and_then(|n| n.as_str())
                                .unwrap_or(profile_dir)
                                .to_string();

                            let profile_path = user_data_dir.join(profile_dir);
                            if profile_path.exists() {
                                profiles.push(ChromeProfile {
                                    id: profile_dir.clone(),
                                    name,
                                    profile_path: profile_path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 如果没有从 Local State 读取到，尝试扫描目录
    if profiles.is_empty() {
        // 检查 Default 配置文件
        let default_path = user_data_dir.join("Default");
        if default_path.exists() {
            profiles.push(ChromeProfile {
                id: "Default".to_string(),
                name: "默认".to_string(),
                profile_path: default_path.to_string_lossy().to_string(),
            });
        }

        // 扫描 Profile 1, Profile 2 等
        if let Ok(entries) = std::fs::read_dir(&user_data_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("Profile ") && entry.path().is_dir() {
                    profiles.push(ChromeProfile {
                        id: name.clone(),
                        name: name.clone(),
                        profile_path: entry.path().to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    // 按名称排序
    profiles.sort_by(|a, b| a.name.cmp(&b.name));

    info!("找到 {} 个 Chrome 配置文件", profiles.len());
    Ok(profiles)
}

/// 获取 Chrome 用户数据目录
pub fn get_chrome_user_data_dir() -> Result<PathBuf, CookieError> {
    let local_app_data =
        dirs::data_local_dir().ok_or_else(|| CookieError::Other("无法获取 LocalAppData 目录".to_string()))?;

    let chrome_path = local_app_data.join("Google").join("Chrome").join("User Data");

    if chrome_path.exists() {
        Ok(chrome_path)
    } else {
        Err(CookieError::ChromeNotFound)
    }
}

/// 查找 Chrome 可执行文件路径
pub fn find_chrome_executable() -> Result<PathBuf, CookieError> {
    // 用户安装路径
    if let Some(local_app_data) = dirs::data_local_dir() {
        let user_chrome = local_app_data
            .join("Google")
            .join("Chrome")
            .join("Application")
            .join("chrome.exe");
        if user_chrome.exists() {
            return Ok(user_chrome);
        }
    }

    // 系统安装路径
    let system_paths = [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
    ];

    for path in system_paths {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    Err(CookieError::ChromeNotFound)
}

/// 从 URL 提取域名
pub fn extract_domain(url: &str) -> String {
    let url = url.trim();
    let url = url.strip_prefix("http://").unwrap_or(url);
    let url = url.strip_prefix("https://").unwrap_or(url);
    let url = url.strip_prefix("www.").unwrap_or(url);

    url.split('/').next().unwrap_or(url).to_string()
}

/// 检查 Cookie 域名是否匹配目标域名
fn domain_matches(cookie_domain: &str, target_domain: &str) -> bool {
    let cookie_domain = cookie_domain.trim_start_matches('.');
    let target_domain = target_domain.trim_start_matches('.');

    if cookie_domain == target_domain {
        return true;
    }

    // 检查子域名匹配
    target_domain.ends_with(&format!(".{}", cookie_domain))
        || cookie_domain.ends_with(&format!(".{}", target_domain))
        || target_domain.contains(cookie_domain)
        || cookie_domain.contains(target_domain)
}

/// 使用 CDP 协议读取 Chrome Cookie
pub async fn read_chrome_cookies_cdp(
    domain: &str,
    profile: Option<&str>,
) -> Result<Vec<Cookie>, CookieError> {
    let target_domain = extract_domain(domain);
    let user_data_dir = get_chrome_user_data_dir()?;
    let chrome_exe = find_chrome_executable()?;
    let profile_name = profile.unwrap_or("Default");

    // 配置浏览器
    let config = BrowserConfig::builder()
        .chrome_executable(chrome_exe)
        .user_data_dir(&user_data_dir)
        .arg(format!("--profile-directory={}", profile_name))
        .arg("--headless=new")
        .arg("--disable-gpu")
        .arg("--no-first-run")
        .arg("--disable-extensions")
        .arg("--disable-logging")
        .arg("--log-level=3")
        .build()
        .map_err(|e| CookieError::BrowserLaunchFailed(format!("配置错误: {}", e)))?;

    // 启动浏览器
    let (mut browser, mut handler) = Browser::launch(config)
        .await
        .map_err(|e| CookieError::BrowserLaunchFailed(format!("启动失败: {}", e)))?;

    // 处理浏览器事件
    let handle = tokio::spawn(async move {
        while handler.next().await.is_some() {}
    });

    // 获取所有 Cookie
    let params = GetCookiesParams::builder().build();
    let result = browser
        .execute(params)
        .await
        .map_err(|e| CookieError::Other(format!("获取 Cookie 失败: {}", e)))?;

    let all_cookies = result.cookies.clone();

    // 过滤匹配域名的 Cookie
    let mut cookies: Vec<Cookie> = all_cookies
        .into_iter()
        .filter(|c| domain_matches(&c.domain, &target_domain))
        .map(|c| Cookie {
            name: c.name,
            value: c.value,
            domain: c.domain,
            path: c.path,
            expires: if c.expires > 0.0 { Some(c.expires as i64) } else { None },
            is_secure: c.secure,
            is_http_only: c.http_only,
        })
        .collect();

    // 关闭浏览器
    let _ = browser.close().await;
    handle.abort();

    if cookies.is_empty() {
        return Err(CookieError::NoCookies);
    }

    info!("读取到 {} 个 {} 的 Cookie", cookies.len(), target_domain);

    // 按名称排序
    cookies.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(cookies)
}
