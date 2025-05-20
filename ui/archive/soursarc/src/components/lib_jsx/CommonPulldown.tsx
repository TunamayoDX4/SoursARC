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
          className='dropdown-content'
          style={{
            boxShadow: '0 2px 8px #0002',
          }}
        >
          {items.map((item, i) => (
            <DropdownMenu.Item
              className='dropdown-item'
              key={i}
              onSelect={item.onSelect}
            >
              {item.label}
            </DropdownMenu.Item>
          ))}
        </DropdownMenu.Content>
      </DropdownMenu.Portal>
    </DropdownMenu.Root>
  )
}