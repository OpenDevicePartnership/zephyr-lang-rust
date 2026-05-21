# Build and run thermal-service on QEMU RISC-V 32
# This is the one-shot "just make it go" script.
#
# Usage: .\scripts\run-qemu.ps1 [-Clean]
#
# Prereqs: Run setup-qemu.ps1 first (one-time).

param(
    [switch]$Clean
)

$ErrorActionPreference = "Stop"
$env:QEMU_BIN_PATH = "C:\Program Files\qemu"

# Verify QEMU is installed
if (-not (Test-Path "$env:QEMU_BIN_PATH\qemu-system-riscv32.exe")) {
    Write-Error "QEMU not found. Run setup-qemu.ps1 first."
    exit 1
}

# Navigate to workspace root (parent of modules/)
$scriptDir = Split-Path -Parent $PSScriptRoot
$workspaceRoot = (Get-Item $scriptDir).Parent.Parent.Parent.Parent.FullName
Push-Location $workspaceRoot

try {
    $buildArgs = @("-b", "qemu_riscv32", "modules/lang/rust/samples/thermal-service", "--no-sysbuild")
    if ($Clean) {
        $buildArgs += "-p"
    }

    Write-Host "Building thermal-service for qemu_riscv32..." -ForegroundColor Cyan
    west build @buildArgs
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

    Write-Host ""
    Write-Host "Running on QEMU (Ctrl+A, X to exit)..." -ForegroundColor Green
    Write-Host ""
    west build -t run
} finally {
    Pop-Location
}
