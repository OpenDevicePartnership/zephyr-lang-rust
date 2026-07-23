use static_cell::StaticCell;
use embassy_sync::mutex::Mutex;
use embedded_services::GlobalRawMutex;
use battery_service::FuelGauge as _;

/// Our `BatteryServiceWrapper` driver but wrapped in a mutex so we can provide it to battery-service
type WrappedFuelGauge = Mutex<GlobalRawMutex, BatteryServiceWrapper>;

/// Registration for our fuel gauge
type Reg = battery_service::ArrayRegistration<'static, WrappedFuelGauge, 1>;

pub type BatteryService = battery_service::Service<'static, Reg>;

pub async fn init(spawner: embassy_executor::Spawner) -> BatteryService {
    log::info!("Initializing battery service...");

    let driver = zephyr::devicetree::labels::fuel_gauge::get_instance().expect("Failed to call get_instance() for fuel_gauge.");

    static FUEL_GAUGE: StaticCell<WrappedFuelGauge> = StaticCell::new();
    let fuel_gauge: &'static WrappedFuelGauge =
        FUEL_GAUGE.init(Mutex::new(BatteryServiceWrapper::from(driver)));

    let service = battery_service::Service::new(battery_service::ArrayRegistration {
        fuel_gauges: [fuel_gauge],
    });

    spawner
        .spawn(update_data_task(fuel_gauge))
        .expect("Failed to spawn battery update data task");

    log::info!("Initialized battery service!");
    service
}

#[embassy_executor::task]
pub async fn update_data_task(fuel_gauge: &'static WrappedFuelGauge) {
    if let Err(e) = battery_service::mock::init_state_machine(fuel_gauge).await {
        log::error!("FG: Failed to init state machine: {:?}. Terminating this task...", e);
        return;
    }

    let mut failures: u32 = 0;
    let mut count: usize = 0;
    loop {
        embassy_time::Timer::after_secs(1).await;

        if count.is_multiple_of(const { 60 * 60 })
            && let Err(e) = fuel_gauge.lock().await.update_static_data().await
        {
            failures += 1;
            log::error!("FG: Static data error: {:?}", e);
        }

        if let Err(e) = fuel_gauge.lock().await.update_dynamic_data().await {
            failures += 1;
            log::error!("FG: Dynamic data error: {:?}", e);
        }

        if failures > 10 {
            failures = 0;
            count = 0;
            log::error!("FG: Too many errors, timing out and starting recovery...");
            if battery_service::mock::recover_state_machine(fuel_gauge).await.is_err() {
                log::error!("FG: Failed to recover state machine!");
            }
        }

        count = count.wrapping_add(1);
    }
}

/// Wrapper around the Zephyr driver.
pub struct BatteryServiceWrapper {
    pub driver: zephyr::device::fuel_gauge::FuelGauge,
    pub state: battery_service::State,
}

/// Map Zephyr error to wrapper error.
#[derive(Debug)]
pub enum BatteryServiceWrapperError { ZephyrError(zephyr::error::Error) }
impl From<zephyr::error::Error> for BatteryServiceWrapperError {
    fn from(err: zephyr::error::Error) -> Self {
        BatteryServiceWrapperError::ZephyrError(err)
    }
}

/// Map a BatteryServiceWrapperError to a smart_battery::ErrorKind
impl embedded_batteries_async::smart_battery::Error for BatteryServiceWrapperError {
    fn kind(&self) -> embedded_batteries_async::smart_battery::ErrorKind {
        match self {
            BatteryServiceWrapperError::ZephyrError(e) => {
                embedded_batteries_async::smart_battery::Error::kind(e)
            }
        }
    }
}

/// Allow you to convert a normal `FuelGauge` into a `BatteryServiceWrapper` to be used with the battery service.
impl From<zephyr::device::fuel_gauge::FuelGauge> for BatteryServiceWrapper {
    fn from(value: zephyr::device::fuel_gauge::FuelGauge) -> Self {
        Self {
            driver: value,
            state: battery_service::State::default(),
        }
    }
}

/// Convert local errors into battery-service errors.
impl From<BatteryServiceWrapperError> for battery_service::FuelGaugeError {
    fn from(_error: BatteryServiceWrapperError) -> Self {
        battery_service::FuelGaugeError::BusError
    }
}

// Implement SmartBattery for Battery Service.
embedded_batteries_async::impl_smart_battery_for_wrapper_type!(BatteryServiceWrapper, driver, BatteryServiceWrapperError);

impl battery_service::FuelGauge for BatteryServiceWrapper {
    type FuelGaugeError = BatteryServiceWrapperError;
    type StaticData = battery_service::StaticBatteryMsgs;
    type DynamicData = battery_service::DynamicBatteryMsgs;

