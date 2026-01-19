# UberCalendurr

> **Your personal time ledger with zero-friction capture** — AI-powered calendar that works offline, understands your world, and keeps your data sovereign.

UberCalendurr is a dual-interface calendar application that transforms natural language into structured events. Built with Rust and Tauri, it combines a lightning-fast terminal widget for quick capture with a beautiful GUI for comprehensive overview—all powered by a local SQLite database.

---

## 🎯 Core Philosophy

- **Offline First**: SimpleParser works without internet—AI is optional enhancement
- **Zero-Friction Capture**: Type "Lunch tomorrow at 2pm" and it's saved in 2 seconds
- **Your World View**: Hardcoded intelligence for S-DNA, KryptoClaw, Neural Draft LLC
- **Data Sovereignty**: SQLite is the single source of truth—export anytime, anywhere
- **Dual Interface**: Terminal for speed, GUI for overview

---

## ✨ Key Features

### 🚀 Natural Language Processing
- **SimpleParser** (Offline): Regex-based parsing that understands:
  - Relative dates: "today", "tomorrow", "next Monday"
  - Times: "2pm", "14:00", "lunch", "evening"
  - Priorities: Detects "urgent", "deadline", "critical"
  - Categories: Auto-classifies work, social, health, personal

- **AIParser** (Optional): DeepSeek integration for advanced parsing when API key is provided

### 🧠 Project Intelligence
Auto-tags events based on your projects:
- **S-DNA** → Work category, project: S-DNA
- **KryptoClaw** → Work category, project: KryptoClaw  
- **Neural Draft** → Work category, company: Neural Draft LLC

Your calendar becomes a **time-allocation ledger**—track exactly where your hours go.

### 💾 Data Sovereignty
- **SQLite Database**: Single source of truth, stored locally
- **Export Formats**: JSON, CSV, ICS (iCalendar)
- **No Vendor Lock-in**: Your data, your format, always exportable

### 🎨 Dual Interface

#### Terminal Widget (`calendar-widget`)
Lightning-fast command-line interface for rapid event capture:
```bash
📅> S-DNA sync tomorrow at 2pm
✅ [2026-01-19 14:00] S-DNA sync (work) — Saved.

📅> Lunch with Sarah next Tuesday
✅ [2026-01-21 12:00] Lunch with Sarah (social) — Saved.

📅> /export json
✅ Exported 47 events to ubercalendurr_export_20260118_143022.json
```

#### GUI Application (`calendar-gui`)
Beautiful React + Tauri interface for:
- Monthly calendar view with event previews
- Natural language input terminal
- Full event management (create, update, delete, search)
- Settings panel

### 🔧 Technical Excellence
- **Rust Backend**: Fast, safe, memory-efficient
- **Synchronous Repository**: Simple, direct database access
- **Unified Errors**: Single `AppError` type across entire codebase
- **Proper Async**: Sync repository with `spawn_blocking` in async contexts
- **No IPC Complexity**: Single-process architecture

---

## 📦 Installation

