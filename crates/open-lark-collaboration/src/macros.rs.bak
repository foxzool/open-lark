/// 核心宏：为Builder类型自动实现ExecutableBuilder trait
///
/// 这个宏消除了手动实现重复execute方法的需要，
/// 通过声明式配置自动生成trait实现。
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