# Campus Timetable (Unofficial)

> **Disclaimer:** This is an independent, unofficial open-source student tool. It is neither affiliated with, maintained by, nor officially endorsed by Universiti Malaysia Perlis (UniMAP).

A fast, privacy-first cross-platform academic timetable for students, available as a lightweight desktop app (Windows, macOS, Linux), offline Progressive Web App (PWA), and mobile app.

---

## Architecture Overview

This project is structured as a unified multi-platform Cargo workspace with a single source of truth for all schedule logic:

- [`crates/timetable-core`](file:///c:/Users/swifty/Documents/-timetable-app-main/crates/timetable-core): Pure Rust schedule engine, reminder planner, RFC 5545 `.ics` generator, time calculation with fixed +08:00 offset, and schedule validator with fake clock test harness.
- [`crates/timetable-wasm`](file:///c:/Users/swifty/Documents/-timetable-app-main/crates/timetable-wasm): `wasm-bindgen` bindings running the Rust core directly in web browsers.
- [`src-tauri`](file:///c:/Users/swifty/Documents/-timetable-app-main/src-tauri): Native Tauri desktop shell with background system tray, native notifications, single-instance lock, safe link routing, and direct Windows wallpaper setters.
- [`web/`](file:///c:/Users/swifty/Documents/-timetable-app-main/web): Shared frontend (HTML5/CSS3/ES6) with PWA offline support and `backend.js` 3-tier adapter (`Tauri IPC` ➔ `WASM` ➔ `Client JS`).

---

## Features

- **Accurate Time Engine:** Computes active class, upcoming class, countdowns, and daily free gaps (including Friday prayer breaks).
- **Single Source of Truth:** `schedule.json` drives the web view, calendar `.ics` files, and AI crawler guides (`llms.txt`).
- **Native Desktop Integration:**
  - System tray icon with live status reflection and tooltip display.
  - Native system notifications for upcoming classes.
  - Set generated timetable graphic directly as Windows desktop wallpaper.
  - Close-to-tray and single-instance protection.
  - External link isolation (opens group links in default system browser).
- **Offline First & PWA:** Service worker caching, homescreen installation, and zero mandatory cloud dependencies.

---

## Building and Running

### Prerequisites

- **Rust & Cargo:** (1.80+)
- **Windows:** MSVC C++ Build Tools or MinGW-w64 / w64devkit.
- **Node.js / npm:** (optional, only if using external bundlers; workspace builds with native Cargo).

### 1. Test Core Logic & Schedule

```bash
# Run unit tests (fake clock, weekend checks, midnight rollover, free gaps)
cargo test -p timetable-core
```

### 2. Regenerate Schedule Artifacts

```bash
# Rebuild web/llms.txt and web/schedule.ics from schedule.json
cargo run --bin generate -p timetable-core
```

### 3. Run Desktop Application (Tauri)

```bash
# Debug desktop build
cargo run --manifest-path src-tauri/Cargo.toml
```

### 4. Serve Web Frontend Locally

```powershell
# Built-in PowerShell HTTP server
powershell -ExecutionPolicy Bypass -File scripts/serve.ps1
```
Open `http://127.0.0.1:8080/` in your browser.

---

## Security & Privacy Policy

- **No Remote Telemetry:** All schedule calculations, notifications, and user preferences (reduce motion, custom group links, layout toggles) remain exclusively on your device.
- **Strict Content Security Policy (CSP):** The Tauri desktop webview enforces an explicit CSP and restricts external navigations to protect against untrusted remote code execution.
- **Minimal System Permissions:** Capabilities are scoped strictly to native notifications and core IPC. Shell execution commands are disabled.

---

## License

MIT License. See [LICENSE](file:///c:/Users/swifty/Documents/-timetable-app-main/LICENSE) for details.
