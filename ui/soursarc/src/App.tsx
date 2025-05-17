import { EmojiText } from './components/EmojiText'
import { normalizeEmoji } from './components/normalizeEmoji'
import { SamplePopover } from './components/SamplePopover'
import { DraggableWindow } from './components/SampleWindowWidget'
import { CommonDropdown } from './components/lib_jsx/CommonPulldown'

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
      <DraggableWindow
        sizeMin="25em, 15em"
        sizeMax="100%, 100%"
        sizeDefault="25em, 15em"
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