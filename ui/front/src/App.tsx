
import { Theme } from '@radix-ui/themes'
import './index.scss'
import '@radix-ui/themes/styles.css'
import { Header } from './Header/index'
import { Main } from './Main/index'

function App() {
  return (
    <Theme id='app-root'
      hasBackground={true}
      accentColor='mint'
      grayColor='slate'
      panelBackground='solid'
    >
      <title>SousARC</title>
      <Header />
      <Main />
    </Theme>
  )
}

export default App
