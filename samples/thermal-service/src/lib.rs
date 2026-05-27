// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::ffi::c_int;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_sensors_hal_async::sensor as sensor_embedded;
use embedded_sensors_hal_async::temperature::{
    DegreesCelsius, TemperatureSensor, TemperatureThresholdSet,
};
use log::info;
use odp_service_common::runnable_service::{Service as RunnableService, ServiceRunner};
use static_cell::StaticCell;
use thermal_service as ts;
use thermal_service_interface::sensor as ts_sensor_interface;
use thermal_service_interface::sensor::SensorService;
use zephyr::device::temperature_sensor::TemperatureSensor as ZephyrTemperatureSensor;
use zephyr::embassy::Executor;
use zephyr::raw;

// The main thread priority.
const MAIN_PRIO: c_int = 2;

static EXECUTOR_MAIN: StaticCell<Executor> = StaticCell::new();

#[unsafe(no_mangle)]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    info!("ODP thermal-services on Zephyr");

    // Set our own thread priority.
    unsafe {
        raw::k_thread_priority_set(raw::k_current_get(), MAIN_PRIO);
    }

    info!("Starting Embassy executor");
    let executor = EXECUTOR_MAIN.init(Executor::new());
    info!("----executor object created");

    executor.run(|spawner| {
        spawner.must_spawn(main_task(spawner));
    })
}

#[embassy_executor::task]
async fn main_task(spawner: Spawner) {
    info!(" Start embedded service");
    embedded_services::init().await;
    info!("----Embedded service initialized");

    init_thermal_service(spawner).await;
}

// Event sender that does nothing (placeholder for real event handling)
struct NoOpEventSender;

impl embedded_services::event::Sender<ts_sensor_interface::Event> for NoOpEventSender {
    async fn send(&mut self, _event: ts_sensor_interface::Event) {
        // No-op for this simple demo
    }

    fn try_send(&mut self, _event: ts_sensor_interface::Event) -> Option<()> {
        Some(())
    }
}

async fn init_thermal_service(spawner: Spawner) {
    info!(" Start thermal service");

    let tmp11x_driver = Tmp11xSensor::new();
    info!("----Sensor driver created");

    // Configuration for the sensor service
    let config = ts::sensor::Config {
        sample_period: Duration::from_secs(2),
        fast_sample_period: Duration::from_secs(2),
        ..Default::default()
    };

    // Static storage for the sensor service resources
    static SENSOR_RESOURCES: StaticCell<ts::sensor::Resources<Tmp11xSensor, 16>> = StaticCell::new();
    let sensor_resources = SENSOR_RESOURCES.init(ts::sensor::Resources::default());

    // Static storage for event senders
    static EVENT_SENDERS: StaticCell<[NoOpEventSender; 1]> = StaticCell::new();
    let event_senders = EVENT_SENDERS.init([NoOpEventSender]);

    let init_params = ts::sensor::InitParams {
        driver: tmp11x_driver,
        config,
        event_senders: event_senders.as_mut_slice(),
    };

    // Initialize sensor service
    let (sensor_service, sensor_runner) = ts::sensor::Service::<Tmp11xSensor, NoOpEventSender, 16>::new(
        sensor_resources,
        init_params,
    )
    .await
    .expect("Failed to initialize sensor service");

    info!("----Sensor service initialized");

    // Spawn the sensor runner task
    spawner.must_spawn(sensor_runner_task(sensor_runner));
    info!("----Sensor task spawned");

    spawner.spawn(heartbeat()).expect("ERROR: Failed to spawn task 'hearbeat()'.");

    // Use the sensor service to read temperature
    let temp = sensor_service.temperature().await;
    info!("----Initial temperature: {} C", temp);
}

#[embassy_executor::task]
async fn sensor_runner_task(
    runner: ts::sensor::Runner<'static, Tmp11xSensor, NoOpEventSender, 16>,
) {
    runner.run().await;
}

#[embassy_executor::task]
async fn heartbeat() {
    loop {
        info!("heartbeat");
        Timer::after_secs(1).await;
    }
}

// Tmp11xSensor, this is a wrapper around the Zephyr tmp11x temperature sensor
// and implements the embedded_sensors_hal_async::temperature::TemperatureSensor
// trait.
#[derive(Copy, Clone, Debug)]
pub struct Tmp11xSensorError;
impl sensor_embedded::Error for Tmp11xSensorError {
    fn kind(&self) -> sensor_embedded::ErrorKind {
        sensor_embedded::ErrorKind::Other
    }
}

pub struct Tmp11xSensor {
    sensor: ZephyrTemperatureSensor,
}

impl Tmp11xSensor {
    fn new() -> Self {
        let sensor = zephyr::devicetree::aliases::temperature_sensor::get_instance().unwrap();

        Self { sensor }
    }
}

impl sensor_embedded::ErrorType for Tmp11xSensor {
    type Error = Tmp11xSensorError;
}

impl TemperatureSensor for Tmp11xSensor {
    async fn temperature(&mut self) -> Result<DegreesCelsius, Self::Error> {
        match self.sensor.read_ambient_temperature() {
            Ok(temperature) => {
                info!("Temperature read out success");
                info!("    {}.{} Celsius", temperature.val1, temperature.val2);
                let temperature: f32 =
                    temperature.val1 as f32 + (temperature.val2 as f32) / 1_000_000.0;
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

impl TemperatureThresholdSet for Tmp11xSensor {
    async fn set_temperature_threshold_low(
        &mut self,
        _threshold: DegreesCelsius,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn set_temperature_threshold_high(
        &mut self,
        _threshold: DegreesCelsius,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Implement the Driver trait (marker trait requiring TemperatureSensor)
impl ts_sensor_interface::Driver for Tmp11xSensor {}
