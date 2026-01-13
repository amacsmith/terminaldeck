import { useEffect } from 'react'
import { cn } from '@/lib/utils'
import { useClaudeStore } from '@/store/claude-store'
import { Progress } from '@/components/ui/progress'
import { Badge } from '@/components/ui/badge'
import { ScrollArea } from '@/components/ui/scroll-area'

interface ClaudeMonitorProps {
  className?: string
  compact?: boolean
}

export function ClaudeMonitor({ className, compact = false }: ClaudeMonitorProps) {
  const session = useClaudeStore((state) => state.session)
  const fetchSession = useClaudeStore((state) => state.fetchSession)

  // Poll session every 2 seconds
  useEffect(() => {
    fetchSession()
    const interval = setInterval(fetchSession, 2000)
    return () => clearInterval(interval)
  }, [fetchSession])

  if (!session) {
    return (
      <div className={cn('flex items-center justify-center p-4 text-muted-foreground', className)}>
        No active session
      </div>
    )
  }

  if (compact) {
    return (
      <div className={cn('flex items-center gap-3 p-2', className)}>
        <PhaseIndicator phase={session.phase} />
        <Progress value={session.progress} className="flex-1 h-2" />
        <span className="text-xs text-muted-foreground">{session.progress}%</span>
        {session.awaitingInput && (
          <Badge variant="destructive" className="animate-pulse">
            Input
          </Badge>
        )}
      </div>
    )
  }

  return (
    <div className={cn('flex flex-col gap-3 p-3', className)}>
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <PhaseIndicator phase={session.phase} />
          <span className="text-sm font-medium">{session.name ?? 'Session'}</span>
        </div>
        <span className="text-xs text-muted-foreground">
          {formatDuration(session.durationSecs)}
        </span>
      </div>

      {/* Progress */}
      <div className="space-y-1">
        <div className="flex justify-between text-xs">
          <span className="text-muted-foreground">Progress</span>
          <span>{session.progress}%</span>
        </div>
        <Progress value={session.progress} className="h-2" />
      </div>

      {/* Current Step */}
      {session.currentStep && (
        <div className="text-sm">
          <span className="text-muted-foreground">Current: </span>
          <span>{session.currentStep}</span>
        </div>
      )}

      {/* Awaiting Input */}
      {session.awaitingInput && session.inputPrompt && (
        <div className="rounded-md bg-yellow-500/10 border border-yellow-500/20 p-2">
          <div className="flex items-center gap-2">
            <Badge variant="destructive" className="animate-pulse">
              Awaiting Input
            </Badge>
          </div>
          <p className="text-sm mt-1">{session.inputPrompt}</p>
        </div>
      )}

      {/* Tasks */}
      {session.tasks.length > 0 && (
        <div className="space-y-1">
          <span className="text-xs text-muted-foreground">Tasks</span>
          <ScrollArea className="h-[100px]">
            <div className="space-y-1">
              {session.tasks.map((task) => (
                <div
                  key={task.id}
                  className={cn(
                    'flex items-center gap-2 text-xs p-1 rounded',
                    task.status === 'Completed' && 'text-green-500',
                    task.status === 'InProgress' && 'text-blue-500',
                    task.status === 'Failed' && 'text-red-500'
                  )}
                >
                  <TaskStatusIcon status={task.status} />
                  <span className="truncate">{task.description}</span>
                </div>
              ))}
            </div>
          </ScrollArea>
        </div>
      )}

      {/* Next Steps */}
      {session.nextSteps.length > 0 && (
        <div className="space-y-1">
          <span className="text-xs text-muted-foreground">Next Steps</span>
          <div className="space-y-1">
            {session.nextSteps.slice(0, 4).map((step, i) => (
              <div key={i} className="flex items-center gap-2 text-xs">
                <span className="text-muted-foreground">{i + 1}.</span>
                <span className="truncate">{step}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}

function PhaseIndicator({ phase }: { phase: string }) {
  const phaseColors: Record<string, string> = {
    Idle: 'bg-gray-500',
    Planning: 'bg-blue-500',
    Specification: 'bg-purple-500',
    Architecture: 'bg-indigo-500',
    Coding: 'bg-green-500',
    Testing: 'bg-yellow-500',
    Refinement: 'bg-orange-500',
    Review: 'bg-pink-500',
    Complete: 'bg-emerald-500',
  }

  return (
    <div className="flex items-center gap-1">
      <div className={cn('w-2 h-2 rounded-full', phaseColors[phase] ?? 'bg-gray-500')} />
      <span className="text-xs">{phase}</span>
    </div>
  )
}

function TaskStatusIcon({ status }: { status: string }) {
  switch (status) {
    case 'Completed':
      return <span className="text-green-500">✓</span>
    case 'InProgress':
      return <span className="text-blue-500 animate-pulse">●</span>
    case 'Failed':
      return <span className="text-red-500">✗</span>
    case 'Skipped':
      return <span className="text-gray-500">○</span>
    default:
      return <span className="text-gray-400">○</span>
  }
}

function formatDuration(secs: number): string {
  const mins = Math.floor(secs / 60)
  const remainingSecs = secs % 60
  if (mins > 0) {
    return `${mins}m ${remainingSecs}s`
  }
  return `${remainingSecs}s`
}
