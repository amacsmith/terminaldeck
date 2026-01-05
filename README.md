# TerminalDeck

Control [Claude Code](https://docs.anthropic.com/en/docs/claude-code) entirely through a Stream Deck and voice dictation. Designed for accessibility and hands-free coding.

## What It Does

TerminalDeck connects your Elgato Stream Deck MK.2 to Claude Code, letting you:

- **Open/switch terminal windows** without touching the keyboard
- **Launch Claude Code** with a single button press
- **Respond to prompts** (Yes, No, Yes to All, Cancel) hands-free
- **Navigate** conversation history (up/down arrows)
- **Submit prompts** and run commands
- **Toggle voice dictation** for truly hands-free coding

## Stream Deck Layout

```
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│  Open   │ Switch  │ Launch  │   New   │  Close  │
│ Terminal│ Window  │ Claude  │Terminal │ Session │
├─────────┼─────────┼─────────┼─────────┼─────────┤
│   Yes   │ Yes to  │   No    │ Cancel  │   Tab   │
│   [Y]   │   All   │   [N]   │   ESC   │   ->|   │
├─────────┼─────────┼─────────┼─────────┼─────────┤
│ Dictate │ Submit  │   UP    │  DOWN   │/compact │
│   MIC   │  ENTER  │         │         │         │
└─────────┴─────────┴─────────┴─────────┴─────────┘
```

## Requirements

- macOS (currently macOS-only, Windows/Linux planned)
- Elgato Stream Deck MK.2 (15-key)
- Node.js 20+
- Rust (latest stable)

## Installation

```bash
git clone https://github.com/sidmohan0/terminaldeck.git
cd terminaldeck
npm install
npm run tauri:dev
```

## Configuration

In the app settings, you can configure:

- **Terminal App**: Terminal, iTerm, or Warp
- **CLI Tool**: Claude or Codex

## How It Works

TerminalDeck uses:
- **HID API** to communicate directly with Stream Deck hardware
- **AppleScript** to control terminal applications and send keystrokes
- **Tauri** for the native macOS app wrapper

## Tech Stack

| Layer    | Technologies                        |
| -------- | ----------------------------------- |
| Frontend | React 19, TypeScript, Vite          |
| UI       | shadcn/ui, Tailwind CSS             |
| Backend  | Tauri v2, Rust                      |
| Hardware | hidapi (Stream Deck communication)  |

## License

[MIT](LICENSE.md) - Sid Mohan

## Acknowledgments

Built on the [Tauri React Template](https://github.com/dannysmith/tauri-template) by Danny Smith.
