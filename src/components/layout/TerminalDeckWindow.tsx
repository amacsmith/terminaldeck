import { Settings, Bot, ChevronRight } from 'lucide-react'
import { DeckGrid } from '@/components/deck/DeckGrid'
import { Button } from '@/components/ui/button'
import { Toaster } from 'sonner'
import { useTheme } from '@/hooks/use-theme'
import { useState } from 'react'
import { SettingsPanel } from './SettingsPanel'
import { ClaudePanel } from '@/components/claude/ClaudePanel'
import { cn } from '@/lib/utils'

type ViewMode = 'deck' | 'settings' | 'claude'

export function TerminalDeckWindow() {
  const { theme } = useTheme()
  const [viewMode, setViewMode] = useState<ViewMode>('deck')
  const [claudePanelVisible, setClaudePanelVisible] = useState(false)

  const toggleView = (mode: ViewMode) => {
    if (viewMode === mode) {
      setViewMode('deck')
    } else {
      setViewMode(mode)
    }
  }

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
        <div className="flex items-center gap-1">
          <Button
            variant={viewMode === 'claude' ? 'default' : 'ghost'}
            size="icon"
            onClick={() => toggleView('claude')}
            className={cn('h-6 w-6', viewMode === 'claude' && 'bg-orange-500 hover:bg-orange-600')}
            title="Claude Integration"
          >
            <Bot className="h-3 w-3" />
          </Button>
          <Button
            variant={viewMode === 'settings' ? 'default' : 'ghost'}
            size="icon"
            onClick={() => toggleView('settings')}
            className="h-6 w-6"
            title="Settings"
          >
            <Settings className="h-3 w-3" />
          </Button>
        </div>
      </div>

      {/* Main content */}
      <div className="flex flex-1 overflow-hidden">
        {/* Primary content area */}
        <div className="flex-1">
          {viewMode === 'settings' ? (
            <SettingsPanel onClose={() => setViewMode('deck')} />
          ) : viewMode === 'claude' ? (
            <ClaudePanel onClose={() => setViewMode('deck')} />
          ) : (
            <DeckGrid />
          )}
        </div>

        {/* Optional: Side panel toggle for Claude when viewing deck */}
        {viewMode === 'deck' && (
          <Button
            variant="ghost"
            size="icon"
            onClick={() => setClaudePanelVisible(!claudePanelVisible)}
            className={cn(
              'absolute right-0 top-1/2 -translate-y-1/2 h-12 w-4 rounded-l-md bg-muted/50 hover:bg-muted border-l border-t border-b border-border',
              claudePanelVisible && 'hidden'
            )}
            title="Show Claude Panel"
          >
            <ChevronRight className="h-3 w-3 rotate-180" />
          </Button>
        )}

        {/* Slide-out Claude panel */}
        {viewMode === 'deck' && claudePanelVisible && (
          <div className="w-64 border-l border-border animate-in slide-in-from-right duration-200">
            <ClaudePanel onClose={() => setClaudePanelVisible(false)} />
          </div>
        )}
      </div>

      <Toaster
        position="bottom-right"
        theme={theme === 'dark' ? 'dark' : theme === 'light' ? 'light' : 'system'}
      />
    </div>
  )
}
