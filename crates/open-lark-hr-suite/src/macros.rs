#[macro_export]
macro_rules! impl_executable_builder {
    ($builder:ty, $service:ty, $request:ty, $response:ty, $method:ident) => {
        #[async_trait::async_trait]
        impl open_lark_core::core::trait_system::executable_builder::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            async fn execute(self, service: &$service) -> open_lark_core::core::SDKResult<$response> {
                service.$method(&self.build(), None).await
            }
        }
    };
}

#[macro_export]
macro_rules! impl_executable_builder_owned {
    ($builder:ty, $service:ty, $request:ty, $response:ty, $method:ident) => {
        #[async_trait::async_trait]
        impl open_lark_core::core::trait_system::executable_builder::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            async fn execute(self, service: &$service) -> open_lark_core::core::SDKResult<$response> {
                service.$method(self.build(), None).await
            }
        }
    };
}

#[macro_export]
macro_rules! impl_executable_builder_config {
    ($builder:ty, $service:ty, $request:ty, $response:ty, $method:ident) => {
        #[async_trait::async_trait]
        impl open_lark_core::core::trait_system::executable_builder::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            async fn execute(self, service: &$service) -> open_lark_core::core::SDKResult<$response> {
                service.$method(self.build(), None).await
            }
        }
    };
}

#[macro_export]
macro_rules! impl_full_service {
    ($service:ty, $name:expr, $version:expr) => {
        impl open_lark_core::core::trait_system::Service for $service {
            fn config(&self) -> &open_lark_core::core::config::Config {
                &self.config
            }

            fn service_name() -> &'static str {
                $name
            }

            fn service_version() -> &'static str {
                $version
            }
        }
    };
}