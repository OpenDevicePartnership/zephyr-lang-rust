# Setup script for thermal-service QEMU development environment
# Run this once to install dependencies needed for QEMU emulation on Windows.
#
# Usage: .\scripts\setup-qemu.ps1
#
# Prerequisites:
#   - winget (ships with Windows 10/11)
#   - Zephyr SDK installed (provides toolchain, not QEMU)
#   - west + Python venv already configured

#Requires -Version 5.1
$ErrorActionPreference = "Stop"

Write-Host "=== Thermal Service QEMU Setup ===" -ForegroundColor Cyan

# 1. Check/install QEMU
$qemuPath = "C:\Program Files\qemu\qemu-system-riscv32.exe"
if (Test-Path $qemuPath) {
    $ver = & $qemuPath --version | Select-Object -First 1
    Write-Host "[OK] QEMU already installed: $ver" -ForegroundColor Green
} else {
    Write-Host "[..] Installing QEMU via winget..." -ForegroundColor Yellow
    winget install SoftwareFreedomConservancy.QEMU --accept-package-agreements --accept-source-agreements
    if (-not (Test-Path $qemuPath)) {
        Write-Error "QEMU installation failed. Install manually from https://qemu.weilnetz.de/w64/"
        exit 1
    }
    $ver = & $qemuPath --version | Select-Object -First 1
    Write-Host "[OK] QEMU installed: $ver" -ForegroundColor Green
}

# 2. Check Rust target
$targets = rustup target list --installed
if ($targets -match "riscv32i-unknown-none-elf") {
    Write-Host "[OK] Rust target riscv32i-unknown-none-elf installed" -ForegroundColor Green
} else {
    Write-Host "[..] Adding Rust target riscv32i-unknown-none-elf..." -ForegroundColor Yellow
    rustup target add riscv32i-unknown-none-elf
    Write-Host "[OK] Rust target added" -ForegroundColor Green
}

# 3. Verify west is available
if (Get-Command west -ErrorAction SilentlyContinue) {
    Write-Host "[OK] west found: $(west --version)" -ForegroundColor Green
} else {
    Write-Error "west not found. Activate your Python venv first: .venv\Scripts\Activate.ps1"
    exit 1
}

Write-Host ""
Write-Host "=== Setup Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "To build and run:" -ForegroundColor White
Write-Host '  $env:QEMU_BIN_PATH = "C:\Program Files\qemu"' -ForegroundColor Gray
Write-Host '  west build -b qemu_riscv32 modules/lang/rust/samples/thermal-service --no-sysbuild' -ForegroundColor Gray
Write-Host '  west build -t run' -ForegroundColor Gray
Write-Host ""
Write-Host "Or use the run script:" -ForegroundColor White
Write-Host '  .\modules\lang\rust\samples\thermal-service\scripts\run-qemu.ps1' -ForegroundColor Gray
