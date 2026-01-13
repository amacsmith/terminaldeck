import { cn } from '@/lib/utils'
import { useClaudeStore } from '@/store/claude-store'
import { useDeckStore } from '@/store/deck-store'
import { Button } from '@/components/ui/button'

interface QuickActionsProps {
  className?: string
}

export function QuickActions({ className }: QuickActionsProps) {
  const respondNumber = useClaudeStore((state) => state.respondNumber)
  const projectCommand = useClaudeStore((state) => state.projectCommand)
  const executeAction = useDeckStore((state) => state.executeAction)

  return (
    <div className={cn('flex flex-col gap-3 p-3', className)}>
      <h3 className="text-sm font-medium">Quick Actions</h3>

      {/* Numbered Responses */}
      <div className="space-y-1">
        <span className="text-xs text-muted-foreground">Respond</span>
        <div className="grid grid-cols-4 gap-1">
          {[1, 2, 3, 4].map((n) => (
            <Button
              key={n}
              variant="outline"
              size="sm"
              className="h-8"
              onClick={() => respondNumber(n)}
            >
              {n}
            </Button>
          ))}
        </div>
      </div>

      {/* Yes/No */}
      <div className="grid grid-cols-2 gap-2">
        <Button
          variant="outline"
          size="sm"
          className="bg-green-500/10 hover:bg-green-500/20 border-green-500/20"
          onClick={() => executeAction('yes')}
        >
          Yes
        </Button>
        <Button
          variant="outline"
          size="sm"
          className="bg-red-500/10 hover:bg-red-500/20 border-red-500/20"
          onClick={() => executeAction('no')}
        >
          No
        </Button>
      </div>

      {/* Project Commands */}
      <div className="space-y-1">
        <span className="text-xs text-muted-foreground">Project</span>
        <div className="grid grid-cols-2 gap-1">
          <Button
            variant="outline"
            size="sm"
            className="h-8"
            onClick={() => projectCommand('build')}
          >
            /build
          </Button>
          <Button
            variant="outline"
            size="sm"
            className="h-8"
            onClick={() => projectCommand('test')}
          >
            /test
          </Button>
          <Button
            variant="outline"
            size="sm"
            className="h-8"
            onClick={() => projectCommand('commit')}
          >
            /commit
          </Button>
          <Button
            variant="outline"
            size="sm"
            className="h-8"
            onClick={() => projectCommand('pr')}
          >
            /pr
          </Button>
        </div>
      </div>

      {/* Submit */}
      <Button
        className="w-full bg-blue-600 hover:bg-blue-700"
        onClick={() => executeAction('submit')}
      >
        Submit ↵
      </Button>
    </div>
  )
}