### Prerequisites
- **Rust** (1.70+): [rustup.rs](https://rustup.rs/)
- **Node.js** (18+): [nodejs.org](https://nodejs.org/)
- **Tauri Prerequisites**: [tauri.app](https://tauri.app/v1/guides/getting-started/prerequisites)

### Build from Source

```bash
# Clone repository
git clone https://github.com/Ghostmonday/ubercalendurr.git
cd ubercalendurr

# Build Rust workspace (all libraries + binaries)
cargo build --release

# Build frontend for GUI
cd binaries/calendar-gui/frontend
npm install
npm run build

# Run terminal widget
cargo run --bin calendar-widget --release

# Or run GUI application
cd binaries/calendar-gui
cargo tauri dev  # Development
cargo tauri build  # Production build
```

### Quick Start Script

```powershell
# Windows (PowerShell)
.\scripts\build.ps1
```

---

## 🎮 Usage

### Terminal Widget

Start the widget:
```bash
cargo run --bin calendar-widget
```

**Natural Language Examples:**
```
📅> Meeting tomorrow at 9am
📅> KryptoClaw review next Friday
📅> Doctor appointment in 2 weeks
📅> S-DNA sync Monday morning
📅> Lunch with team at noon
📅> Neural Draft standup every Monday
```

**Commands:**
- `/help` - Show help
- `/today` - Show today's events
- `/search <term>` - Search events
- `/export <format>` - Export events (json/csv/ics)
- `/exit` - Exit application

### GUI Application

Launch the Tauri GUI:
```bash
cd binaries/calendar-gui
cargo tauri dev
```

**Features:**
- Click calendar dates to view events
- Use terminal input on the right side for natural language entry
- Navigate months with arrow buttons
- Click "Today" to jump to current month
- Search and filter events

---

## 🏗️ Architecture

```
ubercalendurr/
├── libraries/
│   ├── calendar-core/        # Core data models (CalendarEvent, enums)
│   ├── deepseek-client/      # Optional AI integration
│   └── storage-engine/       # SQLite repository layer
├── binaries/
│   ├── calendar-widget/      # Terminal widget (CLI)
│   │   ├── app.rs           # Interactive loop
│   │   ├── input/           # SimpleParser + InputHandler
│   │   ├── export.rs        # Export functionality
│   │   └── storage.rs       # Repository wrapper
│   └── calendar-gui/        # Tauri GUI application
│       ├── src/main.rs      # Tauri commands
│       └── frontend/        # React + TypeScript + TailwindCSS
└── scripts/
    └── build.ps1            # Build automation
```

### Data Flow

```
User Input → SimpleParser/AIParser → CalendarEvent → SQLite Database
                                    ↓
                            Calendar Widget/GUI ← SQLite Database
```

### Key Design Decisions

1. **Single-Process Architecture**: Removed IPC complexity—both interfaces access the same SQLite database directly
2. **Synchronous Repository**: SQLite operations are synchronous, wrapped with `spawn_blocking` in async contexts
3. **Offline-First Parsing**: SimpleParser always works, AIParser is optional enhancement
4. **Unified Error Type**: Single `AppError` across all crates for consistent error handling
5. **Hardcoded Worldview**: Project metadata extraction is deterministic, not dependent on AI

---

## 📊 Database Schema

All events stored in SQLite with full field support:

```sql
CREATE TABLE events (
    id TEXT PRIMARY KEY,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    date TEXT NOT NULL,
    time TEXT,
    end_time TEXT,
    event TEXT NOT NULL,
    notes TEXT,
    priority TEXT NOT NULL DEFAULT 'medium',
    category TEXT NOT NULL DEFAULT 'other',
    color TEXT,
    tags TEXT,                    -- JSON array
    status TEXT NOT NULL DEFAULT 'confirmed',
    visibility TEXT NOT NULL DEFAULT 'private',
    recurring TEXT,               -- JSON object
    reminder TEXT,                -- JSON object
    location TEXT,                -- JSON object
    metadata TEXT NOT NULL DEFAULT '{}'  -- JSON object
);
```

**Location**: `%APPDATA%\ubercalendurr\calendar.db` (Windows)  
**Migration**: Schema versioning table tracks applied migrations

---

## 🔌 Optional AI Integration

DeepSeek API integration is **completely optional**. The app works fully offline.

**To enable AI:**
1. Get API key from [DeepSeek](https://www.deepseek.com/)
2. Set `deepseek_api_key` in settings (or environment variable)
3. AIParser will be used as fallback if SimpleParser fails

**Worldview Hardcoded in Prompts:**
- S-DNA, KryptoClaw, Neural Draft LLC recognition
- Project metadata extraction rules
- Default inference (no clarification questions unless impossible)

---

## 📤 Export Formats

Export your data anytime:

```bash
📅> /export json
✅ Exported 47 events to ubercalendurr_export_20260118_143022.json

📅> /export csv
✅ Exported 47 events to ubercalendurr_export_20260118_143023.csv

📅> /export ics
✅ Exported 47 events to ubercalendurr_export_20260118_143024.ics
```

- **JSON**: Full event data, including metadata
- **CSV**: Spreadsheet-compatible format
- **ICS**: iCalendar format (compatible with Google Calendar, Outlook, etc.)

---

## 🛠️ Development

### Project Structure

- **Libraries** (`libraries/`): Shared Rust crates
- **Binaries** (`binaries/`): Terminal widget and GUI application
- **Frontend** (`binaries/calendar-gui/frontend/`): React + TypeScript + TailwindCSS

### Key Rust Crates

- `calendar-core`: Data models, validation, error types
- `storage-engine`: SQLite repository with synchronous operations
- `deepseek-client`: Optional AI client with rate limiting

### Key Frontend Technologies

- **React 18**: UI framework
- **TypeScript**: Type safety
- **TailwindCSS**: Styling
- **Tauri API**: Rust backend communication

### Running Tests

```bash
# Rust tests
cargo test --workspace

# Frontend tests (when added)
cd binaries/calendar-gui/frontend
npm test
```

### Code Quality

- **Rust**: Follows Rust best practices, uses `thiserror` for error handling
- **TypeScript**: Strict mode enabled, ESLint configured
- **Architecture**: Single responsibility, clear separation of concerns

---

## 🎯 Roadmap

### Completed ✅
- [x] Single-process architecture (removed IPC)
- [x] Complete database schema with all CalendarEvent fields
- [x] SimpleParser with regex-based offline parsing
- [x] Optional AI integration (DeepSeek)
- [x] Terminal widget with interactive loop
- [x] Tauri GUI with real backend connection
- [x] Export functionality (JSON, CSV, ICS)
- [x] Unified error handling
- [x] Project metadata extraction
- [x] Date navigation in GUI
- [x] Conflict detection
- [x] Basic recurring events

### In Progress 🚧
- [ ] Full test coverage
- [ ] Recurring event instance generation
- [ ] Advanced search and filtering
- [ ] Event reminders/notifications
- [ ] Calendar synchronization (CalDAV)

### Planned 📋
- [ ] Mobile app (React Native + shared Rust core)
- [ ] Web version
- [ ] Collaboration features
- [ ] Advanced AI prompts customization
- [ ] Plugin system

---

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

**Guidelines:**
- Follow Rust best practices
- Write tests for new features
- Update documentation
- Keep the offline-first philosophy in mind

---

## 📄 License

[Add your license here]

---

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for cross-platform desktop apps
- Uses [SQLite](https://www.sqlite.org/) for local storage
- Optional AI powered by [DeepSeek](https://www.deepseek.com/)
- Icons from [Lucide](https://lucide.dev/)

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/Ghostmonday/ubercalendurr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Ghostmonday/ubercalendurr/discussions)

---

**Made with ❤️ for developers who value their time and data sovereignty.**
