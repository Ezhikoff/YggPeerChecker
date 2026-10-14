# YggPeerChecker

[![Rust](https://github.com/Ezhikoff/YggPeerChecker/actions/workflows/rust.yml/badge.svg)](https://github.com/Ezhikoff/YggPeerChecker/actions/workflows/rust.yml)
[![Release](https://img.shields.io/github/v/release/Ezhikoff/YggPeerChecker.svg?label=release)](https://github.com/Ezhikoff/YggPeerChecker/releases)
[![License](https://img.shields.io/github/license/Ezhikoff/YggPeerChecker.svg)](https://github.com/Ezhikoff/YggPeerChecker/blob/main/LICENSE)

A GUI utility for checking, sorting, and monitoring Yggdrasil Network peers.

Утилита с графическим интерфейсом для проверки, сортировки и мониторинга пиров сети Yggdrasil.

---

## Features / Возможности

| EN | RU |
|----|----|
| Support for 8 protocols | Поддержка 8 протоколов |
| **Parallel peer checking** | **Параллельная проверка пиров** |
| Measure ping & connection speed | Измерение пинга и скорости подключения |
| Color-coded peer status | Цветовая индикация статуса пиров |
| Sort by ping, speed, or status | Сортировка по пингу, скорости или статусу |
| **Auto-refresh timer (30s / 1min / 5min / 10min)** | **Автообновление (30с / 1мин / 5мин / 10мин)** |
| **Save/load peer list to file** | **Сохранение/загрузка списка пиров в файл** |
| Add/remove peers dynamically | Динамическое добавление/удаление пиров |
| Real-time online/offline/pending stats | Статистика онлайн/офлайн/ожидание в реальном времени |
| Cross-platform (Windows, Linux) | Кроссплатформенность (Windows, Linux) |

---

## Color Indication / Цветовая индикация

| Ping / Пинг | Color / Цвет | Quality / Качество |
|-------------|-------------|-------------------|
| < 100ms | 🟢 Green / Зелёный | Excellent / Отлично |
| 100–300ms | 🟡 Yellow-Green / Жёлто-зелёный | Good / Хорошо |
| > 300ms | 🟠 Yellow / Жёлтый | Acceptable / Приемлемо |
| Unreachable | 🔴 Red / Красный | Offline / Не доступен |

---

## Supported Protocols / Поддерживаемые протоколы

| Protocol | Status / Статус | Example / Пример |
|----------|----------------|-----------------|
| **TCP** | ✅ Implemented / Реализован | `tcp://89.44.86.85:65535` |
| **TLS** | ✅ Implemented / Реализован | `tls://89.44.86.85:65535` |
| **QUIC** | ✅ Implemented / Реализован | `quic://[2a09:5302:ffff::132a]:65535` |
| **WS** (WebSocket) | ✅ Implemented / Реализован | `ws://89.44.86.85:8080` |
| **WSS** (WS over TLS) | ✅ Implemented / Реализован | `wss://89.44.86.85:443` |
| **SOCKS** | ⏳ Placeholder / Заглушка | `socks://[proxy]:port/[host]:port` |
| **SOCKS+TLS** | ⏳ Placeholder / Заглушка | `sockstls://[proxy]:port/[host]:port` |
| **UNIX** | ❌ Not on Windows / Не для Windows | `unix:///path/to/socket` |

---

## Peer Format / Формат пиров

Each peer on a separate line / Каждый пир на отдельной строке:

```
tcp://89.44.86.85:65535
quic://89.44.86.85:65535
tls://94.156.181.85:65535
ws://89.44.86.85:8080
wss://89.44.86.85:443
tcp://[2a09:5302:ffff::132a]:65535
quic://[2a09:5302:ffff::132a]:65535
tls://[2a09:5302:ffff::132a]:65535
```

---

## Save/Load / Сохранение и загрузка

Peer list is saved to a config file and auto-loaded on startup.

Список пиров сохраняется в конфигурационный файл и автоматически загружается при запуске.

| Platform / Платформа | Path / Путь |
|---------------------|-------------|
| **Linux** | `~/.config/ygg-peer-checker/peers.txt` |
| **Windows** | `%APPDATA%\ygg-peer-checker\peers.txt` |

---

## Build & Run / Сборка и запуск

### Requirements / Требования

- **Rust** — install from https://rustup.rs/
- **Windows 10/11** or **Linux with GUI**

### Build / Сборка

```bash
cargo build --release
```

### Run / Запуск

```bash
cargo run --release
```

### Output binary / Исполняемый файл

| Platform / Платформа | Path / Путь |
|---------------------|-------------|
| **Linux** | `target/release/ygg-peer-checker` |
| **Windows** | `target\release\ygg-peer-checker.exe` |

---

## Usage / Использование

1. Paste peer URLs (one per line) into the text field → Click **"Добавить"**
   Вставьте URL пиров (по одному на строку) в текстовое поле → Нажмите **"Добавить"**

2. Click **"Проверить все"** to start parallel checking
   Нажмите **"Проверить все"** для начала параллельной проверки

3. Use **"Сортировка"** dropdown to sort by ping, speed, or status
   Используйте **"Сортировка"** для сортировки по пингу, скорости или статусу

4. Enable **"Автообновление"** for automatic periodic re-checking
   Включите **"Автообновление"** для автоматической периодической перепроверки

5. Use **"Сохранить"** / **"Загрузить"** to persist the peer list
   Используйте **"Сохранить"** / **"Загрузить"** для сохранения списка пиров

---

## Tech Stack / Технологии

- **GUI:** [iced](https://github.com/iced-rs/iced) 0.14 — native cross-platform GUI
- **Async runtime:** [tokio](https://github.com/tokio-rs/tokio)
- **QUIC:** [quinn](https://github.com/quinn-rs/quinn) 0.11
- **TLS:** [rustls](https://github.com/rustls/rustls) 0.23
- **WebSocket:** [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite) 0.30

---

## License / Лицензия

MIT License — see [LICENSE](LICENSE)
