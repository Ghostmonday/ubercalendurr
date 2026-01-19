# UberCalendurr

A revolutionary dual-interface calendar application that combines terminal-centric control with intelligent AI assistance.

## Overview

UberCalendurr is a desktop calendar application featuring:
- **Terminal Widget**: A persistent, always-accessible terminal interface for quick calendar entry
- **GUI Calendar**: A beautiful, intuitive visual calendar with drag-and-drop support
- **AI-Powered Input**: DeepSeek integration transforms natural language into structured calendar entries
- **Cross-Platform**: Built on Rust and Tauri for native performance

## Quick Start

### Prerequisites
- Rust 1.70 or later
- Node.js 18+ (for frontend development)
- Windows 10/11

### Building

```bash
# Install dependencies
cargo install cargo-watch
npm install

# Development build
cargo build --workspace

# Release build
cargo build --workspace --release
```

### Running

```bash
# Terminal widget
cargo run -p calendar-widget

# GUI application
cargo run -p calendar-gui
```

## Project Structure

```
ubercalendurr/
├── Cargo.toml              # Workspace configuration
├── binaries/
│   ├── calendar-widget/    # Terminal widget binary
│   └── calendar-gui/       # GUI application binary
├── libraries/
│   ├── calendar-core/      # Core data models
│   ├── deepseek-client/    # DeepSeek API integration
│   ├── storage-engine/     # SQLite abstraction
│   └── IPC-primitives/     # IPC protocol definitions
└── scripts/                # Build and utility scripts
```

## Architecture

### Terminal Widget
The terminal widget is the primary interface, designed for power users who prefer keyboard-driven workflows. It provides:
- Natural language event entry
- Command-based operations
- Real-time feedback
- Hotkey activation

### GUI Calendar
The companion GUI provides visual representation with:
- Monthly grid view
- Drag-and-drop rescheduling
- Event details and editing
- Theme support

### DeepSeek Integration
Natural language processing powered by DeepSeek handles:
- Event extraction from conversational input
- Temporal reasoning for ambiguous dates
- Clarification questions for missing information
- Context-aware suggestions

## Configuration

Settings are stored in:
- Windows: `%APPDATA%\ubercalendurr\settings.toml`

Key configuration options:
- Hotkey activation
- Theme and appearance
- API credentials
- Notification preferences

## Development

### Running Tests
```bash
cargo test --workspace
```

### Code Formatting
```bash
cargo fmt --all
cargo clippy --workspace
```

### Adding Dependencies
Add dependencies to the appropriate `Cargo.toml` in:
- `libraries/*/` for shared libraries
- `binaries/*/` for application-specific dependencies

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting PRs.

## Support

For issues and feature requests, please use GitHub Issues.
