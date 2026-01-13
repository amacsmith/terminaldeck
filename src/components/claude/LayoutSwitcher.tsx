import { cn } from '@/lib/utils'
import { useClaudeStore, type ButtonLayout } from '@/store/claude-store'
import { Button } from '@/components/ui/button'

interface LayoutSwitcherProps {
  className?: string
}

const layouts: { id: ButtonLayout; label: string; description: string }[] = [
  { id: 'Default', label: 'Default', description: 'Standard terminal control' },
  { id: 'ClaudeCode', label: 'Claude', description: 'Voice + numbered responses' },
  { id: 'ProjectBuild', label: 'Build', description: 'Project commands' },
  { id: 'NumberedResponse', label: 'Numbers', description: 'Quick 1-9 responses' },
  { id: 'Monitoring', label: 'Monitor', description: 'Progress & tasks' },
]

export function LayoutSwitcher({ className }: LayoutSwitcherProps) {
  const currentLayout = useClaudeStore((state) => state.currentLayout)
  const setLayout = useClaudeStore((state) => state.setLayout)

  const handleLayoutChange = (layout: ButtonLayout) => {
    setLayout(layout)
    // TODO: Call backend to update Stream Deck button images
  }

  return (
    <div className={cn('flex flex-col gap-3 p-3', className)}>
      <h3 className="text-sm font-medium">Button Layout</h3>

      <div className="grid grid-cols-2 gap-2">
        {layouts.map((layout) => (
          <Button
            key={layout.id}
            variant={currentLayout === layout.id ? 'default' : 'outline'}
            size="sm"
            className={cn(
              'flex flex-col h-auto py-2',
              currentLayout === layout.id && 'ring-2 ring-blue-500'
            )}
            onClick={() => handleLayoutChange(layout.id)}
          >
            <span className="font-medium">{layout.label}</span>
            <span className="text-[10px] text-muted-foreground">
              {layout.description}
            </span>
          </Button>
        ))}
      </div>
    </div>
  )
}
