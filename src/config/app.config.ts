/**
 * 应用配置文件
 * 新项目时，修改此文件来启用/禁用功能模块
 */

export const appConfig = {
  // 应用名称
  name: 'Tauri Vue Template',

  // 功能开关
  features: {
    // 登录认证模块（卡密校验）
    auth: true,
    // Toast 提示
    toast: true,
  },
}

/**
 * 导航菜单配置
 * enabled: false 或删除整行即可隐藏
 */
export const navItems = [
  { path: '/', label: '首页', enabled: true },
  { path: '/demo', label: 'Tauri', enabled: true },
  { path: '/animation', label: '动画', enabled: true },
  { path: '/icons', label: '图标', enabled: true },
  { path: '/hooks', label: 'Hooks', enabled: true },
  // === 以下为业务模块，不需要可设为 false ===
  { path: '/cookie', label: 'Cookie', enabled: true },
  { path: '/screen', label: '投屏', enabled: true },
]
