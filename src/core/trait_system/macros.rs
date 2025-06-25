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
///
/// # Example
/// ```rust
/// impl_executable_builder!(
///     UploadMediaRequestBuilder,
///     MediaService,
///     UploadMediaRequest,
///     BaseResponse<UploadMediaRespData>,
///     upload_all
/// );
/// ```
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
        impl $crate::core::trait_system::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            fn build(self) -> $request {
                self.build()
            }

            async fn execute(self, service: &$service) -> $crate::core::SDKResult<$response> {
                service.$method(&self.build(), None).await
            }

            async fn execute_with_options(
                self,
                service: &$service,
                option: $crate::core::req_option::RequestOption,
            ) -> $crate::core::SDKResult<$response> {
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
        impl $crate::core::trait_system::ExecutableBuilder<$service, $request, $response>
            for $builder
        {
            fn build(self) -> $request {
                self.build()
            }

            async fn execute(self, service: &$service) -> $crate::core::SDKResult<$response> {
                service.$method(self.build(), None).await
            }

            async fn execute_with_options(
                self,
                service: &$service,
                option: $crate::core::req_option::RequestOption,
            ) -> $crate::core::SDKResult<$response> {
                service.$method(self.build(), Some(option)).await
            }
        }
    };
}

/// 为直接使用Config参数的独立函数实现ExecutableBuilder trait
///
/// 这个宏用于那些不通过服务而是直接调用独立函数的Builder类型
#[macro_export]
macro_rules! impl_executable_builder_config {
    (
        $builder:ty,
        $request:ty,
        $response:ty,
        $function:ident
    ) => {
        impl $builder {
            /// 执行请求
            pub async fn execute(
                self,
                config: &$crate::core::config::Config,
            ) -> $crate::core::SDKResult<$response> {
                $function(self.build(), config, None).await
            }

            /// 执行请求（带选项）
            pub async fn execute_with_options(
                self,
                config: &$crate::core::config::Config,
                option: $crate::core::req_option::RequestOption,
            ) -> $crate::core::SDKResult<$response> {
                $function(self.build(), config, Some(option)).await
            }
        }
    };
}
