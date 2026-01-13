import { cn } from '@/lib/utils'
import { useClaudeStore } from '@/store/claude-store'
import { Button } from '@/components/ui/button'
import { Switch } from '@/components/ui/switch'
import { Label } from '@/components/ui/label'
import { Mic, Volume2, VolumeX } from 'lucide-react'

interface SpeechControlsProps {
  className?: string
  compact?: boolean
}

export function SpeechControls({ className, compact = false }: SpeechControlsProps) {
  const ttsSettings = useClaudeStore((state) => state.ttsSettings)
  const sttActive = useClaudeStore((state) => state.sttActive)
  const toggleTts = useClaudeStore((state) => state.toggleTts)
  const toggleStt = useClaudeStore((state) => state.toggleStt)

  if (compact) {
    return (
      <div className={cn('flex items-center gap-2', className)}>
        <Button
          variant={sttActive ? 'default' : 'outline'}
          size="icon"
          className="h-8 w-8"
          onClick={toggleStt}
        >
          <Mic className={cn('h-4 w-4', sttActive && 'animate-pulse text-red-500')} />
        </Button>
        <Button
          variant={ttsSettings.enabled ? 'default' : 'outline'}
          size="icon"
          className="h-8 w-8"
          onClick={toggleTts}
        >
          {ttsSettings.enabled ? (
            <Volume2 className="h-4 w-4" />
          ) : (
            <VolumeX className="h-4 w-4" />
          )}
        </Button>
      </div>
    )
  }

  return (
    <div className={cn('flex flex-col gap-3 p-3', className)}>
      <h3 className="text-sm font-medium">Speech Controls</h3>

      {/* STT Toggle */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <Mic className={cn('h-4 w-4', sttActive && 'text-red-500 animate-pulse')} />
          <Label htmlFor="stt-toggle" className="text-sm">
            Voice Input (STT)
          </Label>
        </div>
        <Switch
          id="stt-toggle"
          checked={sttActive}
          onCheckedChange={toggleStt}
        />
      </div>

      {/* TTS Toggle */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          {ttsSettings.enabled ? (
            <Volume2 className="h-4 w-4" />
          ) : (
            <VolumeX className="h-4 w-4 text-muted-foreground" />
          )}
          <Label htmlFor="tts-toggle" className="text-sm">
            Voice Output (TTS)
          </Label>
        </div>
        <Switch
          id="tts-toggle"
          checked={ttsSettings.enabled}
          onCheckedChange={toggleTts}
        />
      </div>

      {/* Voice Info */}
      {ttsSettings.enabled && (
        <div className="text-xs text-muted-foreground">
          Voice: {ttsSettings.voice} @ {ttsSettings.rate} wpm
        </div>
      )}
    </div>
  )
}
