import * as Dialog from '@radix-ui/react-dialog'
import { useLayoutEffect, useState, useRef } from 'react'

/**
 * DOMからサイズ指定値を取得する関数
 * @param selector クラス名セレクタ
 * @param fallback デフォルト値（"width, height"形式）
 * @returns { width: string, height: string }
 */
function getSizePairFromDOM(
  selector: string,
  fallback: string
): { width: string; height: string } {
  const el = document.querySelector(selector)
  if (el && el.textContent && el.textContent.trim()) {
    const [w, h] = el.textContent.trim().split(/[,\s]+/).map((s) => s.trim()).filter(Boolean)
    return {
      width: w || '8em',
      height: h || '20em',
    }
  }
  const [w, h] = fallback.split(/[,\s]+/).map((s) => s.trim()).filter(Boolean)
  return {
    width: w || '8em',
    height: h || '20em',
  }
}

/**
 * "幅, 高さ"形式の文字列を分割して返す
 * @param sizeStr サイズ文字列（例: "12em, 8em"）
 * @param fallback デフォルト値
 * @returns { width: string, height: string }
 */
function parseSizePair(sizeStr: string | null, fallback: string): { width: string; height: string } {
  if (sizeStr && typeof sizeStr === "string" && sizeStr.trim()) {
    const [w, h] = sizeStr.trim().split(/[,\s]+/).map((s) => s.trim()).filter(Boolean)
    return {
      width: w || fallback.split(",")[0].trim(),
      height: h || fallback.split(",")[1]?.trim() || fallback.split(",")[0].trim(),
    }
  }
  const [fw, fh] = fallback.split(/[,\s]+/).map((s) => s.trim()).filter(Boolean)
  return {
    width: fw || "8em",
    height: fh || "20em",
  }
}

export function DraggableWindow({
  sizeMin = "10em, 5em",
  sizeMax = "100%, 100%",
  sizeDefault = null,
}: {
  sizeMin?: string
  sizeMax?: string
  sizeDefault?: string | null
}) {
  const dialogRef = useRef<HTMLDivElement>(null)
  // 初回のみデフォルトサイズを反映
  const [size, setSize] = useState<{ width: string; height: string }>(() =>
    parseSizePair(sizeDefault, "32em, 20em")
  )
  const [open, setOpen] = useState(false)

  // サイズ指定値を引数から取得
  function getWidgetSizes() {
    return {
      min: parseSizePair(sizeMin, "10em, 5em"),
      max: parseSizePair(sizeMax, "100%, 100%"),
    }
  }

  // Dialogが表示された直後に中央配置（サイズはstateで固定）
  useLayoutEffect(() => {
    if (open) {
      const dialog = dialogRef.current
      if (dialog) {
        dialog.style.width = size.width
        dialog.style.height = size.height
        // 左上基準で中央配置
        const rect = dialog.getBoundingClientRect()
        const winWidth = rect.width
        const winHeight = rect.height
        dialog.style.left = window.innerWidth / 2 - winWidth / 2 + 'px'
        dialog.style.top = window.innerHeight / 2 - winHeight / 2 + 'px'
        dialog.style.transform = ''
      }
    }
  }, [open, size.width, size.height])

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

  // リサイズ処理（右下ハンドル基準でリサイズ、左上は絶対に動かさない）
  function onResizeMouseDown(e: React.MouseEvent) {
    e.stopPropagation()
    const dialog = dialogRef.current
    if (!dialog) return
    // transform解除して絶対座標に切り替え
    const rect = dialog.getBoundingClientRect()
    dialog.style.transform = ''
    // left/topを絶対座標で固定
    dialog.style.left = rect.left + 'px'
    dialog.style.top = rect.top + 'px'

    const startX = e.clientX
    const startY = e.clientY
    const startWidth = rect.width
    const startHeight = rect.height
    const startLeft = rect.left
    const startTop = rect.top
    const sizes = getWidgetSizes()

    function parseSize(val: string, base: number): number {
      if (val.endsWith('em')) {
        const em = parseFloat(val)
        return em * parseFloat(getComputedStyle(document.documentElement).fontSize)
      }
      if (val.endsWith('%')) {
        const percent = parseFloat(val)
        return base * percent / 100
      }
      if (val.endsWith('px')) {
        return parseFloat(val)
      }
      return parseFloat(val)
    }

    const minWidthPx = parseSize(sizes.min.width, window.innerWidth)
    const minHeightPx = parseSize(sizes.min.height, window.innerHeight)
    const maxWidthPx = parseSize(sizes.max.width, window.innerWidth)
    const maxHeightPx = parseSize(sizes.max.height, window.innerHeight)

    function onMouseMove(ev: MouseEvent) {
      if (!dialog) return
      // 右下ハンドル基準でリサイズ（左上は絶対に動かさない）
      let newWidth = Math.max(minWidthPx, Math.min(startWidth + (ev.clientX - startX), maxWidthPx))
      let newHeight = Math.max(minHeightPx, Math.min(startHeight + (ev.clientY - startY), maxHeightPx))
      // 右下が画面外に出ないように制限
      const maxRight = window.innerWidth
      const maxBottom = window.innerHeight
      if (startLeft + newWidth > maxRight) {
        newWidth = maxRight - startLeft
      }
      if (startTop + newHeight > maxBottom) {
        newHeight = maxBottom - startTop
      }
      dialog.style.width = newWidth + 'px'
      dialog.style.height = newHeight + 'px'
      // left/topは絶対にstartLeft/startTopのまま
      dialog.style.left = startLeft + 'px'
      dialog.style.top = startTop + 'px'
      setSize({ width: newWidth + 'px', height: newHeight + 'px' })
    }
    function onMouseUp() {
      window.removeEventListener('mousemove', onMouseMove)
      window.removeEventListener('mouseup', onMouseUp)
    }
    window.addEventListener('mousemove', onMouseMove)
    window.addEventListener('mouseup', onMouseUp)
  }

  const widgetSizes = getWidgetSizes()

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
            minWidth: widgetSizes.min.width,
            minHeight: widgetSizes.min.height,
            maxWidth: widgetSizes.max.width,
            maxHeight: widgetSizes.max.height,
            width: size.width,
            height: size.height,
            zIndex: 1000,
            position: 'fixed',
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
          <div
            className="resize-handle"
            onMouseDown={onResizeMouseDown}
            style={{ userSelect: 'none' }}
          ></div>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  )
}