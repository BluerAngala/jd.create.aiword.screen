/**
 * 图标生成脚本
 * 从 public/logo.png 生成 Tauri 所需的所有图标
 * 使用方法: node script/generate-icons.mjs
 */

import sharp from 'sharp'
import pngToIco from 'png-to-ico'
import { writeFileSync, existsSync, mkdirSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const rootDir = join(__dirname, '..')
const sourceImage = join(rootDir, 'public/logo.png')
const iconsDir = join(rootDir, 'src-tauri/icons')

// Tauri 需要的图标尺寸
const pngSizes = [
  { name: '32x32.png', size: 32 },
  { name: '128x128.png', size: 128 },
  { name: '128x128@2x.png', size: 256 },
  { name: 'icon.png', size: 512 },
  // Windows Store 图标
  { name: 'Square30x30Logo.png', size: 30 },
  { name: 'Square44x44Logo.png', size: 44 },
  { name: 'Square71x71Logo.png', size: 71 },
  { name: 'Square89x89Logo.png', size: 89 },
  { name: 'Square107x107Logo.png', size: 107 },
  { name: 'Square142x142Logo.png', size: 142 },
  { name: 'Square150x150Logo.png', size: 150 },
  { name: 'Square284x284Logo.png', size: 284 },
  { name: 'Square310x310Logo.png', size: 310 },
  { name: 'StoreLogo.png', size: 50 },
]

// ICO 文件需要的尺寸
const icoSizes = [16, 24, 32, 48, 64, 128, 256]

async function generateIcons() {
  // 检查源文件是否存在
  if (!existsSync(sourceImage)) {
    console.error('❌ 错误: 未找到 public/logo.png')
    console.log('请将你的 logo 图片放到 public/logo.png（建议 512x512 或更大）')
    process.exit(1)
  }

  // 确保输出目录存在
  if (!existsSync(iconsDir)) {
    mkdirSync(iconsDir, { recursive: true })
  }

  console.log('🎨 开始生成图标...\n')

  // 生成 PNG 图标
  for (const { name, size } of pngSizes) {
    const outputPath = join(iconsDir, name)
    await sharp(sourceImage)
      .resize(size, size, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
      .png()
      .toFile(outputPath)
    console.log(`✅ ${name} (${size}x${size})`)
  }

  // 生成 ICO 文件（Windows 图标）
  const icoBuffers = await Promise.all(
    icoSizes.map((size) =>
      sharp(sourceImage)
        .resize(size, size, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
        .png()
        .toBuffer()
    )
  )
  const icoBuffer = await pngToIco(icoBuffers)
  writeFileSync(join(iconsDir, 'icon.ico'), icoBuffer)
  console.log(`✅ icon.ico (多尺寸)`)

  // 生成 ICNS 文件（macOS 图标）- 使用 PNG 作为替代
  // 注意: 真正的 ICNS 需要专门工具，这里用 512x512 PNG 替代
  // macOS 构建时 Tauri 会自动处理
  await sharp(sourceImage)
    .resize(512, 512, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .png()
    .toFile(join(iconsDir, 'icon.icns'))
  console.log(`✅ icon.icns (512x512 PNG 替代)`)

  console.log('\n🎉 图标生成完成！')
  console.log(`📁 输出目录: ${iconsDir}`)
}

generateIcons().catch((err) => {
  console.error('❌ 生成失败:', err.message)
  process.exit(1)
})
