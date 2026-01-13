import { create } from 'zustand'
import { logger } from '@/lib/logger'

// Types matching Rust structs
export type SessionPhase =
  | 'Idle'
  | 'Planning'
  | 'Specification'
  | 'Architecture'
  | 'Coding'
  | 'Testing'
  | 'Refinement'
  | 'Review'
  | 'Complete'

export type TaskState = 'Pending' | 'InProgress' | 'Completed' | 'Skipped' | 'Failed'

export interface MonitoredTask {
  id: number
  description: string
  status: TaskState
  substeps: string[]
}

export interface SessionInfo {
  id: string
  name: string | null
  phase: SessionPhase
  progress: number
  tasks: MonitoredTask[]
  currentStep: string | null
  nextSteps: string[]
  awaitingInput: boolean
  inputPrompt: string | null
  durationSecs: number
}

export interface TtsSettings {
  enabled: boolean
  voice: string
  rate: number
  lastText: string
}

export type ButtonLayout = 'Default' | 'ClaudeCode' | 'ProjectBuild' | 'NumberedResponse' | 'Monitoring'

interface ClaudeStore {
  // State
  session: SessionInfo | null
  ttsSettings: TtsSettings
  sttActive: boolean
  currentLayout: ButtonLayout
  isLoading: boolean

  // Actions
  fetchSession: () => Promise<void>
  startSession: (name?: string) => Promise<void>
  setPhase: (phase: string) => Promise<void>
  addTask: (description: string) => Promise<number>
  updateTask: (taskId: number, status: string) => Promise<void>
  setNextSteps: (steps: string[]) => Promise<void>

  // Speech
  speak: (text: string) => Promise<void>
  speakSummary: (text: string) => Promise<void>
  toggleTts: () => Promise<void>
  toggleStt: () => Promise<void>
  fetchTtsSettings: () => Promise<void>
  setTtsSettings: (settings: TtsSettings) => Promise<void>

  // Layout
  setLayout: (layout: ButtonLayout) => void

  // Actions
  respondNumber: (num: number) => Promise<void>
  nextTerminal: () => Promise<void>
  terminalTab: (tab: number) => Promise<void>
  projectCommand: (cmd: string) => Promise<void>
}

export const useClaudeStore = create<ClaudeStore>((set, _get) => ({
  // Initial state
  session: null,
  ttsSettings: {
    enabled: true,
    voice: 'Samantha',
    rate: 200,
    lastText: '',
  },
  sttActive: false,
  currentLayout: 'Default',
  isLoading: false,

  // Session management
  fetchSession: async () => {
    try {
      // TODO: Uncomment when bindings are generated
      // const result = await commands.claudeGetSession()
      // if (result.status === 'ok') {
      //   set({ session: result.data })
      // }
    } catch (err) {
      logger.error('Failed to fetch session', { error: String(err) })
    }
  },

  startSession: async (name?: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeStartSession(name ?? null)
      // await get().fetchSession()
      logger.info('Started new Claude session', { name })
    } catch (err) {
      logger.error('Failed to start session', { error: String(err) })
    }
  },

  setPhase: async (_phase: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeSetPhase(_phase)
      // await _get().fetchSession()
    } catch (err) {
      logger.error('Failed to set phase', { error: String(err) })
    }
  },

  addTask: async (_description: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // const result = await commands.claudeAddTask(_description)
      // if (result.status === 'ok') {
      //   await _get().fetchSession()
      //   return result.data
      // }
      return 0
    } catch (err) {
      logger.error('Failed to add task', { error: String(err) })
      return 0
    }
  },

  updateTask: async (_taskId: number, _status: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeUpdateTask(_taskId, _status)
      // await _get().fetchSession()
    } catch (err) {
      logger.error('Failed to update task', { error: String(err) })
    }
  },

  setNextSteps: async (_steps: string[]) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeSetNextSteps(_steps)
      // await _get().fetchSession()
    } catch (err) {
      logger.error('Failed to set next steps', { error: String(err) })
    }
  },

  // Speech
  speak: async (text: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeSpeak(text)
      logger.info('TTS speak', { text: text.slice(0, 50) })
    } catch (err) {
      logger.error('Failed to speak', { error: String(err) })
    }
  },

  speakSummary: async (_text: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeSpeakSummary(_text)
    } catch (err) {
      logger.error('Failed to speak summary', { error: String(err) })
    }
  },

  toggleTts: async () => {
    try {
      // TODO: Uncomment when bindings are generated
      // const result = await commands.claudeToggleTts()
      // if (result.status === 'ok') {
      //   set((state) => ({
      //     ttsSettings: { ...state.ttsSettings, enabled: result.data }
      //   }))
      // }
      set((state) => ({
        ttsSettings: { ...state.ttsSettings, enabled: !state.ttsSettings.enabled }
      }))
    } catch (err) {
      logger.error('Failed to toggle TTS', { error: String(err) })
    }
  },

  toggleStt: async () => {
    try {
      // TODO: Uncomment when bindings are generated
      // const result = await commands.claudeToggleStt()
      // if (result.status === 'ok') {
      //   set({ sttActive: result.data })
      // }
      set((state) => ({ sttActive: !state.sttActive }))
    } catch (err) {
      logger.error('Failed to toggle STT', { error: String(err) })
    }
  },

  fetchTtsSettings: async () => {
    try {
      // TODO: Uncomment when bindings are generated
      // const result = await commands.claudeGetTtsSettings()
      // if (result.status === 'ok') {
      //   set({ ttsSettings: result.data })
      // }
    } catch (err) {
      logger.error('Failed to fetch TTS settings', { error: String(err) })
    }
  },

  setTtsSettings: async (settings: TtsSettings) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeSetTtsSettings(settings)
      set({ ttsSettings: settings })
    } catch (err) {
      logger.error('Failed to set TTS settings', { error: String(err) })
    }
  },

  // Layout
  setLayout: (layout: ButtonLayout) => {
    set({ currentLayout: layout })
    logger.info('Changed layout', { layout })
  },

  // Actions
  respondNumber: async (num: number) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeRespondNumber(num)
      logger.info('Responded with number', { num })
    } catch (err) {
      logger.error('Failed to respond with number', { error: String(err) })
    }
  },

  nextTerminal: async () => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeNextTerminal('Terminal')
      logger.info('Switched to next terminal')
    } catch (err) {
      logger.error('Failed to switch terminal', { error: String(err) })
    }
  },

  terminalTab: async (tab: number) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeTerminalTab(tab, 'Terminal')
      logger.info('Switched to terminal tab', { tab })
    } catch (err) {
      logger.error('Failed to switch tab', { error: String(err) })
    }
  },

  projectCommand: async (cmd: string) => {
    try {
      // TODO: Uncomment when bindings are generated
      // await commands.claudeProjectCommand(cmd)
      logger.info('Sent project command', { cmd })
    } catch (err) {
      logger.error('Failed to send project command', { error: String(err) })
    }
  },
}))
