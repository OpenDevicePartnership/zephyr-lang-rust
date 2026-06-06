//! Local helpers for the thermal-service sample.

// This is the same as `odp_service_common::spawn_service!` but adapted for embassy-executor version 0.7.0.
// The actual odp_service_common::spawn_service!() macro is meant for embassy-executor 0.10.0, but
// this project is still on 0.7.0 and updating it to 0.10.0 would require also changing the dependency
// versions in the rest of zephyr-lang-rust
macro_rules! spawn_service {
    ($spawner:expr, $service_ty:ty, $init_arg:expr) => {{
        use ::odp_service_common::runnable_service::Service;

        static SERVICE_RESOURCES: ::static_cell::StaticCell<<$service_ty as Service>::Resources> =
            ::static_cell::StaticCell::new();
        let service_resources = SERVICE_RESOURCES
            .init(<<$service_ty as Service>::Resources as Default>::default());

        #[::embassy_executor::task]
        async fn service_task_fn(
            runner: <$service_ty as ::odp_service_common::runnable_service::Service<'static>>::Runner,
        ) {
            use ::odp_service_common::runnable_service::ServiceRunner;
            runner.run().await;
        }

        <$service_ty>::new(service_resources, $init_arg)
            .await
            .map(|(control_handle, runner)| {
                $spawner
                    .spawn(service_task_fn(runner))
                    .expect("Failed to spawn service task");
                control_handle
            })
    }};
}

pub(crate) use spawn_service;
