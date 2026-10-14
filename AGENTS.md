# AGENTS.md

## Project shape

Single-file Rust GUI app. All application code lives in `src/main.rs` (~950 lines). There are no modules, no tests, no CI, and no lint/format config.

## Build gotcha: `app.ico` is generated, not checked in

`build.rs` writes `app.ico` to the repo root at build time. `main.rs` embeds it via `include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/app.ico"))`. The file is gitignored. A clean `cargo build` works because `build.rs` runs first, but if you delete `target/` and try to compile `main.rs` in isolation (e.g. with `rustc`), it will fail. Always use `cargo build`.

## Commands

```bash
cargo build --release          # release build
cargo run --release            # build + run GUI
cargo build                    # debug build (GUI still works)
cargo deb                      # Debian .deb package (requires cargo-deb)
```

There is no test, lint, or typecheck step beyond `cargo build`.

## Cross-compilation for Windows

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Requires MinGW-w64 (`apt install gcc-mingw-w64-x86-64`). The `.cargo/config.toml` adds `-subsystem,windows` linker flag.

## iced version

Cargo.toml pins **iced 0.14**.

## Windows-specific wiring

- `.cargo/config.toml` adds `-subsystem,windows` linker flag to hide the console window.
- `build.rs` uses `winresource` (Windows-only build-dependency) to embed the icon into the `.exe`.
- On Linux, `build.rs` still generates `app.ico` (for the `include_bytes!` in main.rs) but skips the `winresource` step.

## Protocol checkers — what's real vs. stub

| Protocol | Status |
|----------|--------|
| TCP, TLS, QUIC, WS, WSS | Implemented |
| SOCKS, SOCKS+TLS | Stub — always returns "not implemented" error |
| UNIX | Stub — returns "not supported on Windows" error |

QUIC and WSS use `SkipServerVerification` (accepts any TLS cert). TLS uses `webpki-roots` for real verification.

## Parallel checking

`check_peers` uses `futures::future::join_all` to check all peers concurrently. Each peer gets its own async task. This is the main performance feature — total check time is max(individual times) instead of sum.

## Auto-refresh

When enabled, after each `CheckDone` the app schedules a one-shot timer via `Task::perform(tokio::time::sleep(...))`. When the timer fires (`AutoRefreshTick`), it triggers `CheckAll` again. Toggling off breaks the chain.

## Save/Load

Peer list is persisted to a platform-appropriate config file:
- Linux: `~/.config/ygg-peer-checker/peers.txt`
- Windows: `%APPDATA%\ygg-peer-checker\peers.txt`

Auto-loaded on startup. Manual save/load via buttons.

## Shared TLS configs

TLS and WSS configs are created once via `std::sync::OnceLock` and reused across all checks. `tls_connector()` returns a shared `TlsConnector` (webpki-roots). `wss_tls_config()` returns a shared `Arc<ClientConfig>` (SkipServerVerification).

## Bilingual UI convention

All user-facing strings (button labels, status text, error messages) are in **Russian**. Code comments are in English. Section separators in `main.rs` use `// ── Section name ──` with Russian section names.

## Connection timeout

All protocol checkers use a hard **3-second** timeout. Speed measurement (TCP only) runs for 1 second after a successful connect.
