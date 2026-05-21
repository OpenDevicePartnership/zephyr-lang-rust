# Thermal Service Sample (Rust + Zephyr)

A working demo of the [ODP `thermal-service`](https://github.com/OpenDevicePartnership/embedded-services) crate
on Zephyr RTOS — running entirely in QEMU with a mock temperature sensor.
No hardware required.

**Stack:** Rust + Embassy async executor → `thermal-service` crate → Zephyr sensor API → mock C driver

---

## Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| Windows 10/11 | — | `winget` must be available |
| Python | 3.10+ | For `west` and Zephyr scripts |
| Rust | stable | `rustup` installed |
| Zephyr SDK | 1.0.1+ | Provides cross-compiler (we skip its QEMU) |

---

## Quick Start

### 1. Clone

```powershell
mkdir zephyrproject ; cd zephyrproject

# Zephyr core (includes Rust build-system patches)
git clone -b qemu-odp-thermal https://github.com/kurtjd/zephyr.git

# Rust module (thermal-service sample + mock sensor driver)
mkdir modules\lang -Force
git clone -b qemu-odp-thermal https://github.com/kurtjd/zephyr-lang-rust.git modules\lang\rust
```

### 2. Initialize west workspace

```powershell
python -m venv .venv
.\.venv\Scripts\Activate.ps1
pip install west
west init -l zephyr
west update
pip install -r zephyr\scripts\requirements.txt
```

### 3. Install tools

```powershell
.\modules\lang\rust\samples\thermal-service\scripts\setup-qemu.ps1
```

Installs standalone QEMU (via `winget`) and the `riscv32i-unknown-none-elf` Rust target.

### 4. Build & Run

```powershell
.\modules\lang\rust\samples\thermal-service\scripts\run-qemu.ps1
```

Or manually:

```powershell
$env:QEMU_BIN_PATH = "C:\Program Files\qemu"
west build -b qemu_riscv32 modules/lang/rust/samples/thermal-service --no-sysbuild
west build -t run
```

### 5. Observe

```
[00:00:00.006,000] <inf> rust: rustapp: Temperature read out success
[00:00:00.006,000] <inf> rust: rustapp:     25.0 Celsius
[00:00:02.008,000] <inf> rust: rustapp: Temperature read out success
[00:00:02.009,000] <inf> rust: rustapp:     25.500000 Celsius
[00:00:04.011,000] <inf> rust: rustapp: Temperature read out success
[00:00:04.011,000] <inf> rust: rustapp:     26.0 Celsius
...
```

The mock sensor ramps from 25°C → 45°C in 0.5° steps (one reading every ~2 seconds).

**Exit QEMU:** `Ctrl+A`, then `X`

---

## Why two repos?

| Repo | Branch | What's changed |
|------|--------|----------------|
| [`kurtjd/zephyr`](https://github.com/kurtjd/zephyr/tree/qemu-odp-thermal) | `qemu-odp-thermal` | CMake patches for Rust toolchain integration |
| [`kurtjd/zephyr-lang-rust`](https://github.com/kurtjd/zephyr-lang-rust/tree/qemu-odp-thermal) | `qemu-odp-thermal` | Thermal-service sample, mock driver, DTS bindings |

The Zephyr core patches enable the Rust build system. Once those land upstream, only the Rust module repo will be needed.

---

## Why `QEMU_BIN_PATH`?

The Zephyr SDK 1.0.1 ships a QEMU binary that's missing DLLs on Windows
(`libgcc_s_seh-1.dll`). We use the standalone QEMU from
[qemu.weilnetz.de](https://qemu.weilnetz.de/) instead. The `QEMU_BIN_PATH`
env var tells Zephyr's CMake to use it.

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│  thermal-service crate (Rust, Embassy async)    │
│    - Polls sensor on a 2-second profile         │
│    - Logs temperature readings                  │
├─────────────────────────────────────────────────┤
│  Zephyr Sensor API (TemperatureSensor trait)    │
├─────────────────────────────────────────────────┤
│  mock_temp_sensor.c (C driver)                  │
│    - No I2C, no hardware                        │
│    - Ramps 25°C → 45°C in 0.5° steps           │
├─────────────────────────────────────────────────┤
│  Zephyr RTOS kernel (qemu_riscv32)             │
│    - 10 kHz tick rate                           │
│    - QEMU icount aligned to wall clock          │
└─────────────────────────────────────────────────┘
```

---

## File Layout

```
samples/thermal-service/
├── src/lib.rs                              # Rust app entry point
├── CMakeLists.txt                          # Build config + driver source
├── Cargo.toml                              # Rust dependencies
├── Cargo.lock                              # Pinned dependency versions
├── prj.conf                                # Zephyr project config
├── sample.yaml                             # Zephyr test metadata
├── .cargo/config.toml                      # Cargo/rust-analyzer settings
├── boards/
│   ├── qemu_riscv32.conf                   # Kconfig: no I2C, 10kHz tick
│   └── qemu_riscv32.overlay                # DTS: mock sensor node
├── drivers/mock_temp_sensor/
│   ├── mock_temp_sensor.c                  # Mock C sensor driver
│   ├── CMakeLists.txt                      # Driver build config
│   └── Kconfig                             # Driver Kconfig options
├── dts/bindings/sensor/
│   └── zephyr,mock-temp-sensor.yaml        # DTS binding definition
└── scripts/
    ├── setup-qemu.ps1                      # One-time env setup
    └── run-qemu.ps1                        # Build + run helper
```
