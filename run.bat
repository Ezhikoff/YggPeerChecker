@echo off
echo ========================================
echo  YggPeerChecker - Build and Run
echo ========================================
echo.

echo [1/2] Building project...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo BUILD FAILED!
    echo Please ensure Rust is installed: https://rustup.rs/
    pause
    exit /b 1
)

echo.
echo [2/2] Running application...
cargo run --release
pause
