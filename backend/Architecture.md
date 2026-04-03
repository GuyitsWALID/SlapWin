# System Architecture

## High-Level Architecture Diagram

```
+---------------------------------------------------------------------+
|                         UI LAYER (Webview)                          |
|  +-------------------+  +------------------+  +-------------------+  |
|  |  Main Window      |  |  Settings Panel  |  |  License Dialog   |  |
|  |  (HTML/CSS/JS)    |  |  (HTML/CSS/JS)   |  |  (HTML/CSS/JS)    |  |
|  +--------+----------+  +--------+---------+  +--------+----------+  |
|           |                      |                      |            |
+-----------+----------------------+----------------------+------------+
            |                      |                      |
            v                      v                      v
+-----------+---------------------------------------------------------+
|  Tauri IPC BRIDGE (command invocations via @tauri-apps/api/core)   |
+--------------------------------+-----------------------------------+
                                 |
            +--------------------+--------------------+
            |                    |                    |
            v                    v                    v
+-------------------+  +-------------------+  +-------------------+
|   CORE MODULES    |  |  SYSTEM MODULES   |  |  UTIL MODULES     |
|                   |  |                   |  |                   |
| - App State       |  | - File System     |  | - License Manager |
| - Config Manager  |  | - Process Mgmt    |  | - Key Storage     |
| - Event Emitter   |  | - Env Detection   |  | - Logger          |
| - Timer/Scheduler |  | - Window Mgmt     |  | - Validation      |
+-------------------+  | - Hardware Info   |  +-------------------+
                       +-------------------+
                                 |
                                 v
+---------------------------------------------------------------------+
|  OS / HARDWARE LAYER                                                |
|  (Windows API calls via `windows-rs` where needed)                  |
+---------------------------------------------------------------------+
```

## Data Flow

```
User Action → Frontend JS → Tauri Command → Rust Backend → OS API → Response → UI Update
```

## Project Module Dependencies

```
src/
├── main.rs                    # Entry point, Tauri setup (no deps)
├── commands/                  # IPC handlers (depends on core, system)
│   ├── mod.rs
│   ├── app.rs
│   ├── license.rs
│   └── system.rs
├── core/                      # Business logic (depends on system)
│   ├── mod.rs
│   ├── config.rs
│   ├── state.rs
│   └── events.rs
├── system/                    # OS interaction (no internal deps)
│   ├── mod.rs
│   ├── file.rs
│   ├── process.rs
│   └── hardware.rs
├── utils/                     # Shared utilities (no internal deps)
│   ├── mod.rs
│   ├── license.rs
│   └── logger.rs
└── lib.rs                     # Library exports (wire all modules)
```

## Architecture Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Framework | Tauri 2.x | <15MB binary, cross-platform future |
| Frontend | Vanilla HTML/CSS/JS | Minimal dependencies, fastest load |
| Backend | Rust | Performance, safety, small footprint |
| State Mgmt | Tauri `State<>` | Built-in, no extra deps |
| IPC | Tauri Commands | Type-safe, async, minimal overhead |
| Storage | JSON file + Registry | Simple, no SQLite overhead |
| Licensing | Custom JWT-like tokens | Verify offline, no license server |
| Logging | `tracing` crate | Zero-cost profiling, structured output |

## Key Design Principles

1. **Zero Unnecessary Dependencies** — Every crate must justify its weight
2. **Fast Startup** — Target <200ms cold start to UI responsiveness
3. **Minimal Memory** — Target <100MB resident set at idle
4. **Graceful Degradation** — Non-critical features fail silently
5. **Testable by Design** — Core logic isolated from IO/UI
