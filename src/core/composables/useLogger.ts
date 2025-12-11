import { trace, debug, info, warn, error, attachConsole } from '@tauri-apps/plugin-log'

// 日志级别
export type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error'

// 前端日志工具，同时输出到控制台和 Tauri 日志系统
export function useLogger(module: string = 'App') {
  const formatMessage = (msg: string) => `[${module}] ${msg}`

  return {
    trace: (msg: string) => {
      console.log(`[TRACE] ${formatMessage(msg)}`)
      trace(formatMessage(msg))
    },
    debug: (msg: string) => {
      console.log(`[DEBUG] ${formatMessage(msg)}`)
      debug(formatMessage(msg))
    },
    info: (msg: string) => {
      console.info(`[INFO] ${formatMessage(msg)}`)
      info(formatMessage(msg))
    },
    warn: (msg: string) => {
      console.warn(`[WARN] ${formatMessage(msg)}`)
      warn(formatMessage(msg))
    },
    error: (msg: string) => {
      console.error(`[ERROR] ${formatMessage(msg)}`)
      error(formatMessage(msg))
    },
  }
}

// 初始化日志系统，将 Tauri 日志转发到浏览器控制台
export async function initLogger() {
  try {
    await attachConsole()
    info('[Logger] 日志系统初始化完成')
  } catch (e) {
    console.warn('日志系统初始化失败，可能在浏览器环境运行', e)
  }
}
