// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use core::ffi::c_int;
use embassy_executor::Spawner;
use embassy_sync::once_lock::OnceLock;
use embedded_sensors_hal_async::sensor as sensor_embedded;
use embedded_sensors_hal_async::temperature::{
    DegreesCelsius, TemperatureSensor, TemperatureThresholdSet,
};
use log::info;
use static_cell::StaticCell;
use zephyr::device::temperature_sensor::TemperatureSensor as ZephyrTemperatureSensor;
use zephyr::embassy::Executor;
use zephyr::raw;

use thermal_service as ts;

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

async fn init_thermal_service(spawner: Spawner) {
    info!(" Start thermal service");
    ts::init().await.unwrap();
    info!("----ODP initialize thermal service");

    let sensor_dev = MockTempSensor::new();
    static SENSOR: OnceLock<ts::sensor::Sensor<MockTempSensor, 16>> = OnceLock::new();
    info!("----Sensor object allocated");

    // The sample period is in milliseconds however it does not match the Timer::after_millis implementation in
    // Zephyr. The value 20 is equivalent to 2000ms in Zephyr.
    // TODO: Investigate the discrepancy between the two values.
    let profile = ts::sensor::Profile {
        sample_period: 20,
        fast_sample_period: 20,
        ..Default::default()
    };
    let sensor = SENSOR
        .get_or_init(|| ts::sensor::Sensor::new(ts::sensor::DeviceId(0), sensor_dev, profile));
    info!("----Sensor initialized");

    ts::register_sensor(sensor.device()).await.unwrap();
    info!("----Sensor registered");

    spawner.must_spawn(mock_temp_sensor_task(sensor));
    info!("----Sensor task spawned");
}

ts::impl_sensor_task!(mock_temp_sensor_task, MockTempSensor, 16);

// MockTempSensor wraps the Zephyr temperature sensor device and implements
// the embedded_sensors_hal_async::temperature::TemperatureSensor trait.
#[derive(Copy, Clone, Debug)]
pub struct MockTempSensorError;
impl sensor_embedded::Error for MockTempSensorError {
    fn kind(&self) -> sensor_embedded::ErrorKind {
        sensor_embedded::ErrorKind::Other
    }
}

pub struct MockTempSensor {
    sensor: ZephyrTemperatureSensor,
}

impl MockTempSensor {
    fn new() -> Self {
        let sensor = zephyr::devicetree::aliases::temperature_sensor::get_instance().unwrap();

        Self { sensor }
    }
}

impl sensor_embedded::ErrorType for MockTempSensor {
    type Error = MockTempSensorError;
}

impl TemperatureSensor for MockTempSensor {
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

impl TemperatureThresholdSet for MockTempSensor {
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

impl ts::sensor::CustomRequestHandler for MockTempSensor {}
impl ts::sensor::Controller for MockTempSensor {}
