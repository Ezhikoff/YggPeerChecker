# YggPeerChecker / Проверка пиров Yggdrasil

[![GitHub release](https://img.shields.io/github/release/Ezhikoff/YggPeerChecker.svg?label=latest%20release)](https://github.com/Ezhikoff/YggPeerChecker/releases/latest)
[![License](https://img.shields.io/github/license/Ezhikoff/YggPeerChecker.svg)](https://github.com/Ezhikoff/YggPeerChecker/blob/main/LICENSE)

**EN:** A GUI utility for checking, sorting, and monitoring Yggdrasil Network peers.  
**RU:** Утилита с графическим интерфейсом для проверки, сортировки и мониторинга пиров сети Yggdrasil.

---

## Features / Возможности

| EN | RU |
|----|----|
| Support for 8 protocols | Поддержка 8 протоколов |
| Measure ping & connection speed | Измерение пинга и скорости подключения |
| Color-coded peer status | Цветовая индикация статуса пиров |
| Sort by ping, speed, or status | Сортировка по пингу, скорости или статусу |
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

**Windows (PowerShell):**
```powershell
cd C:\path\to\YggPeerChecker
cargo run --release
```

**Linux (WSL or native):**
```bash
cd /home/mike/DEV/YggPeerChecker
./target/release/ygg-peer-checker
```

### Output binary / Исполняемый файл

| Platform / Платформа | Path / Путь |
|---------------------|-------------|
| **Linux** | `target/release/ygg-peer-checker` (ELF, ~23MB) |
| **Windows** | `target\release\ygg-peer-checker.exe` |

> **Note / Примечание:** If you built the project in WSL, the Linux binary will not run on Windows. Build from Windows PowerShell to get the `.exe`.  
> Если проект был собран в WSL, Linux-бинарник не запустится на Windows. Соберите из PowerShell Windows, чтобы получить `.exe`.

---

## Usage / Использование

1. **EN:** Paste peer URLs (one per line) into the text field → Click **"Добавить / Add"**  
   **RU:** Вставьте URL пиров (по одному на строку) в текстовое поле → Нажмите **"Добавить / Add"**

2. **EN:** Click **"Проверить все / Check All"** to start checking  
   **RU:** Нажмите **"Проверить все / Check All"** для начала проверки

3. **EN:** Use the **"Сортировка / Sort"** dropdown to sort by ping, speed, or status  
   **RU:** Используйте выпадающий список **"Сортировка / Sort"** для сортировки по пингу, скорости или статусу

4. **EN:** Green = fast peers, Red = unreachable  
   **RU:** Зелёный = быстрые пиры, Красный = недоступные

---

## Tech Stack / Технологии

- **GUI:** [iced](https://github.com/iced-rs/iced) 0.13 — native cross-platform GUI
- **Async runtime:** [tokio](https://github.com/tokio-rs/tokio)
- **QUIC:** [quinn](https://github.com/quinn-rs/quinn) 0.11
- **TLS:** [rustls](https://github.com/rustls/rustls) 0.23
- **WebSocket:** [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)

---

## License / Лицензия

MIT License — see [LICENSE](LICENSE)
