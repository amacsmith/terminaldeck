import { useEffect } from 'react'
import { cn } from '@/lib/utils'
import { useDeckStore } from '@/store/deck-store'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'

interface DeckButtonProps {
  label: string
  sublabel?: string | null
  color: string
  onClick: () => void
}

// Map hex colors to Bloomberg-style classes
function colorToClass(hexColor: string): string {
  const colorMap: Record<string, string> = {
    '#2563eb': 'bg-[#0068ff]',      // Bloomberg blue
    '#475569': 'bg-[#1a1a1a]',      // Dark gray
    '#dc2626': 'bg-[#ff433d]',      // Bloomberg red
    '#059669': 'bg-[#00d26a]',      // Bloomberg green
    '#7c3aed': 'bg-[#9945ff]',      // Purple
    '#0891b2': 'bg-[#00b8d4]',      // Cyan
  }
  return colorMap[hexColor] || 'bg-[#1a1a1a]'
}

function DeckButton({ label, sublabel, color, onClick }: DeckButtonProps) {
  return (
    <button
      onClick={onClick}
      className={cn(
        'aspect-square rounded-lg flex flex-col items-center justify-center',
        'transition-all duration-100 hover:brightness-125 active:scale-95',
        'border border-[#444] shadow-lg',
        'w-[144px] h-[144px]',
        colorToClass(color)
      )}
    >
      <span className={cn(
        'font-bold text-white leading-none drop-shadow-sm',
        label.length <= 2 ? 'text-4xl' : label.length <= 6 ? 'text-xl' : 'text-lg'
      )}>
        {label}
      </span>
      {sublabel && (
        <span className={cn(
          'text-white/90 leading-none mt-2 drop-shadow-sm',
          sublabel.length <= 2 ? 'text-2xl' : 'text-lg'
        )}>
          {sublabel}
        </span>
      )}
    </button>
  )
}

export function DeckGrid() {
  const { enabled, connected, buttons, fetchStatus, executeAction, setEnabled, connect, loadSettings, initialized } = useDeckStore()

  // Load saved settings on mount (only once)
  useEffect(() => {
    loadSettings()
  }, []) // eslint-disable-line react-hooks/exhaustive-deps

  // Connect on mount if enabled (starts button monitoring)
  useEffect(() => {
    if (initialized && enabled && !connected) {
      connect()
    }
  }, [initialized]) // Run after settings are loaded

  useEffect(() => {
    // Only fetch status if enabled
    if (enabled) {
      fetchStatus()
    }
  }, [fetchStatus, enabled])

  // Poll connection status every 2 seconds when enabled
  useEffect(() => {
    if (!enabled) return

    const interval = setInterval(async () => {
      await fetchStatus()
    }, 2000)

    return () => clearInterval(interval)
  }, [enabled, fetchStatus])

  return (
    <div className="flex flex-col items-center justify-center gap-3 p-4 h-full">
      {/* Enable/Disable toggle and connection status */}
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Switch
            id="deck-enabled"
            checked={enabled}
            onCheckedChange={setEnabled}
          />
          <Label htmlFor="deck-enabled" className="text-sm font-medium">
            {enabled ? 'Enabled' : 'Disabled'}
          </Label>
        </div>
        {enabled && (
          <div className="flex items-center gap-2 text-xs">
            <div className={cn(
              'w-2 h-2 rounded-full',
              connected ? 'bg-green-500' : 'bg-red-500'
            )} />
            <span className="text-muted-foreground">
              {connected ? 'Connected' : 'Disconnected'}
            </span>
          </div>
        )}
      </div>

      {/* 5x3 Grid - centered */}
      <div className={cn(
        "grid grid-cols-5 gap-1.5",
        !enabled && "opacity-50 pointer-events-none"
      )}>
        {buttons.map((button) => (
          <DeckButton
            key={button.id}
            label={button.label}
            sublabel={button.sublabel}
            color={button.color}
            onClick={() => enabled && executeAction(button.action)}
          />
        ))}
      </div>

      {/* Footer hint */}
      <p className="text-[10px] text-muted-foreground">
        {enabled
          ? 'Click buttons here or press on your Stream Deck'
          : 'Enable to control your Stream Deck'
        }
      </p>
    </div>
  )
}
