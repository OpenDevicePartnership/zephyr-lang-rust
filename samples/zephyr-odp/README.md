# Zephyr ODP Integration PoC
This project is a proof-of-concept for integrating Zephyr with ODP's Rust services. It builds upon the existing zephyr-lang-rust project, but with a few new drivers that make Zephyr-ODP integration possible.

This project contains two parts: 
- Several new Rust drivers that wrap around Zephyr's generic API (`pwm_fan.rs`, `pwm.rs`, `rtc.rs`, `tachometer.rs`, `temperature_sensor.rs`, `uart.rs`, and `fuel_gaquge.rs`).
- The `zephyr-odp` sample, which sets up `thermal-service`, `uart-service`, `time-alarm-service`, and `battery-service` using the new drivers. This demonstrates how these drivers can plug into ODP services.

The `zephyr-odp` sample targets the NXP MIMXRT685-EVK dev board, but the Rust drivers themselves are completely generic/hardware agnostic (since they just wrap over Zephyr's generic device APIs). The only hardware-specific configuration happens in `mimxrt685_evk_mimxrt685s_cm33.overlay` (a devicetree overlay), which just binds the generic Rust drivers to Zephyr's hardware-specific C drivers. This means that these Rust drivers could be used with any target hardware without needing to change any Rust or C code, provided the target is supported by Zephyr.

Nonetheless, this PoC is still very barebones and many things aren't fully thought through. Please leave feedback!

## Installation
This section provides the information needed to clone and build this project for yourself.

### Pre-reqs
- You should have a MIMXRT685-EVK dev board to flash to.
- You must have Zephyr development environment set up (with the `west` tool). Follow Zephyr's official instructions [here](https://docs.zephyrproject.org/latest/develop/getting_started/index.html).
- You should install Rust, cargo, and probe-rs. If you're developing in WSL, you should also install these things on the Windows side, since flashing from Windows can work better than trying to do it directly from WSL.

### Setting up Environment
To clone the project and set up your environment, run these commands:
```sh
west init -m git@github.com:OpenDevicePartnership/zephyr-lang-rust.git --mr secure-ec-poc --mf ci-manifest.yml my-workspace
cd my-workspace
west update
west zephyr-export
pip install -r zephyr/scripts/requirements.txt
cd modules/lang/rust # this is the main working directory of the project
```

### Building

If you haven't yet, make sure to add the rustup target for this board:
```sh
rustup target add thumbv8m.main-none-eabihf
```

Then, to build the project, run (still in `my-workspace/modules/lang/rust`):
```sh
west build -b mimxrt685_evk/mimxrt685s/cm33 samples/zephyr-odp -d $(west topdir)/build
```

I've also made a helper script for convenience:
```sh
./build.sh
```

## Flashing

There are several ways you can flash the project. The `Flashing from WSL` and `Flashing from Native Linux` sections use probe-rs, since that's what I used. If you don't want to use probe-rs, see the `Flashing without probe-rs` section.

Before trying to flash, connect the dev board to your PC via the J5 and J6 Micro USBs. I have some pictures of what my board setup looked like at the bottom of the README for reference.

### Flashing from WSL

If you're using WSL, make sure you have probe-rs installed in Windows. Then, back in WSL, run (still inside `my-workspace/modules/lang/rust`):
```sh
./flash.sh
```

This script will copy your binary from WSL into Windows, and flash the board from the Windows side via probe-rs.

### Flashing from Native Linux

If you're using native Linux and have probe-rs installed, run:
```sh
./flash.sh
```

Or, if you prefer, you should be able to directly run:
```sh
probe-rs run --chip MIMXRT685SFVKB build/zephyr/zephyr.elf
```

### Flashing without probe-rs

If you don't want to use probe-rs, you should also be able to use the normal `west flash` command, though it might take additional setup.

First, you will probably need to download the LinkServer tool from NXP, since this is what `west flash` seems to default to. If you're on Linux, you may need to add LinkSever to your path after installing it (or else west might not be able to find it).

Once you've done that, running `west flash` should just work if you're on Native Linux (or are developing in Windows).

If you're on WSL, `west flash` might work, but I personally ran into issues with flashing via WSL passthrough. I don't think this was a west-specific issue, but nonetheless, getting probe-rs on Windows and using the `./flash.sh` script will probably be more straightforward.


## Testing

This section provides info on how to set up the dev board for testing this sample. NXP's datasheets forthe i.MX RT685 Eval Board can be found [here](https://www.nxp.com/design/design-center/development-boards-and-designs/i-mx-evaluation-and-development-boards/i-mx-rt600-evaluation-kit:MIMXRT685-EVK).

As of right now, the zephyr-odp sample uses `thermal-service`, `uart-service`, `battery-service`, and `time-alarm-service`. 
- `battery-service` and `time-alarm-service` both use mock Zephyr drivers for RTC and fuel gauge, so no hardware setup is needed there. 
- `uart-service` uses the UART peripheral included directly on the dev board, so it should work automatically when you flash the board. If UART doesn't seem to be working correctly, read the JP4 section on page 10 of the eval board User Guide (you may need to mess with the J16 jumper).
- For `thermal-service`, the sample currently uses an external temperature sensor and an external PWM fan. These two components require external wiring, which is described below.

### Temperature Sensor and Fan Setup

The temperature sensor is a TMP117, connected to the first four pins (10/SCL, 9/SDA, 8/3V3, 7/GND) on J28 (see the picture on page 29 of the eval board User Guide). It should be possible to use another I2C temperature sensor if you want to, but you'd have to edit the `ti_tmp11x` node in the `.overlay`, plus add your nwe sensor's Zephyr compatible to the `temperature-sensor` matcher in `dt-rust.yaml`.

The PWM Fan is just a generic 5V DC Brushless Fan with four wires (5V, GND, PWM Control/Blue, and Tachometer/Yellow). The 5V and GND wires are connected to 5V0/8 and GND/7 respectively on J29 (see page 30 of the eval board User Guide). The PWM Control wire is connected to pin 4/P0_27 on J27 (see User Guide page 29), and the Tachometer wire is connected to pin "IO 0_16" (see User Guide page 21). Any generic PWM fan should be usable here without having to mess with the overlay or `dt-rust.yaml`.

### TUI Setup

If you haven't already, clone and build the ODP ec-test-tui project located [here](https://github.com/OpenDevicePartnership/odp-platform-common/tree/main/ec/test-tui). If you're using the workflow from the `./flash.sh` script (where you build in WSL but flash from Windows), you should build and run ec-test-tui from the Windows side.

This TUI will serve as the MCTP host for the uart-service. If everything is working, the TUI should dispaly something like this once you flash:

<img width="1004" height="567" alt="image" src="https://github.com/user-attachments/assets/3c118772-99eb-4e66-8e88-5e9ad315ffe7" />

Note: As of right now, the mock fuel gauge driver just sends out static battery data, so those numbers won't change.

### Images of Set-up Board

<img width="50%" alt="IMG_5234" src="https://github.com/user-attachments/assets/f7bddfb8-9fc4-4d33-8bfc-54b63e7f71da"/>
<img width="50%" alt="IMG_5235" src="https://github.com/user-attachments/assets/09bfa985-01f8-4e22-91ba-a83a77be8278"/>

## Development

This section contains some notes for exploring the code itself:
- `samples/zephyr-odp` contains the app-layer code that actually sets up and starts the services.
- The new Rust drivers are located in `zephyr/src/device/`.
- There's a `./doc.sh` script that generates the project's Rust docs for you, and opens them in a browser. This is mainly helpful for WSL users.
