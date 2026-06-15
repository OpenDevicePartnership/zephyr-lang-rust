
pub type TimeAlarmService = time_alarm_service::Service<'static>;

#[allow(clippy::needless_return)]
pub async fn init(spawner: embassy_executor::Spawner) -> TimeAlarmService {
    use static_cell::StaticCell;

    static TZ_STORAGE: StaticCell<time_alarm_service::mock::MockNvramStorage<'static>> = StaticCell::new();
    let tz_storage = TZ_STORAGE.init(time_alarm_service::mock::MockNvramStorage::new(0));

    static AC_EXP_STORAGE: StaticCell<time_alarm_service::mock::MockNvramStorage<'static>> = StaticCell::new();
    let ac_exp_storage = AC_EXP_STORAGE.init(time_alarm_service::mock::MockNvramStorage::new(0));

    static AC_POL_STORAGE: StaticCell<time_alarm_service::mock::MockNvramStorage<'static>> = StaticCell::new();
    let ac_pol_storage = AC_POL_STORAGE.init(time_alarm_service::mock::MockNvramStorage::new(0));

    static DC_EXP_STORAGE: StaticCell<time_alarm_service::mock::MockNvramStorage<'static>> = StaticCell::new();
    let dc_exp_storage = DC_EXP_STORAGE.init(time_alarm_service::mock::MockNvramStorage::new(0));

    static DC_POL_STORAGE: StaticCell<time_alarm_service::mock::MockNvramStorage<'static>> = StaticCell::new();
    let dc_pol_storage = DC_POL_STORAGE.init(time_alarm_service::mock::MockNvramStorage::new(0));

    static RTC_DRIVER: StaticCell<zephyr::device::rtc::Rtc> = StaticCell::new();
    let rtc_driver = RTC_DRIVER.init(zephyr::devicetree::rs_rtc::get_instance().expect("Failed to call zephyr::devicetree::rs_rtc::get_instance()"));

    log::info!("init time-alarm service"); // u_Note: REMOVE

    crate::utils::spawn_service!(spawner, TimeAlarmService, |resources| {
        time_alarm_service::Service::new(
            resources,
            rtc_driver,
            tz_storage,
            ac_exp_storage,
            ac_pol_storage,
            dc_exp_storage,
            dc_pol_storage,
        )
    })
    .expect("Failed to spawn time alarm service!")
}
