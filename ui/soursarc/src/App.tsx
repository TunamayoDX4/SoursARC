import { SamplePopover } from './components/SamplePopover'
import { DraggableWindow } from './components/SampleWindowWidget'

function App() {
  return (
    <section id="app">
      <header>
        <SamplePopover />
        <SamplePopover />
        <SamplePopover />
      </header>
      <main>
        <nav></nav>
        <section>
          <h1>SoursARCへようこそ！</h1>
          <DraggableWindow />
          <SamplePopover />
        </section>
      </main>
    </section>
  )
}

export default App