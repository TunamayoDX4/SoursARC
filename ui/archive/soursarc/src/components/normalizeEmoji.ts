import rawMap from "../assets/emoji/mapReverse.json"

const reverseMap = rawMap as Record<string, string>

// すべての絵文字キーを「長い順」に並べて正規表現パターンにする
const emojiList = Object.keys(reverseMap)
emojiList.sort((a, b) => b.length - a.length) // 長い順（複数文字優先）
const emojiRegex = new RegExp(emojiList.map(s => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|'), "gu")

// Unicode Variation Selector-16 (U+FE0F)を除去
function removeVS16(str: string): string {
  return str.replace(/\uFE0F/g, "")
}

export function normalizeEmoji(text: string): string {
  return text.replace(emojiRegex, match => {
    // まずそのまま
    if (reverseMap[match]) return reverseMap[match]
    // バリアント除去版で
    const noVS = removeVS16(match)
    if (reverseMap[noVS]) return reverseMap[noVS]
    return match
  })
}
