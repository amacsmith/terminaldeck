import { Settings } from 'lucide-react'
import { DeckGrid } from '@/components/deck/DeckGrid'
import { Button } from '@/components/ui/button'
import { Toaster } from 'sonner'
import { useTheme } from '@/hooks/use-theme'
import { useState } from 'react'
import { SettingsPanel } from './SettingsPanel'

export function TerminalDeckWindow() {
  const { theme } = useTheme()
  const [showSettings, setShowSettings] = useState(false)

  return (
    <div className="flex h-screen w-full flex-col overflow-hidden bg-background">
      {/* Title bar with drag region */}
      <div
        className="flex items-center justify-between px-3 py-1.5 border-b border-border"
        data-tauri-drag-region
      >
        <h1 className="text-xs font-semibold text-foreground" data-tauri-drag-region>
          TerminalDeck
        </h1>
        <Button
          variant="ghost"
          size="icon"
          onClick={() => setShowSettings(!showSettings)}
          className="h-6 w-6"
        >
          <Settings className="h-3 w-3" />
        </Button>
      </div>

      {/* Main content - no scrolling needed */}
      <div className="flex-1">
        {showSettings ? (
          <SettingsPanel onClose={() => setShowSettings(false)} />
        ) : (
          <DeckGrid />
        )}
      </div>

      <Toaster
        position="bottom-right"
        theme={theme === 'dark' ? 'dark' : theme === 'light' ? 'light' : 'system'}
      />
    </div>
  )
}
