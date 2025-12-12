/**
 * 生成商品模板 xlsx 文件
 */
import * as XLSX from 'xlsx'
import { writeFileSync } from 'fs'

// 创建工作簿
const workbook = XLSX.utils.book_new()

// 示例数据
const data = [
  ['ID'],
  ['10001234567'],
  ['10001234568'],
  ['10001234569'],
]

// 创建工作表
const worksheet = XLSX.utils.aoa_to_sheet(data)

// 设置列宽
worksheet['!cols'] = [{ wch: 15 }]

// 添加工作表到工作簿
XLSX.utils.book_append_sheet(workbook, worksheet, '商品列表')

// 写入文件
const buffer = XLSX.write(workbook, { type: 'buffer', bookType: 'xlsx' })
writeFileSync('public/商品模板.xlsx', buffer)

console.log('商品模板.xlsx 已生成到 public 目录')
