import { useState } from 'react'
import { cn } from '@/lib/utils'
import { ClaudeMonitor } from './ClaudeMonitor'
import { SpeechControls } from './SpeechControls'
import { QuickActions } from './QuickActions'
import { LayoutSwitcher } from './LayoutSwitcher'
import { Button } from '@/components/ui/button'
import { ChevronLeft } from 'lucide-react'

interface ClaudePanelProps {
  className?: string
  onClose?: () => void
}

type TabId = 'monitor' | 'actions' | 'layout'

export function ClaudePanel({ className, onClose }: ClaudePanelProps) {
  const [activeTab, setActiveTab] = useState<TabId>('monitor')

  return (
    <div className={cn('flex flex-col h-full bg-background', className)}>
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b border-border">
        <h2 className="text-sm font-semibold">Claude Integration</h2>
        {onClose && (
          <Button variant="ghost" size="icon" className="h-6 w-6" onClick={onClose}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
        )}
      </div>

      {/* Speech Controls - Always visible */}
      <SpeechControls compact className="px-3 py-2 border-b border-border" />

      {/* Tabs */}
      <div className="flex border-b border-border">
        <TabButton
          active={activeTab === 'monitor'}
          onClick={() => setActiveTab('monitor')}
        >
          Monitor
        </TabButton>
        <TabButton
          active={activeTab === 'actions'}
          onClick={() => setActiveTab('actions')}
        >
          Actions
        </TabButton>
        <TabButton
          active={activeTab === 'layout'}
          onClick={() => setActiveTab('layout')}
        >
          Layout
        </TabButton>
      </div>

      {/* Tab Content */}
      <div className="flex-1 overflow-auto">
        {activeTab === 'monitor' && <ClaudeMonitor />}
        {activeTab === 'actions' && <QuickActions />}
        {activeTab === 'layout' && <LayoutSwitcher />}
      </div>

      {/* Footer */}
      <div className="px-3 py-2 border-t border-border text-[10px] text-muted-foreground text-center">
        Press Stream Deck buttons or use controls above
      </div>
    </div>
  )
}

function TabButton({
  active,
  onClick,
  children,
}: {
  active: boolean
  onClick: () => void
  children: React.ReactNode
}) {
  return (
    <button
      className={cn(
        'flex-1 py-2 text-xs font-medium transition-colors',
        active
          ? 'text-foreground border-b-2 border-primary'
          : 'text-muted-foreground hover:text-foreground'
      )}
      onClick={onClick}
    >
      {children}
    </button>
  )
}
