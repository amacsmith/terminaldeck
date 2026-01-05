import { create } from 'zustand'
import { toast } from 'sonner'
import { commands, type DeckSettings as TauriDeckSettings, type ButtonConfig } from '@/lib/bindings'

interface DeckSettings {
  terminalApp: 'terminal' | 'iterm' | 'warp' | 'custom'
  customTerminalApp: string
  cliTool: 'claude' | 'codex' | 'custom'
  customCliTool: string
  dictationShortcut: 'fn_twice' | 'fn_hold' | 'ctrl_twice' | 'custom'
  customDictationShortcut: string
  launchOnStartup: boolean
}

interface DeckState {
  enabled: boolean
  connected: boolean
  buttons: ButtonConfig[]
  settings: DeckSettings
  initialized: boolean
  setEnabled: (enabled: boolean) => Promise<void>
  setConnected: (connected: boolean) => void
  setButtons: (buttons: ButtonConfig[]) => void
  updateSettings: (settings: Partial<DeckSettings>) => Promise<void>
  loadSettings: () => Promise<void>
  fetchStatus: () => Promise<void>
  connect: () => Promise<boolean>
  disconnect: () => Promise<void>
  executeAction: (action: string) => Promise<void>
  refreshButtons: () => Promise<void>
}

const defaultSettings: DeckSettings = {
  terminalApp: 'terminal',
  customTerminalApp: '',
  cliTool: 'claude',
  customCliTool: '',
  dictationShortcut: 'ctrl_twice',
  customDictationShortcut: '',
  launchOnStartup: false,
}

// Convert saved preference string to DeckSettings type
function parseTerminalApp(saved: string | null | undefined): DeckSettings['terminalApp'] {
  if (!saved) return 'terminal'
  const lower = saved.toLowerCase()
  if (lower === 'terminal' || lower === 'iterm' || lower === 'warp') {
    return lower as DeckSettings['terminalApp']
  }
  return 'custom'
}

function parseCliTool(saved: string | null | undefined): DeckSettings['cliTool'] {
  if (!saved) return 'claude'
  const lower = saved.toLowerCase()
  if (lower === 'claude' || lower === 'codex') {
    return lower as DeckSettings['cliTool']
  }
  return 'custom'
}

function parseDictationShortcut(saved: string | null | undefined): DeckSettings['dictationShortcut'] {
  if (!saved) return 'ctrl_twice'
  const lower = saved.toLowerCase()
  if (lower === 'fn_twice' || lower === 'fn_hold' || lower === 'ctrl_twice') {
    return lower as DeckSettings['dictationShortcut']
  }
  return 'custom'
}

// Convert frontend settings to Tauri command format
function toTauriSettings(settings: DeckSettings): TauriDeckSettings {
  const terminalApp = settings.terminalApp === 'custom'
    ? settings.customTerminalApp
    : settings.terminalApp
  const cliTool = settings.cliTool === 'custom'
    ? settings.customCliTool
    : settings.cliTool
  const dictationShortcut = settings.dictationShortcut === 'custom'
    ? settings.customDictationShortcut
    : settings.dictationShortcut

  return {
    terminal_app: terminalApp || 'Terminal',
    cli_tool: cliTool || 'claude',
    dictation_shortcut: dictationShortcut || 'ctrl_twice',
  }
}

