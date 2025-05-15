import * as Dialog from '@radix-ui/react-dialog'
import { useLayoutEffect, useState, useRef } from 'react'

export function DraggableWindow() {
  const dialogRef = useRef<HTMLDivElement>(null)
  const [open, setOpen] = useState(false)

  // Dialogが表示された直後に中央配置
  useLayoutEffect(() => {
    if (open) {
      const dialog = dialogRef.current
      if (dialog) {
        const rect = dialog.getBoundingClientRect()
        const winWidth = rect.width
        const winHeight = rect.height
        dialog.style.left = window.innerWidth / 2 - winWidth / 2 + 'px'
        dialog.style.top = window.innerHeight / 2 - winHeight / 2 + 'px'
        dialog.style.transform = '' // transformが残ってたらリセット
      }
    }
  }, [open])

  // ドラッグ処理（画面からはみ出さないように制御）
  function onMouseDown(e: React.MouseEvent) {
    const dialog = dialogRef.current
    if (!dialog) return
    // transform解除して絶対座標に切り替え
    const rect = dialog.getBoundingClientRect()
    dialog.style.transform = ''
    dialog.style.left = rect.left + 'px'
    dialog.style.top = rect.top + 'px'

    const startX = e.clientX
    const startY = e.clientY
    const origLeft = rect.left
    const origTop = rect.top
    const winWidth = rect.width
    const winHeight = rect.height

    function onMouseMove(ev: MouseEvent) {
      if (!dialog) return
      let newLeft = origLeft + (ev.clientX - startX)
      let newTop = origTop + (ev.clientY - startY)
      const maxLeft = window.innerWidth - winWidth
      const maxTop = window.innerHeight - winHeight
      newLeft = Math.max(0, Math.min(newLeft, maxLeft))
      newTop = Math.max(0, Math.min(newTop, maxTop))
      dialog.style.left = newLeft + 'px'
      dialog.style.top = newTop + 'px'
    }
    function onMouseUp() {
      window.removeEventListener('mousemove', onMouseMove)
      window.removeEventListener('mouseup', onMouseUp)
    }
    window.addEventListener('mousemove', onMouseMove)
    window.addEventListener('mouseup', onMouseUp)
  }

  return (
    <Dialog.Root open={open} onOpenChange={setOpen}>
      <Dialog.Trigger asChild>
        <button>ウィンドウを開く</button>
      </Dialog.Trigger>
      <Dialog.Portal>
        <Dialog.Content
          ref={dialogRef}
          className="widget-window"
          style={{
            minWidth: 320,
            zIndex: 1000,
            position: 'fixed',
            // 初期値は中央付近（useLayoutEffectで上書き）
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
          }}
        >
          <header onMouseDown={onMouseDown}>
            <Dialog.Title>ウィンドウタイトル</Dialog.Title>
            <Dialog.Close asChild>
              <button>×</button>
            </Dialog.Close>
          </header>
          <main style={{ padding: '1em' }}>
            ここにウィンドウの内容を書くよ！
          </main>
          <footer></footer>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  )
}