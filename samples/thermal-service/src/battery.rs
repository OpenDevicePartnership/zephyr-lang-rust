use static_cell::StaticCell;

pub type BatteryService = battery_service::Service<'static, 1>;

const BAT_ID: battery_service::device::DeviceId = battery_service::device::DeviceId(0);

pub async fn init(spawner: embassy_executor::Spawner) -> BatteryService {
    log::info!("Initializing battery service...");

    static BATTERY_DEVICE: StaticCell<battery_service::device::Device> = StaticCell::new();
    let device = BATTERY_DEVICE.init(battery_service::device::Device::new(BAT_ID));
    let driver = zephyr::devicetree::labels::fuel_gauge::get_instance().expect("Failed to call get_instance() for fuel_gauge.");
    let battery = battery_service::wrapper::Wrapper::new(device, driver);

    let service = crate::utils::spawn_service!(spawner, BatteryService, |resources| battery_service::Service::new(
        resources,
        battery_service::InitParams {
            devices: [device],
            config: battery_service::context::Config::default(),
        }
    ))
    .expect("Failed to initialize battery service");

    spawner.spawn(battery_device_controller_task(battery)).expect("Failed to spawn battery device controller task");
    spawner.spawn(update_data_task(service)).expect("Failed to spawn battery update data task");

    log::info!("Initialized battery service!");
    service
}

#[embassy_executor::task]
async fn battery_device_controller_task(battery: battery_service::wrapper::Wrapper<'static, zephyr::device::fuel_gauge::FuelGauge>) {
    battery.process().await;
}

#[embassy_executor::task]
pub async fn update_data_task(service: BatteryService) {

    log::info!("Inside update_data_task()"); // u_Note: REMOVE

    // Initialize the state machine
    if let Err(e) = battery_service::mock::init_state_machine(&service).await {
        log::error!("FG: Failed to init state machine: {:?}. Terminating this task...", e);
        return;
    }

    let mut failures: u32 = 0;
    let mut count: usize = 0;
    loop {
        embassy_time::Timer::after_secs(1).await;
        if count.is_multiple_of(const { 60 * 60 })
            && let Err(e) = service
                .execute_event(battery_service::context::BatteryEvent {
                    event: battery_service::context::BatteryEventInner::PollStaticData,
                    device_id: BAT_ID,
                })
                .await
        {
            failures += 1;
            log::error!("FG: Static data error: {:#?}", e);
        }
        if let Err(e) = service
            .execute_event(battery_service::context::BatteryEvent {
                event: battery_service::context::BatteryEventInner::PollDynamicData,
                device_id: BAT_ID,
            })
            .await
        {
            failures += 1;
            log::error!("FG: Dynamic data error: {:#?}", e);
        }

        if failures > 10 {
            failures = 0;
            count = 0;
            log::error!("FG: Too many errors, timing out and starting recovery...");
            if battery_service::mock::recover_state_machine(&service).await.is_err() {
                log::error!("FG: Failed to recover state machine!");
            }
        }

        count = count.wrapping_add(1);
    }
}