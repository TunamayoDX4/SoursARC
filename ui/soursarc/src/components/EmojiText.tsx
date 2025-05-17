// src/components/EmojiText.tsx
import React from "react"
import rawMap from "../assets/emoji/map.json"

const emojiMap = rawMap as Record<string, string>
const emojiRegex = /(:[a-z0-9_ ]+:)/gi

/**
 * `:emoji:` 記法を Fluent Emoji の画像に置換して表示する React コンポーネント
 */
export const EmojiText: React.FC<{ children: string }> = ({ children }) => {
  const parts = typeof children === "string" ? children.split(emojiRegex) : [children]

  return (
    <span>
      {parts.map((part, index) => {
        const path = emojiMap[part]
        if (path) {
          return (
            <img
              key={index}
              src={`./src/assets/emoji/${path}`}
              alt={part}
              style={{
                width: "1em",
                height: "1em",
                verticalAlign: "middle",
                display: "inline-block",
              }}
            />
          )
        }
        return <React.Fragment key={index}>{part}</React.Fragment>
      })}
    </span>
  )
}
