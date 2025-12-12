import * as XLSX from 'xlsx'
import { readFileSync } from 'fs'

const filePath = './test/test.xlsx'
const data = readFileSync(filePath)
const workbook = XLSX.read(data, { type: 'buffer' })

const firstSheetName = workbook.SheetNames[0]
const worksheet = workbook.Sheets[firstSheetName]

// 获取工作表范围
const range = XLSX.utils.decode_range(worksheet['!ref'] || 'A1')
console.log('工作表范围:', worksheet['!ref'])
console.log('行数:', range.e.r + 1)

// 转换为 JSON
const jsonData = XLSX.utils.sheet_to_json(worksheet, { header: 1 })
console.log('JSON 数据行数:', jsonData.length)

// 检查前几行和最后几行
console.log('前5行:', jsonData.slice(0, 5))
console.log('最后5行:', jsonData.slice(-5))
