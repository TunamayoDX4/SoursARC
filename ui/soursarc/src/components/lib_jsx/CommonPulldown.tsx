import * as DropdownMenu from '@radix-ui/react-dropdown-menu'

/**
 * 共通プルダウンメニュー
 *
 * ## Summary
 * 任意の項目を渡して使える共通ドロップダウン
 *
 * ## Argument
 * - `items`: { label: string, onSelect: () => void }[] ドロップダウンの項目
 * - `triggerLabel`: string トリガーボタンのラベル
 *
 * ## Return value
 * - JSX.Element
 *   - ドロップダウンUI
 */
export function CommonDropdown({
  items,
  triggerLabel = 'メニュー',
}: {
  items: { label: string; onSelect: () => void }[]
  triggerLabel?: string
}) {
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger asChild>
        <button>{triggerLabel} ▼</button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Portal>
        <DropdownMenu.Content
          sideOffset={4}
          style={{
            minWidth: 120,
            background: 'var(--col-bg-pri)',
            color: 'var(--col-font-pri)',
            border: '1px solid var(--col-border-sec)',
            borderRadius: 8,
            boxShadow: '0 2px 8px #0002',
            padding: '0.25em 0',
            zIndex: 2000,
          }}
        >
          {items.map((item, i) => (
            <DropdownMenu.Item
              key={i}
              onSelect={item.onSelect}
              style={{
                padding: '0.5em 1em',
                cursor: 'pointer',
                outline: 'none',
              }}
            >
              {item.label}
            </DropdownMenu.Item>
          ))}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  )
}