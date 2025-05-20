import { EmojiText } from './components/EmojiText'
import { normalizeEmoji } from './components/normalizeEmoji'
import { SamplePopover } from './components/SamplePopover'
import { CommonDropdown } from './components/lib_jsx/CommonPulldown'
import { CommonWindow } from './components/lib_jsx/CommonWindow/CommonWindow'

function Header() {
  return (
    <header>
      <CommonDropdown
        items={[
          { label: 'メニュー1', onSelect: () => console.log('メニュー1') },
          { label: 'メニュー2', onSelect: () => console.log('メニュー2') },
          { label: 'メニュー3', onSelect: () => console.log('メニュー3') },
        ]}
        triggerLabel="メニュー"
      />
      <CommonDropdown
        items={[
          { label: 'メニュー1', onSelect: () => console.log('メニュー1') },
          { label: 'メニュー2', onSelect: () => console.log('メニュー2') },
          { label: 'メニュー3', onSelect: () => console.log('メニュー3') },
        ]}
        triggerLabel="メニュー"
      />
    </header>
  )
}

function MainNav() {
  return (
    <nav>
      あああ
      <EmojiText>{teststring + ':yellow_heart:'}</EmojiText>
    </nav>
  )
}

function MainSection() {
  return (
    <section>
      <h1>SoursARCへようこそ！</h1>
      <EmojiText>{teststring + ':yellow_heart:'}</EmojiText>
      <CommonWindow
        sizeMin="10em, 8em"
        sizeMax="45em, 20em"
        sizeDefault="25em, 8em"
        title="ウィンドウタイトル"
        children={
          <div>
            <p>ウィンドウの中身</p> 
          </div>
        }
      />
      <SamplePopover />
    </section>
  )
}

const teststring = normalizeEmoji("こんにちは💪🙂🚀💛⚙🌌⭐🤩");

function App() {
  return (
    <section id="app">
      <Header />
      <main>
        <MainNav />
        <MainSection />
      </main>
    </section>
  )
}

export default App