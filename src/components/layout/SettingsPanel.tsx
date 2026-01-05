import { ArrowLeft } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Label } from '@/components/ui/label'
import { Input } from '@/components/ui/input'
import { Switch } from '@/components/ui/switch'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { useDeckStore } from '@/store/deck-store'

interface SettingsPanelProps {
  onClose: () => void
}

export function SettingsPanel({ onClose }: SettingsPanelProps) {
  const { settings, updateSettings } = useDeckStore()

  return (
    <div className="p-3 space-y-3 text-sm">
      {/* Header */}
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={onClose} className="h-6 w-6">
          <ArrowLeft className="h-3 w-3" />
        </Button>
        <h2 className="text-sm font-semibold">Settings</h2>
      </div>

      {/* Terminal App */}
      <div className="space-y-1">
        <Label htmlFor="terminal-app" className="text-xs">Terminal</Label>
        <Select
          value={settings.terminalApp}
          onValueChange={(value) =>
            updateSettings({ terminalApp: value as typeof settings.terminalApp })
          }
        >
          <SelectTrigger id="terminal-app" className="h-8 text-xs">
            <SelectValue placeholder="Select terminal" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="terminal">Terminal.app</SelectItem>
            <SelectItem value="iterm">iTerm2</SelectItem>
            <SelectItem value="warp">Warp</SelectItem>
            <SelectItem value="custom">Custom</SelectItem>
          </SelectContent>
        </Select>
        {settings.terminalApp === 'custom' && (
          <Input
            placeholder="App name"
            value={settings.customTerminalApp}
            onChange={(e) => updateSettings({ customTerminalApp: e.target.value })}
            className="h-8 text-xs"
          />
        )}
      </div>

      {/* CLI Tool */}
      <div className="space-y-1">
        <Label htmlFor="cli-tool" className="text-xs">CLI Tool</Label>
        <Select
          value={settings.cliTool}
          onValueChange={(value) =>
            updateSettings({ cliTool: value as typeof settings.cliTool })
          }
        >
          <SelectTrigger id="cli-tool" className="h-8 text-xs">
            <SelectValue placeholder="Select CLI tool" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="claude">Claude Code</SelectItem>
            <SelectItem value="codex">Codex</SelectItem>
            <SelectItem value="custom">Custom</SelectItem>
          </SelectContent>
        </Select>
        {settings.cliTool === 'custom' && (
          <Input
            placeholder="Command"
            value={settings.customCliTool}
            onChange={(e) => updateSettings({ customCliTool: e.target.value })}
            className="h-8 text-xs"
          />
        )}
      </div>

      {/* Launch on Startup */}
      <div className="flex items-center justify-between pt-2">
        <Label htmlFor="launch-startup" className="text-xs">Launch on Startup</Label>
        <Switch
          id="launch-startup"
          checked={settings.launchOnStartup}
          onCheckedChange={(checked) => updateSettings({ launchOnStartup: checked })}
        />
      </div>

      {/* Info */}
      <p className="text-[10px] text-muted-foreground pt-2 border-t border-border">
        Changes update UI and Stream Deck buttons automatically
      </p>
    </div>
  )
}
