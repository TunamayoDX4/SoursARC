import * as Dialog from '@radix-ui/react-dialog'
import { useLayoutEffect, useRef, useState } from 'react'

/**
 * "幅, 高さ"形式の文字列を分割して返す
 */
function parseSizePair(sizeStr: string | null | undefined, fallback: string): { width: string; height: string } {
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

/**
 * px/%,emなどをpxに変換
 */
function toPx(val: string, base: number): number {
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

/**
 * ドラッグ移動用フック
 */
function useDraggable(dialogRef: React.RefObject<HTMLDivElement | null>) {
  function onMouseDown(e: React.MouseEvent) {
    const dialog = dialogRef.current
    if (!dialog) return
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
  return onMouseDown
}

/**
 * リサイズ用フック
 */
function useResizable(
  dialogRef: React.RefObject<HTMLDivElement | null>,
  getWidgetSizes: () => any,
  setSize: (s: { width: string, height: string }) => void
) {
  function onResizeMouseDown(e: React.MouseEvent) {
    e.stopPropagation()
    const dialog = dialogRef.current
    if (!dialog) return
    const rect = dialog.getBoundingClientRect()
    dialog.style.transform = ''
    dialog.style.left = rect.left + 'px'
    dialog.style.top = rect.top + 'px'
    const startX = e.clientX
    const startY = e.clientY
    const startWidth = rect.width
    const startHeight = rect.height
    const startLeft = rect.left
    const startTop = rect.top
    const sizes = getWidgetSizes()

    const minWidthPx = toPx(sizes.min.width, window.innerWidth)
    const minHeightPx = toPx(sizes.min.height, window.innerHeight)
    const maxWidthPx = toPx(sizes.max.width, window.innerWidth)
    const maxHeightPx = toPx(sizes.max.height, window.innerHeight)

    function onMouseMove(ev: MouseEvent) {
      if (!dialog) return
      let newWidth = Math.max(minWidthPx, Math.min(startWidth + (ev.clientX - startX), maxWidthPx))
      let newHeight = Math.max(minHeightPx, Math.min(startHeight + (ev.clientY - startY), maxHeightPx))
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
  return onResizeMouseDown
}

/**
 * 共通ウィンドウ
 */
export function CommonWindow({
  sizeMin = "10em, 5em",
  sizeMax = "100%, 100%",
  sizeDefault = null,
  title = "ウィンドウタイトル",
  children,
}: {
  sizeMin?: string
  sizeMax?: string
  sizeDefault?: string | null
  title?: string
  children?: React.ReactNode
}) {
  const dialogRef = useRef<HTMLDivElement>(null)
  const [size, setSize] = useState<{ width: string; height: string }>(() =>
    parseSizePair(sizeDefault, "32em, 20em")
  )
  const [open, setOpen] = useState(false)

  function getWidgetSizes() {
    return {
      min: parseSizePair(sizeMin, "10em, 5em"),
      max: parseSizePair(sizeMax, "100%, 100%"),
    }
  }

  const onMouseDown = useDraggable(dialogRef)
  const onResizeMouseDown = useResizable(dialogRef, getWidgetSizes, setSize)
  const widgetSizes = getWidgetSizes()

  useLayoutEffect(() => {
    if (open) {
      const dialog = dialogRef.current
      if (dialog) {
        // すでに絶対座標(left/top)が設定されていればそれを維持
        // まだなら中央配置
        const left = dialog.style.left
        const top = dialog.style.top
        if (!left || !top || left === '' || top === '') {
          dialog.style.width = size.width
          dialog.style.height = size.height
          const rect = dialog.getBoundingClientRect()
          const winWidth = rect.width
          const winHeight = rect.height
          dialog.style.left = window.innerWidth / 2 - winWidth / 2 + 'px'
          dialog.style.top = window.innerHeight / 2 - winHeight / 2 + 'px'
          dialog.style.transform = ''
        } else {
          // サイズだけ更新、位置は維持
          dialog.style.width = size.width
          dialog.style.height = size.height
          dialog.style.transform = ''
        }
      }
    }
  }, [open, size.width, size.height])

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
            <Dialog.Title>{title}</Dialog.Title>
            <Dialog.Close asChild>
              <button>×</button>
            </Dialog.Close>
          </header>
          <main>
            {children}
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