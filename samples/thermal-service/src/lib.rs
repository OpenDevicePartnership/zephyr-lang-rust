// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::ffi::c_int;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use log::info;
use static_cell::StaticCell;
use odp_service_common::runnable_service::{Service, ServiceRunner};

// Entry point into the Rust program from Zephyr.
#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    const MAIN_PRIO: c_int = 2;

    unsafe {
        zephyr::set_logger().unwrap();
        zephyr::raw::k_thread_priority_set(zephyr::raw::k_current_get(), MAIN_PRIO);
    }

    // Spawn the main task. u_Note: We could technically just spawn all the tasks in rust_main() directly and completely get rid of main_task(), but it would be weird having to put `embedded_services::init().await;` in an actual task since it feels like it "belongs" to all tasks (i.e., it wouldn't make sense to tie it to a specific task when it is relavent to multiple tasks).
    static EXECUTOR_MAIN: StaticCell<zephyr::embassy::Executor> = StaticCell::new();
    let executor = EXECUTOR_MAIN.init(zephyr::embassy::Executor::new());
    executor.run(|spawner| {
        spawner.spawn(main_task(spawner)).expect("Failed to spawn main_task");
    })
}

// Main embassy task to spawn all the child services.
#[embassy_executor::task]
async fn main_task(spawner: Spawner) {
    // Initialize embedded_services.
    embedded_services::init().await;
    info!("Embedded services initialized");

    // Spawn all the different tasks.
    spawner.spawn(heartbeat()).expect("Failed to spawn heartbeat()");
    spawner.spawn(thermal_service()).expect("Failed to spawn thermal_service()");
}

// Sends "heartbeat" every second.
#[embassy_executor::task]
async fn heartbeat() {
    loop {
        info!("heartbeat");
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn thermal_service() {

    // Sensor event handler
    struct SensorEventHandler;
    static SENSOR_EVENT_SENDERS: StaticCell<[SensorEventHandler; 1]> = StaticCell::new();
    let sensor_event_senders = SENSOR_EVENT_SENDERS.init([SensorEventHandler]);
    impl embedded_services::event::Sender<thermal_service_interface::sensor::Event> for SensorEventHandler {
        async fn send(&mut self, event: thermal_service_interface::sensor::Event) {
            info!("Thermal event: {:?}", event);
        }
        fn try_send(&mut self, _event: thermal_service_interface::sensor::Event) -> Option<()> {
            Some(())
        }
    }

    // Static storage for the sensor service resources
    static SENSOR_RESOURCES: StaticCell<thermal_service::sensor::Resources<tmp11x::Sensor, 16>> = StaticCell::new();
    let sensor_resources = SENSOR_RESOURCES.init(thermal_service::sensor::Resources::default());

    // Initialize sensor runner.
    let (sensor_service, sensor_runner) = thermal_service::sensor::Service::<tmp11x::Sensor, SensorEventHandler, 16>::new(
        sensor_resources,                                   // Resources used by the temperature sensor

        // Thermal service init params.
        thermal_service::sensor::InitParams {
            driver: tmp11x::Sensor::new(),                         // The TMP11x temperature sensor driver
            event_senders: sensor_event_senders.as_mut_slice(),    // List of event senders

            // Thermal service sensor config
            config: thermal_service::sensor::Config {
                sample_period: Duration::from_secs(2),      // Rate at which to sample the sensor when operating in normal conditions
                fast_sample_period: Duration::from_secs(2), // Rate at which to sample the sensor when operating in fast conditions
                ..Default::default()
            },
        },
    )
    .await
    .expect("ERROR: Failed to initialize sensor service");

    // Fan event handler
    struct FanEventHandler;
    static FAN_EVENT_SENDERS: StaticCell<[FanEventHandler; 1]> = StaticCell::new();
    let fan_event_senders = FAN_EVENT_SENDERS.init([FanEventHandler]);
    impl embedded_services::event::Sender<thermal_service_interface::fan::Event> for FanEventHandler {
        async fn send(&mut self, event: thermal_service_interface::fan::Event) {
            info!("Thermal event: {:?}", event);
        }
        fn try_send(&mut self, _event: thermal_service_interface::fan::Event) -> Option<()> {
            Some(())
        }
    }

    // Static storage for the fan runner resources
    static FAN_RESOURCES: StaticCell<thermal_service::fan::Resources<zephyr::device::pwm_fan::PwmFan, 16>> = StaticCell::new();
    let fan_resources = FAN_RESOURCES.init(thermal_service::fan::Resources::default());

    // Initialize fan runner, using sensor_service.
    let (_fan_service, fan_runner) = thermal_service::fan::Service::<
        zephyr::device::pwm_fan::PwmFan,
        thermal_service::sensor::Service<tmp11x::Sensor, SensorEventHandler, 16>,
        FanEventHandler,
        16
    >::new(
        fan_resources,

        // Fan service init params.
        thermal_service::fan::InitParams {
            driver: zephyr::devicetree::labels::fan0::get_instance().unwrap(),
            event_senders: fan_event_senders.as_mut_slice(),
            sensor_service,

            // Thermal service fan config
            config: thermal_service::fan::Config {
                ..Default::default()
            },
        },
    )
    .await
    .expect("ERROR: Failed to initialize fan service.");

    // Start the services
    info!("Starting thermal_service() runners.");
    embassy_futures::join::join(
        sensor_runner.run(),
        fan_runner.run(),
    ).await;
}

mod tmp11x {
    pub use embedded_sensors_hal_async::temperature::DegreesCelsius;
    use log::info;

    // Wrapper around the Zephyr TMP11x temperature sensor with implementations for the generic embedded_sensors_hal_async::temperature::TemperatureSensor trait.
    pub struct Sensor(zephyr::device::temperature_sensor::TemperatureSensor);
    impl Sensor {
        pub fn new() -> Self {
            Self(zephyr::devicetree::labels::ti_tmp11x::get_instance().unwrap())
        }
    }
    impl thermal_service_interface::sensor::Driver for Sensor {} // Marker trait so Sensor can be used with thermal_service.

    // Error type.
    impl embedded_sensors_hal_async::sensor::ErrorType for Sensor {
        type Error = embedded_sensors_hal_async::sensor::ErrorKind;
    }

    // Returns a temperature sample in degrees Celsius.
    impl embedded_sensors_hal_async::temperature::TemperatureSensor for Sensor {
        async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
            let Self(sensor) = self;
            match sensor.read_ambient_temperature() {
                Ok(temperature) => {
                    info!("Temperature read out success");
                    info!("    {}.{} Celsius", temperature.val1, temperature.val2);
                    let temperature: f32 = temperature.val1 as f32 + (temperature.val2 as f32) / 1_000_000.0;
                    Ok(temperature)
                }
                Err(e) => {
                    info!("Temperature read out failed {}", e);
                    let temperature: DegreesCelsius = 42.0;
                    Ok(temperature)
                }
            }
        }
    }
}