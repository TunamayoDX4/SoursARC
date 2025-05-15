import * as Popover from '@radix-ui/react-popover'

/**
 * サンプルのPopoverコンポーネント
 *
 * ## Summary
 * ボタンを押すとポップアップが表示されるサンプル。
 *
 * ## Return value
 * - JSX.Element
 *   - ポップオーバーUI
 */
export function SamplePopover() {
  return (
    <Popover.Root>
      <Popover.Trigger asChild>
        <button>ポップアップを開く</button>
      </Popover.Trigger>
      <Popover.Portal>
        <Popover.Content style={{
          background: "#fff",
          border: "1px solid #ccc",
          borderRadius: "8px",
          padding: "1rem",
          boxShadow: "0 2px 8px #0002"
        }}>
          ここに内容を書くよ！<br />
          <Popover.Close asChild>
            <button style={{marginTop: "1rem"}}>閉じる</button>
          </Popover.Close>
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  )
}