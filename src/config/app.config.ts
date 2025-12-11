/**
 * 应用配置文件
 * 新项目时，修改此文件来启用/禁用功能模块
 */

export const appConfig = {
  // 应用名称
  name: '京东直播助手',

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
]
