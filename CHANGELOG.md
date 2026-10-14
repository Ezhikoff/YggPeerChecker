# Changelog

All notable changes to this project are documented here.

## [v0.2.0] — 2026-10-14

### Added
- Parallel peer checking (all peers checked concurrently via `join_all`)
- Auto-refresh timer with configurable interval (30s / 1min / 5min / 10min)
- Save/load peer list to config file with auto-load on startup
- Shared TLS configs via `OnceLock` (eliminate per-check allocations)
- DNS resolution for hostname-based peers (was IP-only)
- Proper SNI (Server Name Indication) for TLS/QUIC/WSS connections
- RPM packaging support (`cargo generate-rpm`)
- GitHub Actions CI with auto-release on tag push

### Changed
- Updated `tokio-tungstenite` 0.26 → 0.30
- Updated `rcgen` 0.13 → 0.14 (adapted to `signing_key` rename)
- Updated `webpki-roots` 0.26 → 1.0
- Icon now embedded in Windows .exe during cross-compilation
- Rewrote README (bilingual EN/RU)

### Removed
- Unused dependency `chrono`
- Redundant `#![windows_subsystem = "windows"]` attribute
- `QWEN.md` and related `.gitignore` entries

## [v0.1.0] — 2025-01-01

### Added
- Initial release
- TCP, TLS, QUIC, WS, WSS protocol checkers
- Color-coded peer status with ping/speed measurement
- Sorting by ping, speed, or status
- Bilingual Russian UI
