/// 核心宏：为Builder类型自动实现ExecutableBuilder trait
///
/// 这个宏消除了手动实现重复execute方法的需要，
/// 通过声明式配置自动生成trait实现。
///
/// # 参数
/// - `$builder`: Builder类型名称
/// - `$service`: 服务类型名称
/// - `$request`: 请求类型名称
/// - `$response`: 响应类型名称
/// - `$method`: 服务方法名称
///
/// # 生成的代码
/// 为指定的Builder类型实现ExecutableBuilder trait，包括：
/// - `execute()` 方法：调用 `service.$method(self.build(), None)`
/// - `execute_with_options()` 方法：调用 `service.$method(self.build(), Some(option))`
#[macro_export]
macro_rules! impl_executable_builder {
    (
        $builder:ty,
        $service:ty,
        $request:ty,
        $response:ty,
        $method:ident
    ) => {
        #[async_trait::async_trait]
        impl open_lark_core::core::trait_system::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            fn build(self) -> $request {
                self.build()
            }

            async fn execute(self, service: &$service) -> open_lark_core::core::SDKResult<$response> {
                service.$method(&self.build(), None).await
            }

            async fn execute_with_options(
                self,
                service: &$service,
                option: open_lark_core::core::req_option::RequestOption,
            ) -> open_lark_core::core::SDKResult<$response> {
                service.$method(&self.build(), Some(option)).await
            }
        }
    };
}

/// 为使用值类型参数的Builder实现ExecutableBuilder trait
///
/// 与主宏的差异：服务方法接受值类型而不是引用类型的Request
#[macro_export]
macro_rules! impl_executable_builder_owned {
    (
        $builder:ty,
        $service:ty,
        $request:ty,
        $response:ty,
        $method:ident
    ) => {
        #[async_trait::async_trait]
        impl open_lark_core::core::trait_system::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            fn build(self) -> $request {
                self.build()
            }

            async fn execute(self, service: &$service) -> open_lark_core::core::SDKResult<$response> {
                service.$method(self.build(), None).await
            }

            async fn execute_with_options(
                self,
                service: &$service,
                option: open_lark_core::core::req_option::RequestOption,
            ) -> open_lark_core::core::SDKResult<$response> {
                service.$method(self.build(), Some(option)).await
            }
        }
    };
}

/// 一次性实现所有基础服务 traits 的便利宏
#[macro_export]
macro_rules! impl_full_service {
    ($service_type:ty, $name:expr) => {
        impl_full_service!($service_type, $name, "v1");
    };
    ($service_type:ty, $name:expr, $version:expr) => {
        impl_basic_service!($service_type, $name, $version);
        impl_service_health_check!($service_type);
        impl_configurable_service!($service_type);
    };
}

/// 为基础服务生成标准实现的宏
#[macro_export]
macro_rules! impl_basic_service {
    ($service_type:ty, $name:expr, $version:expr) => {
        impl open_lark_core::core::trait_system::Service for $service_type {
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

        impl open_lark_core::core::trait_system::ServiceObservability for $service_type {}

        impl open_lark_core::core::trait_system::ServiceBuilder<$service_type> for $service_type {
            fn build(config: open_lark_core::core::config::Config) -> $service_type {
                Self { config }
            }
        }
    };
}

/// 为服务生成健康检查实现的宏
#[macro_export]
macro_rules! impl_service_health_check {
    ($service_type:ty) => {
        impl open_lark_core::core::trait_system::ServiceHealthCheck for $service_type {
            async fn health_check(
                &self,
            ) -> open_lark_core::core::SDKResult<open_lark_core::core::trait_system::ServiceHealthStatus> {
                use open_lark_core::core::trait_system::ServiceHealthStatus;

                if !self.is_config_valid() {
                    return Ok(ServiceHealthStatus::Unhealthy(
                        "Invalid configuration".to_string(),
                    ));
                }

                // 基础健康检查 - 可以在具体服务中重写
                Ok(ServiceHealthStatus::Healthy)
            }
        }
    };
}

/// 为服务生成可配置实现的宏
#[macro_export]
macro_rules! impl_configurable_service {
    ($service_type:ty) => {
        impl open_lark_core::core::trait_system::ConfigurableService for $service_type {
            fn update_config(
                &mut self,
                new_config: open_lark_core::core::config::Config,
            ) -> open_lark_core::core::SDKResult<()> {
                self.validate_config(&new_config)?;
                self.config = new_config;
                Ok(())
            }
        }
    };
}