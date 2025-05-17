const fs = require("fs")
const path = require("path")
const fse = require("fs-extra")
const unicodeEmoji = require("unicode-emoji-json")

const fluentRoot = path.resolve(__dirname, "fluentui-emoji", "assets")
const outputPath = path.resolve(__dirname, "soursarc", "assets", "emoji")
const imageBaseDir = "emoji_images"

/**
 * 指定ディレクトリ以下を再帰的に走査し、svg/pngファイルのパス（相対）を配列で返す
 * @param {string} dir ルートディレクトリ
 * @param {string} base 下位ディレクトリ
 * @returns {Array<string>} ファイルの相対パス
 */
function findAllImageFiles(dir, base = "") {
  let results = []
  const items = fs.readdirSync(dir)
  for (const item of items) {
    const fullPath = path.join(dir, item)
    const relPath = path.join(base, item)
    if (fs.statSync(fullPath).isDirectory()) {
      results = results.concat(findAllImageFiles(fullPath, relPath))
    } else if (item.endsWith(".svg") || item.endsWith(".png")) {
      results.push(relPath)
    }
  }
  return results
}

/**
 * メタデータを取得する
 * @param {string} folderName
 * @param {object} info
 * @returns {object}
 */
function getEmojiMeta(folderName, info) {
  const metaJsonPath = path.join(fluentRoot, folderName, "metadata.json")
  let fluentMeta = {}
  if (fs.existsSync(metaJsonPath)) {
    try {
      fluentMeta = JSON.parse(fs.readFileSync(metaJsonPath, "utf8"))
    } catch(e) {
      console.warn("Failed to parse", metaJsonPath)
    }
  }
  return {
    unicode: fluentMeta.unicode || info.hexcode,
    glyph: fluentMeta.glyph || info.emoji,
    cldr: fluentMeta.cldr || info.cldr || info.name,
    group: fluentMeta.group || info.group,
    keywords: fluentMeta.keywords || info.keywords,
    tts: fluentMeta.tts || info.tts,
    // imageは後でセット
  }
}

/**
 * 画像ファイルをコピーし、マップを更新する
 */
function processEmojiImages({
  emojiChar,
  info,
  outEmojiMap,
  outReverseMap,
  emojiMetaMap
}) {
  const name = info.slug.replace(/-/g, "_")
  const folderName = name.replace(/_/g, " ")
  const slugDir = name

  const baseDir = path.join(fluentRoot, folderName)
  if (!fs.existsSync(baseDir)) return

  const variantDirs = fs.readdirSync(baseDir)
  const hasFlat = variantDirs.some(v => v.toLowerCase() === "flat")
  let defaultRegistered = false

  const priorityOrder = ["flat", "default", "high contrast"]
  const sortedVariants = [
    ...priorityOrder.filter(pv => variantDirs.some(v => v.toLowerCase() === pv)),
    ...variantDirs.filter(v =>
      !priorityOrder.includes(v.toLowerCase()))
  ]

  for (const variant of sortedVariants) {
    const variantPath = path.join(baseDir, variant)
    if (!fs.statSync(variantPath).isDirectory()) continue

    const files = findAllImageFiles(variantPath)
    if (files.length === 0) continue

    for (const fileRel of files) {
      const subDirs = fileRel.split(path.sep)
      const file = subDirs.pop()
      const allVariant = [variant, ...subDirs].filter(Boolean)
      const suffix = allVariant.join("_").toLowerCase().replace(/-/g, "_")

      let emojiKey
      if (hasFlat && variant.toLowerCase() === "flat" && !defaultRegistered) {
        emojiKey = `:${name}:`
        defaultRegistered = true
      } else if (!hasFlat && !defaultRegistered &&
        (variant.toLowerCase() === "default" || variant.toLowerCase() === "high contrast")) {
        emojiKey = `:${name}:`
        defaultRegistered = true
      } else {
        emojiKey = (suffix === "flat" || suffix === "default")
          ? `:${name}:`
          : `:${name}_${suffix}:`
      }

      const relativeOutPath = path.join(imageBaseDir, slugDir, ...allVariant, file)
      const absoluteSrc = path.join(variantPath, fileRel)
      const absoluteDst = path.join(outputPath, relativeOutPath)
      fse.ensureDirSync(path.dirname(absoluteDst))
      fse.copyFileSync(absoluteSrc, absoluteDst)
      outEmojiMap[emojiKey] = path.posix.join(imageBaseDir, slugDir, ...allVariant, file)

      // メタデータ
      const meta = getEmojiMeta(folderName, info)
      meta.image = outEmojiMap[emojiKey]
      emojiMetaMap[emojiKey] = meta
      emojiMetaMap[info.emoji] = Object.assign({ emoji_key: emojiKey }, meta)

      // reverseMap
      if (emojiKey === `:${name}:`) {
        outReverseMap[emojiChar] = emojiKey
        if (emojiChar.endsWith('\uFE0F')) {
          outReverseMap[emojiChar.slice(0, -1)] = emojiKey
        } else if (!Object.keys(outReverseMap).includes(emojiChar + '\uFE0F')) {
          outReverseMap[emojiChar + '\uFE0F'] = emojiKey
        }
      }
      for (const [unicodeChar2, info2] of Object.entries(unicodeEmoji)) {
        const candidateSlug = info2.slug.replace(/-/g, "_")
        const expectedKey = (suffix === "flat" || suffix === "default")
          ? `:${candidateSlug}:`
          : `:${candidateSlug}_${suffix}:`
        if (emojiKey === expectedKey) {
          outReverseMap[unicodeChar2] = emojiKey
          if (unicodeChar2.endsWith('\uFE0F')) {
            outReverseMap[unicodeChar2.slice(0, -1)] = emojiKey
          } else if (!Object.keys(outReverseMap).includes(unicodeChar2 + '\uFE0F')) {
            outReverseMap[unicodeChar2 + '\uFE0F'] = emojiKey
          }
        }
      }
    }
  }
}

/**
 * 画像ディレクトリを初期化
 */
function prepareOutputDir() {
  fse.ensureDirSync(outputPath)
  const imagesFullPath = path.join(outputPath, imageBaseDir)
  if (fs.existsSync(imagesFullPath)) {
    fse.removeSync(imagesFullPath)
  }
}

/**
 * マップを書き出す
 */
function writeMaps(outEmojiMap, outReverseMap, emojiMetaMap) {
  fs.writeFileSync(path.join(outputPath, "map.json"), JSON.stringify(outEmojiMap, null, 2))
  console.log("✅ emojiMap generated")

  fs.writeFileSync(path.join(outputPath, "mapReverse.json"), JSON.stringify(outReverseMap, null, 2))
  console.log("✅ emojiMapReverse generated")

  fs.writeFileSync(path.join(outputPath, "emojiMetaMap.json"), JSON.stringify(emojiMetaMap, null, 2))
  console.log("✅ emojiMetaMap generated")
}

/**
 * メイン処理
 */
function main() {
  const outEmojiMap = {}
  const outReverseMap = {}
  const emojiMetaMap = {}

  prepareOutputDir()

  for (const emojiChar of Object.keys(unicodeEmoji)) {
    const info = unicodeEmoji[emojiChar]
    processEmojiImages({
      emojiChar,
      info,
      outEmojiMap,
      outReverseMap,
      emojiMetaMap
    })
  }

  writeMaps(outEmojiMap, outReverseMap, emojiMetaMap)
}

main()