export const useDeckStore = create<DeckState>((set, get) => ({
  enabled: true, // Default to enabled
  connected: false,
  buttons: [],
  settings: defaultSettings,
  initialized: false,

  setEnabled: async (enabled) => {
    if (enabled) {
      // Try to connect first, only set enabled if successful
      const success = await get().connect()
      if (success) {
        set({ enabled: true })
      }
    } else {
      set({ enabled: false })
      await get().disconnect()
    }
  },

  setConnected: (connected) => set({ connected }),
  setButtons: (buttons) => set({ buttons }),

  loadSettings: async () => {
    const { initialized } = get()
    if (initialized) {
      console.log('[loadSettings] Already initialized, skipping')
      return
    }
    console.log('[loadSettings] Loading settings from disk...')
    try {
      const result = await commands.loadPreferences()
      if (result.status === 'error') {
        console.error('[loadSettings] Failed to load:', result.error)
        set({ initialized: true })
        return
      }
      const prefs = result.data
      console.log('[loadSettings] Loaded prefs:', prefs)
      const terminalApp = parseTerminalApp(prefs.terminal_app)
      const cliTool = parseCliTool(prefs.cli_tool)
      const dictationShortcut = parseDictationShortcut(prefs.dictation_shortcut)

      const loadedSettings: DeckSettings = {
        terminalApp,
        customTerminalApp: terminalApp === 'custom' ? (prefs.terminal_app || '') : '',
        cliTool,
        customCliTool: cliTool === 'custom' ? (prefs.cli_tool || '') : '',
        dictationShortcut,
        customDictationShortcut: dictationShortcut === 'custom' ? (prefs.dictation_shortcut || '') : '',
        launchOnStartup: false,
      }

      set({ settings: loadedSettings, initialized: true })
      console.log('[loadSettings] Set settings:', loadedSettings)

      // Immediately set backend action settings for physical button monitoring
      const tauriSettings = toTauriSettings(loadedSettings)
      await commands.deckSetActionSettings(tauriSettings)
      console.log('[loadSettings] Set action settings:', tauriSettings)
    } catch (error) {
      console.error('[loadSettings] Failed to load settings:', error)
      set({ initialized: true })
    }
  },

  updateSettings: async (newSettings) => {
    const currentSettings = get().settings
    const updatedSettings = { ...currentSettings, ...newSettings }
    console.log('[updateSettings] Setting new state:', updatedSettings)
    set({ settings: updatedSettings })

    // Save to preferences
    try {
      const prefsResult = await commands.loadPreferences()
      if (prefsResult.status === 'error') {
        console.error('[updateSettings] Failed to load prefs:', prefsResult.error)
        return
      }
      const prefs = prefsResult.data
      const tauriSettings = toTauriSettings(updatedSettings)
      console.log('[updateSettings] Saving preferences:', { terminal_app: tauriSettings.terminal_app, cli_tool: tauriSettings.cli_tool, dictation_shortcut: tauriSettings.dictation_shortcut })
      const saveResult = await commands.savePreferences({
        ...prefs,
        terminal_app: tauriSettings.terminal_app,
        cli_tool: tauriSettings.cli_tool,
        dictation_shortcut: tauriSettings.dictation_shortcut,
      })
      if (saveResult.status === 'error') {
        console.error('[updateSettings] Failed to save:', saveResult.error)
      } else {
        console.log('[updateSettings] Saved successfully')
      }
    } catch (error) {
      console.error('[updateSettings] Failed to save settings:', error)
    }

    // Refresh buttons with new settings (updates both UI and Stream Deck)
    await get().refreshButtons()
  },

  refreshButtons: async () => {
    const { settings, connected } = get()
    const tauriSettings = toTauriSettings(settings)

    // Get buttons with current settings
    const buttons = await commands.deckGetButtonsWithSettings(tauriSettings)
    set({ buttons })

    // Update physical Stream Deck if connected
    if (connected) {
      const result = await commands.deckUpdateButtonsWithSettings(tauriSettings)
      if (result.status === 'error') {
        console.error('Failed to update Stream Deck:', result.error)
      }
    }
  },

  fetchStatus: async () => {
    const status = await commands.deckStatus()
    const { settings } = get()

    // If connected, get buttons with current settings
    if (status.connected) {
      const tauriSettings = toTauriSettings(settings)
      const buttons = await commands.deckGetButtonsWithSettings(tauriSettings)
      set({ connected: status.connected, buttons })
    } else {
      set({ connected: status.connected, buttons: status.buttons })
    }
  },

  connect: async (): Promise<boolean> => {
    const result = await commands.deckConnect()
    if (result.status === 'ok') {
      set({ connected: true })
      // Refresh buttons with current settings after connect
      await get().refreshButtons()
      return true
    } else {
      console.error('Failed to connect:', result.error)
      set({ connected: false })
      return false
    }
  },

  disconnect: async () => {
    const result = await commands.deckDisconnect()
    if (result.status === 'ok') {
      set({ connected: false })
    } else {
      console.error('Failed to disconnect:', result.error)
    }
  },

  executeAction: async (action: string) => {
    // launchClaude types keystrokes - only works from Stream Deck with terminal focused
    if (action === 'launchClaude') {
      toast.info('Use this button from your Stream Deck while the terminal is focused')
      return
    }

    const { settings } = get()
    const result = await commands.deckExecuteAction(action, toTauriSettings(settings))
    if (result.status === 'error') {
      console.error('Action failed:', result.error)
    }
  },
}))
