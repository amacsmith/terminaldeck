import { useEffect } from 'react'
import { logger } from './lib/logger'
import './App.css'
import { TerminalDeckWindow } from './components/layout/TerminalDeckWindow'
import { ThemeProvider } from './components/ThemeProvider'
import { ErrorBoundary } from './components/ErrorBoundary'

function App() {
  useEffect(() => {
    logger.info('🚀 TerminalDeck starting up')

    // Force dark mode for Bloomberg terminal aesthetic
    document.documentElement.classList.add('dark')

    logger.info('App environment', {
      isDev: import.meta.env.DEV,
      mode: import.meta.env.MODE,
    })
  }, [])

  return (
    <ErrorBoundary>
      <ThemeProvider defaultTheme="dark">
        <TerminalDeckWindow />
      </ThemeProvider>
    </ErrorBoundary>
  )
}

export default App
