# Contributing

Thanks for your interest in improving YggPeerChecker!

## Getting started

```bash
git clone https://github.com/Ezhikoff/YggPeerChecker.git
cd YggPeerChecker
cargo build --release
```

Requires Rust (stable) via [rustup](https://rustup.rs/).

On Linux, install system dependencies:
```bash
sudo apt install libgtk-3-dev libxkbcommon-dev
```

## Making changes

1. Fork the repo and create a branch from `main`.
2. Make your changes.
3. Ensure the build passes:
   ```bash
   cargo clippy --release -- -D warnings
   cargo build --release
   ```
4. Update `CHANGELOG.md` if the change is user-facing.
5. Open a Pull Request.

## Code style

- All user-facing strings are in **Russian**.
- Code comments are in English.
- Run `cargo fmt` before committing.

## Reporting bugs

Use the [Bug Report template](https://github.com/Ezhikoff/YggPeerChecker/issues/new?template=bug_report.yml).
