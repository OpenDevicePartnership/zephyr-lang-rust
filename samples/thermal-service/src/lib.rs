// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::ffi::c_int;
use embassy_executor::Spawner;
use embassy_time::Timer;
use log::info;
use static_cell::StaticCell;

mod thermal;
mod utils;

// Entry point into the Rust program from Zephyr.
#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    const MAIN_PRIO: c_int = 2;

    // SAFETY: `rust_main` runs once during application startup before any rust tasks
    // are spawned, so the global logger is initialized before concurrent use.
    unsafe {
        zephyr::set_logger().unwrap();
    }

    // Set our own priority.
    // SAFETY: `rust_main` runs in a thread context.
    unsafe {
        zephyr::raw::k_thread_priority_set(zephyr::raw::k_current_get(), MAIN_PRIO);
    }

    // Spawn the main task. u_Note: We could technically just spawn all the tasks in rust_main() directly and completely get rid of main_task(), but it would be weird having to put `embedded_services::init().await;` in an actual task since it feels like it "belongs" to all tasks (i.e., it wouldn't make sense to tie it to a specific task when it is relavent to multiple tasks).
    static EXECUTOR_MAIN: StaticCell<zephyr::embassy::Executor> = StaticCell::new();
    let executor = EXECUTOR_MAIN.init(zephyr::embassy::Executor::new());
    executor.run(|spawner| {
        spawner.spawn(init(spawner)).expect("Failed to spawn main_task");
    })
}

// Main embassy task to spawn all the child services.
#[embassy_executor::task]
async fn init(spawner: Spawner) {
    // Initialize embedded_services.
    embedded_services::init().await;
    info!("Embedded services initialized");
    
    // Spawn all the different tasks.
    spawner.spawn(uart_service(spawner)).expect("Failed to spawn uart_service()");

    // HEARTBEAT
    loop {
        info!("heartbeat");
        Timer::after_secs(1).await;
    }
}

// UART service. Spawns out the thermal, battery, and timer services.
#[embassy_executor::task]
async fn uart_service(spawner: Spawner) {
    // Define RelayHandler for UART Service
    embedded_services::relay::mctp::impl_odp_mctp_relay_handler!(
        RelayHandler;
        Thermal, 0x09, thermal_service_relay::ThermalServiceRelayHandler<crate::thermal::ThermalService>;
    );

    // Initialize services
    let thermal = crate::thermal::init(spawner).await;
    // gonna put more here eventually

    // Create relay handler for the above services
    let relay = RelayHandler::new(
        thermal_service_relay::ThermalServiceRelayHandler::new(thermal),
    );

    static UART_SERVICE: StaticCell<uart_service::DefaultService<RelayHandler>> = StaticCell::new();
    let uart_service = UART_SERVICE.init(uart_service::DefaultService::default_smbusespi(relay).unwrap());
    let uart_driver: zephyr::device::uart::Uart = zephyr::devicetree::labels::arduino_serial::get_instance().unwrap();
    let Err(e) = uart_service::task::uart_service(uart_service, uart_driver).await;
    log::error!("uart_service() encountered an error (Error: {:?})", e);
}