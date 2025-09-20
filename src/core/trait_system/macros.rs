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
/// ```rust,ignore
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

#[cfg(test)]
mod tests {
    use crate::core::{
        api_resp::{ApiResponseTrait, BaseResponse, RawResponse, ResponseFormat},
        config::Config,
        req_option::RequestOption,
        trait_system::ExecutableBuilder,
        SDKResult,
    };
    use serde::{Deserialize, Serialize};

    // Test types for macro validation
    #[derive(Debug, Clone)]
    struct MockRequest {
        data: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct MockResponse {
        result: String,
    }

    impl ApiResponseTrait for MockResponse {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    #[derive(Clone)]
    struct MockService;

    impl MockService {
        async fn test_method(
            &self,
            request: &MockRequest,
            _option: Option<RequestOption>,
        ) -> SDKResult<BaseResponse<MockResponse>> {
            Ok(BaseResponse {
                raw_response: RawResponse {
                    code: 0,
                    msg: "success".to_string(),
                    err: None,
                },
                data: Some(MockResponse {
                    result: format!("processed: {}", request.data),
                }),
            })
        }

        async fn test_method_owned(
            &self,
            request: MockRequest,
            _option: Option<RequestOption>,
        ) -> SDKResult<BaseResponse<MockResponse>> {
            Ok(BaseResponse {
                raw_response: RawResponse {
                    code: 0,
                    msg: "success".to_string(),
                    err: None,
                },
                data: Some(MockResponse {
                    result: format!("owned: {}", request.data),
                }),
            })
        }
    }

    #[derive(Default)]
    struct MockRequestBuilder {
        data: String,
    }

    impl MockRequestBuilder {
        pub fn data(mut self, data: impl Into<String>) -> Self {
            self.data = data.into();
            self
        }

        pub fn build(self) -> MockRequest {
            MockRequest { data: self.data }
        }
    }

    #[derive(Default)]
    struct MockRequestBuilderOwned {
        data: String,
    }

    impl MockRequestBuilderOwned {
        pub fn data(mut self, data: impl Into<String>) -> Self {
            self.data = data.into();
            self
        }

        pub fn build(self) -> MockRequest {
            MockRequest { data: self.data }
        }
    }

    // Use the macro to implement the trait
    crate::impl_executable_builder!(
        MockRequestBuilder,
        MockService,
        MockRequest,
        BaseResponse<MockResponse>,
        test_method
    );

    crate::impl_executable_builder_owned!(
        MockRequestBuilderOwned,
        MockService,
        MockRequest,
        BaseResponse<MockResponse>,
        test_method_owned
    );

    async fn mock_config_function(
        request: MockRequest,
        _config: &Config,
        _option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MockResponse>> {
        Ok(BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: Some(MockResponse {
                result: format!("config: {}", request.data),
            }),
        })
    }

    #[derive(Default)]
    struct MockConfigBuilder {
        data: String,
    }

    impl MockConfigBuilder {
        pub fn data(mut self, data: impl Into<String>) -> Self {
            self.data = data.into();
            self
        }

        pub fn build(self) -> MockRequest {
            MockRequest { data: self.data }
        }
    }

    crate::impl_executable_builder_config!(
        MockConfigBuilder,
        MockRequest,
        BaseResponse<MockResponse>,
        mock_config_function
    );

    #[tokio::test]
    async fn test_executable_builder_macro() {
        let service = MockService;
        let builder = MockRequestBuilder::default().data("test data");

        let result = builder.execute(&service).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(
            response.data.as_ref().unwrap().result,
            "processed: test data"
        );
    }

    #[tokio::test]
    async fn test_executable_builder_macro_with_options() {
        let service = MockService;
        let builder = MockRequestBuilder::default().data("test with options");
        let option = RequestOption::default();

        let result = builder.execute_with_options(&service, option).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(
            response.data.as_ref().unwrap().result,
            "processed: test with options"
        );
    }

    #[tokio::test]
    async fn test_executable_builder_owned_macro() {
        let service = MockService;
        let builder = MockRequestBuilderOwned::default().data("owned test");

        let result = builder.execute(&service).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(response.data.as_ref().unwrap().result, "owned: owned test");
    }

    #[tokio::test]
    async fn test_executable_builder_config_macro() {
        let config = Config::default();
        let builder = MockConfigBuilder::default().data("config test");

        let result = builder.execute(&config).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(
            response.data.as_ref().unwrap().result,
            "config: config test"
        );
    }

    #[tokio::test]
    async fn test_executable_builder_config_macro_with_options() {
        let config = Config::default();
        let builder = MockConfigBuilder::default().data("config with options");
        let option = RequestOption::default();

        let result = builder.execute_with_options(&config, option).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.code(), 0);
        assert_eq!(
            response.data.as_ref().unwrap().result,
            "config: config with options"
        );
    }

    #[test]
    fn test_builder_construction() {
        let builder = MockRequestBuilder::default().data("test");
        let request = builder.build();
        assert_eq!(request.data, "test");
    }

    #[test]
    fn test_builder_chaining() {
        let builder = MockRequestBuilder::default().data("first").data("second");
        let request = builder.build();
        assert_eq!(request.data, "second");
    }

    #[test]
    fn test_owned_builder_construction() {
        let builder = MockRequestBuilderOwned::default().data("owned test");
        let request = builder.build();
        assert_eq!(request.data, "owned test");
    }

    #[test]
    fn test_config_builder_construction() {
        let builder = MockConfigBuilder::default().data("config builder test");
        let request = builder.build();
        assert_eq!(request.data, "config builder test");
    }

    #[test]
    fn test_mock_response_api_trait() {
        let format = MockResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_mock_response_serialization() {
        let response = MockResponse {
            result: "test result".to_string(),
        };

        let serialized = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: MockResponse =
            serde_json::from_str(&serialized).expect("Should deserialize");

        assert_eq!(response.result, deserialized.result);
    }

    #[test]
    fn test_mock_request_debug() {
        let request = MockRequest {
            data: "debug test".to_string(),
        };

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("MockRequest"));
        assert!(debug_str.contains("debug test"));
    }

    #[test]
    fn test_mock_request_clone() {
        let request = MockRequest {
            data: "clone test".to_string(),
        };

        let cloned = request.clone();
        assert_eq!(request.data, cloned.data);
    }

    #[test]
    fn test_builder_with_empty_data() {
        let builder = MockRequestBuilder::default().data("");
        let request = builder.build();
        assert_eq!(request.data, "");
    }

    #[test]
    fn test_builder_with_unicode_data() {
        let builder = MockRequestBuilder::default().data("测试数据 🚀");
        let request = builder.build();
        assert_eq!(request.data, "测试数据 🚀");
    }

    #[test]
    fn test_builder_with_long_data() {
        let long_data = "a".repeat(10000);
        let builder = MockRequestBuilder::default().data(&long_data);
        let request = builder.build();
        assert_eq!(request.data, long_data);
    }
}