    async fn initialize(&mut self) -> Result<(), Self::FuelGaugeError> {
        self.driver.set_capacity_mode(zephyr::device::fuel_gauge::CapacityMode::MilliAmp).await?;
        self.state.on_initialized();
        Ok(())
    }

    async fn ping(&mut self) -> Result<(), Self::FuelGaugeError> {
        use embedded_batteries_async::smart_battery::SmartBattery;
        SmartBattery::charging_voltage(self).await.inspect_err(|e| log::error!("Failed to ping fuel gauge: Call to self.charging_voltage() failed with e: {:?}", e))?;
        log::info!("Successfully pinged fuel gauge.");
        self.state.on_recovered();
        Ok(())
    }

    async fn update_dynamic_data(&mut self) -> Result<(), Self::FuelGaugeError> {
        use embedded_batteries_async::smart_battery::SmartBattery;

        let average_current = SmartBattery::average_current(self).await?;
        let battery_status = SmartBattery::battery_status(self).await?.into_bits();
        let battery_temp = SmartBattery::temperature(self).await?;
        let charging_current = SmartBattery::charging_current(self).await?;
        let charging_voltage = SmartBattery::charging_voltage(self).await?;
        let voltage = SmartBattery::voltage(self).await?;
        let current = SmartBattery::current(self).await?;
        let full_charge_capacity = SmartBattery::full_charge_capacity(self).await?;
        let remaining_capacity = SmartBattery::remaining_capacity(self).await?;
        let relative_soc = SmartBattery::relative_state_of_charge(self).await?;
        let cycle_count = SmartBattery::cycle_count(self).await?;
        let max_error = SmartBattery::max_error(self).await.unwrap_or(0); // u_TODO: Hardcoding to zero since the Zephyr API doesn't expose max error yet

        self.state.on_dynamic_data(|d| {
            d.average_current = average_current;
            d.battery_status = battery_status;
            d.max_power_mw = 0;
            d.battery_temp = battery_temp;
            d.sus_power_mw = 0;
            d.charging_current = charging_current;
            d.charging_voltage = charging_voltage;
            d.voltage = voltage;
            d.current = current;
            d.full_charge_capacity = full_charge_capacity;
            d.remaining_capacity = remaining_capacity;
            d.relative_soc = relative_soc;
            d.cycle_count = cycle_count;
            d.max_error = max_error;
            d.bmd_status = battery_service_interface::BmdStatusFlags::default();
            d.turbo_vload = 0;
            d.turbo_rhf_effective_mohm = 0;
        });
        Ok(())
    }

    async fn update_static_data(&mut self) -> Result<(), Self::FuelGaugeError> {
        use embedded_batteries_async::smart_battery::SmartBattery;
        use embedded_batteries_async::smart_battery::CapacityModeValue;
        use battery_service_interface::fuel_gauge::{DEVICE_CHEMISTRY_SIZE, DEVICE_NAME_SIZE, MANUFACTURER_NAME_SIZE};

        let design_voltage = SmartBattery::design_voltage(self).await?;
        let design_capacity = SmartBattery::design_capacity(self).await?;
        let design_capacity_value: u16 = match design_capacity {
            CapacityModeValue::CentiWattUnsigned(v) => v,
            CapacityModeValue::MilliAmpUnsigned(v) => v,
        };
        let battery_mode = SmartBattery::battery_mode(self).await?;
        let measurement_accuracy: u32 = SmartBattery::max_error(self).await.unwrap_or(0).into(); // u_TODO: Hardcoding to zero since the Zephyr API doesn't expose max error yet

        let mut manufacturer_name = [0u8; MANUFACTURER_NAME_SIZE];
        SmartBattery::manufacturer_name(self, &mut manufacturer_name).await?;
        let mut device_name = [0u8; DEVICE_NAME_SIZE];
        SmartBattery::device_name(self, &mut device_name).await?;
        let mut device_chemistry = [0u8; DEVICE_CHEMISTRY_SIZE];
        SmartBattery::device_chemistry(self, &mut device_chemistry).await?;

        self.state.on_static_data(|s| {
            s.manufacturer_name = manufacturer_name;
            s.device_name = device_name;
            s.device_chemistry = device_chemistry;
            s.design_voltage = design_voltage;
            s.design_capacity = design_capacity;
            s.battery_mode = battery_mode;
            s.design_cap_warning = CapacityModeValue::MilliAmpUnsigned(design_capacity_value / 4);
            s.design_cap_low = CapacityModeValue::MilliAmpUnsigned(design_capacity_value / 10);
            s.measurement_accuracy = measurement_accuracy;
            s.power_threshold_support = battery_service_interface::PowerThresholdSupport::empty();
            s.bmc_flags = battery_service_interface::BmcControlFlags::empty();
            s.bmd_capability = battery_service_interface::BmdCapabilityFlags::empty();
        });
        Ok(())
    }

    fn state(&self) -> &battery_service::State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut battery_service::State {
        &mut self.state
    }
}