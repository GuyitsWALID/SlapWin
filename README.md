# SlapMAC

A lightweight Windows utility inspired by slapmac.com.

## Tech Stack

- **Framework**: Tauri 2.x (Rust)
- **Frontend**: Vanilla HTML/CSS/JS
- **Size Target**: < 10 MB binary
- **Memory**: < 100 MB idle

## Project Structure

```
windows-utility/
├── src-frontend/           # Frontend assets (index.html, styles.css, main.js)
│   ├── index.html
│   ├── styles.css
│   └── main.js
├── src/                   # Rust source
│   ├── commands/          # Tauri IPC commands
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   ├── license.rs
│   │   └── system.rs
│   ├── core/              # Business logic
│   │   ├── mod.rs
│   │   └── state.rs
│   ├── system/            # OS abstraction
│   │   └── helpers.rs
│   ├── utils/             # Utilities
│   │   ├── mod.rs
│   │   ├── license.rs
│   │   └── logger.rs
│   ├── main.rs           # Entry point
│   └── lib.rs           # Library root
├── src-tauri/            # Tauri configuration
│   ├── tauri.conf.json
│   ├── capabilities/default.json
│   └── build.rs
├── Cargo.toml
├── package.json
├── build.rs
└── README.md
```

## Getting Started

### Prerequisites

- Rust (stable)
- Node.js (for Tauri CLI)
- For Windows: Visual Studio Build Tools with C++ tools

### Installation

```bash
# Install dependencies
npm install

# Run in dev mode
npm run dev

# Build for production
npm run build
```

Output binary: `src-tauri/target/release/slapmac.exe`

## Testing Strategy

### 1. Unit Tests

- Location: `src/` modules with `#[cfg(test)]` tests
- Framework: Rust `std::test` (no extra dependency)
- Coverage target: 80% for core logic
- Mock external dependencies using traits

Example: `tests/` directory or test modules inside each file.

### 2. Integration Tests

- IPC command tests: verify commands return expected results
- State management tests: config load/save, license validation
- File I/O tests: config path handling

Location: `tests/` with separate test binaries
```bash
cargo test
```

### 3. UI Tests (Optional)

- Tauri does not have built-in UI testing yet
- Optionally use Playwright or similar with Tauri's webview access
- Manual sanity checks for now

### 4. Performance Benchmarks

Run with:
```bash
cargo bench
```

Key benchmarks (to implement in `benches/`):
- Command latency: < 5ms for internal commands
- Startup time: cold start < 200ms to UI ready
- Memory usage: < 100MB at idle
- Binary size: < 10MB compressed

## Performance Benchmarks

### Target Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Binary size | < 10 MB | File size of exe + resources |
| Startup time | < 200ms | Process creation to window show |
| Command latency | < 5ms | IPC round-trip for simple commands |
| Memory (idle) | < 100 MB RSS | Process Explorer |
| Memory (active) | < 150 MB RSS | Process Explorer |
| CPU (idle) | < 1% | Task manager |

### Load Testing

- Simulate 1000 rapid-fire IPC calls
- Measure memory stability (no leaks)
- Ensure UI remains responsive

## Development Timeline

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| **Sprint 1**: Foundation | 2 days | Tauri setup, basic UI, IPC commands working |
| **Sprint 2**: Licensing | 2 days | Trial activation, license persistence, validation logic |
| **Sprint 3**: Core Feature | 3 days | Main utility functionality, performance tuning |
| **Sprint 4**: Polish & Build | 2 days | Icon, branding, installer creation, testing |
| **Sprint 5**: Final Testing | 1 day | Bug fixes, performance validation, release candidate |

**Total**: ~10 business days

### Dependencies & Risks

- Tauri 2.x stability: Monitor alpha/beta releases, pin version
- Windows code signing: Need certificate (~$50/year)
- Distribution: NSIS installer or Microsoft Store (MSIX)

## License

All rights reserved. This is proprietary commercial software.

## Contact

Support: support@slapmac.com
