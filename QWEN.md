# YggPeerChecker - Project Context

## Project Overview

**YggPeerChecker** is a cross-platform GUI utility for checking, sorting, and monitoring peers on the **Yggdrasil Network**. It is written in **Rust** and uses the **iced** framework for its native cross-platform GUI.

### Key Features
- Supports 8 protocols: TCP, TLS, QUIC, WS, WSS, SOCKS, SOCKS+TLS, UNIX
- Measures ping latency and connection speed
- Color-coded peer status (green/yellow/red based on ping)
- Sort peers by ping, speed, status, or insertion order
- Real-time online/offline/pending statistics
- Cross-platform (Windows, Linux)

### Tech Stack
- **GUI:** [iced](https://github.com/iced-rs/iced) 0.13 with tokio integration
- **Async runtime:** tokio (full features)
- **QUIC:** quinn 0.11
- **TLS:** rustls 0.23, tokio-rustls 0.26
- **WebSocket:** tokio-tungstenite 0.26
- **Certificate generation:** rcgen 0.13
- **Time/Date:** chrono 0.4

## Project Structure

```
YggPeerChecker/
├── Cargo.toml          # Project manifest, dependencies
├── build.rs            # Windows icon generation (ICO from BMP)
├── run.bat             # Windows batch launcher
├── src/
│   └── main.rs         # Entire application (~849 lines)
├── .cargo/
│   └── config.toml     # Windows linker flags (subsystem=windows)
└── README.md           # Bilingual documentation (EN/RU)
```

The application is a **single-file Rust application** (`src/main.rs`). All code — GUI logic, protocol checkers, styling, and models — lives in one file.

## Architecture

### Models
- **`Protocol`** — Enum for supported protocols (Tcp, Tls, Quic, Ws, Wss, Socks, SocksTls, Unix)
- **`Peer`** — Parsed peer with protocol, address, port, raw string
- **`PeerStatus`** — Pending, Checking, Online { ping_ms, speed_mbps }, Offline { error }
- **`PeerEntry`** — Combines Peer + PeerStatus
- **`SortBy`** — Order, Ping, Speed, Status

### Application (`YggPeerChecker`)
Uses the **iced Elm-like architecture**:
- `update()` — handles messages (AddPeer, CheckAll, CheckDone, SortChanged, RemovePeer, etc.)
- `view()` — renders the UI (header, input area, peer list, stats)
- `main()` — entry point via `iced::application()`

### Protocol Checkers
Each protocol has its own async check function:
- **TCP:** `tokio::net::TcpStream::connect` + speed measurement via write throughput
- **TLS:** TCP + rustls TLS handshake with webpki roots
- **QUIC:** quinn endpoint with self-signed cert, SkipServerVerification
- **WS/WSS:** tokio-tungstenite connect_async (with optional TLS connector)
- **SOCKS/SOCKS+TLS:** Stub — not implemented
- **UNIX:** Stub — not supported on Windows

## Building and Running

### Requirements
- **Rust** (edition 2021+) — install from https://rustup.rs/

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

### Output Binary
| Platform | Path |
|----------|------|
| Linux | `target/release/ygg-peer-checker` |
| Windows | `target\release\ygg-peer-checker.exe` |

### Platform-Specific Notes
- **Windows:** The `.cargo/config.toml` sets `-subsystem,windows` to hide the console window. The `build.rs` generates an embedded ICO icon.
- **Linux:** No special configuration needed. The build.rs generates `app.ico` for reference only.
- **Cross-compilation:** Building in WSL produces a Linux binary; build from Windows PowerShell for `.exe`.

## Development Conventions

- **Single-file architecture:** All application code is in `src/main.rs` (~849 lines)
- **Bilingual UI:** All user-facing text is in Russian (with English comments)
- **Color conventions:** Green (<100ms), Yellow-Green (100-300ms), Yellow (>300ms), Red (unreachable)
- **Timeout:** 3 seconds for all connection attempts
- **Error messages:** In Russian
- **No tests:** The project currently has no test suite

## Peer URL Format

Each peer is specified on a separate line:
```
tcp://89.44.86.85:65535
quic://[2a09:5302:ffff::132a]:65535
tls://94.156.181.85:65535
ws://89.44.86.85:8080
wss://89.44.86.85:443
```

## Key Implementation Details

- **QUIC** uses a custom `SkipServerVerification` certificate verifier (accepts any cert)
- **TLS** uses webpki roots for server verification
- **WSS** also uses `SkipServerVerification` (dangerous mode)
- **Speed measurement** is done via TCP write throughput over 1 second
- **Peer parsing** handles both IPv4 and IPv6 (bracket notation)
