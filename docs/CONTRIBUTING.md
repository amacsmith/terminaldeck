# Contributing Guidelines

Thank you for your interest in contributing to TerminalDeck!

## Tested Environment

| Component | Version | Notes |
|-----------|---------|-------|
| macOS | 15.5 (Sequoia) | Minimum: 12.0 (Monterey) |
| Architecture | Apple Silicon (arm64) | Universal binary supports Intel |
| Rust | 1.91.0 (stable) | |
| Node.js | 25.x | |
| Stream Deck | MK.2 (15-key) | Other models untested |

## Hardware Requirements

This project currently supports:

- **Elgato Stream Deck MK.2** (Product ID: `0x0080`)

Other Stream Deck models have different HID protocols and are not yet supported.

## Quick Start

### Prerequisites

- macOS 12.0 or later
- [Node.js](https://nodejs.org/) (v20+)
- [Rust](https://rustup.rs/) (latest stable)
- Elgato Stream Deck MK.2

### Setup

```bash
git clone https://github.com/sidmohan0/terminaldeck.git
cd terminaldeck
npm install
npm run tauri:dev
```

Verify everything works:
```bash
npm run check:all
```

## How to Contribute

### Issues

- **Bug Reports**: Include your macOS version, Stream Deck model, and reproduction steps
- **Feature Requests**: Describe the use case and expected behavior
- **Security Issues**: See [SECURITY.md](SECURITY.md)

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make changes following the guidelines below
4. Ensure checks pass: `npm run check:all`
5. Commit using conventional commits
6. Push and open a Pull Request

## Adding Support for New Stream Deck Models

We welcome PRs for additional Stream Deck hardware! Requirements:

1. **You must own the device** - We can't test hardware we don't have
2. **Document the HID protocol** - Include your findings (packet format, button mapping, byte offsets)
3. **Provide test evidence** - Video or logs showing buttons working correctly
4. **Keep MK.2 working** - Don't break existing support

### What to Include in Your PR

- Device product ID and any protocol differences
- Test results (screenshots/video of buttons working)
- Updates to README listing supported devices
- Your device info for the "Tested By" section

### HID Protocol Notes

The MK.2 uses this button state format:
```
[0x01, 0x00, 0x0F, 0x00, key1, key2, ..., key15]
       ^           ^     ^--- Button states start at byte 4
       header      0x0F = 15 keys
```

Other models may differ. See `src-tauri/src/streamdeck/mod.rs` for implementation details.

## Code Guidelines

### TypeScript/React

- Use TypeScript for all new code
- Follow existing component patterns
- See `docs/developer/` for architecture patterns

### Rust

- Use `cargo fmt` and `cargo clippy`
- Use `Result<T, String>` for Tauri commands
- See `docs/developer/rust-architecture.md`

## Quality Gates

All PRs must pass:

- TypeScript type checking
- ESLint and Prettier
- Rust formatting and clippy
- Tests

Run locally: `npm run check:all`

## Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
feat: add user authentication
fix(ui): resolve sidebar toggle issue
docs: update installation instructions
refactor(store): simplify state management
test: add preferences tests
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

## Code Review

- Keep PRs focused and reasonably sized
- Write clear PR descriptions
- Respond to feedback promptly
- Update documentation as needed

## Legal

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT).
