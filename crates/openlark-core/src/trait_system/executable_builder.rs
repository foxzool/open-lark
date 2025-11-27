/// ExecutableBuilder trait - 统一的Builder执行接口
///
/// 这个trait为所有Builder提供了统一的execute方法接口，
/// 消除了手动实现重复execute方法的需要。
///
/// # 类型参数
/// - `TService`: 服务类型
/// - `TRequest`: 请求类型
/// - `TResponse`: 响应类型
///
/// # 方法
/// - `build()`: 构建请求对象
/// - `execute()`: 执行请求
/// - `execute_with_options()`: 带选项执行请求
use async_trait::async_trait;

#[async_trait]
pub trait ExecutableBuilder<TService, TRequest, TResponse>
where
    TService: Send + Sync,
    TRequest: Send + Sync,
    TResponse: Send + Sync,
{
    /// 构建请求对象
    fn build(self) -> TRequest;

    /// 执行请求并返回响应
    async fn execute(self, service: &TService) -> crate::SDKResult<TResponse>
    where
        Self: Sized;

    /// 带选项执行请求
    async fn execute_with_options(
        self,
        service: &TService,
        option: crate::req_option::RequestOption,
    ) -> crate::SDKResult<TResponse>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    // 测试用的模拟服务
    #[derive(Debug)]
    struct MockService {
        should_fail: bool,
        response_data: Option<String>,
    }

    impl MockService {
        fn new() -> Self {
            Self {
                should_fail: false,
                response_data: None,
            }
        }

        fn with_failure(mut self) -> Self {
            self.should_fail = true;
            self
        }

        fn with_response(mut self, data: String) -> Self {
            self.response_data = Some(data);
            self
        }

        async fn process_request(&self, request: TestRequest) -> crate::SDKResult<TestResponse> {
            if self.should_fail {
                return Err(crate::error::validation_error("simulation", "模拟失败"));
            }

            let response = TestResponse {
                id: request.id,
                message: request.message,
                data: self.response_data.clone(),
            };

            Ok(response)
        }

        async fn process_request_with_options(
            &self,
            request: TestRequest,
            _option: crate::req_option::RequestOption,
        ) -> crate::SDKResult<TestResponse> {
            self.process_request(request).await
        }
    }

    // 测试用的请求和响应类型
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    struct TestRequest {
        id: String,
        message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestResponse {
        id: String,
        message: String,
        data: Option<String>,
    }

    // 测试用的Builder实现
    #[derive(Debug, Default)]
    struct TestBuilder {
        id: Option<String>,
        message: Option<String>,
    }

    impl TestBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn id(mut self, id: impl ToString) -> Self {
            self.id = Some(id.to_string());
            self
        }

        fn message(mut self, message: impl ToString) -> Self {
            self.message = Some(message.to_string());
            self
        }

        fn with_option(self, _option: &crate::req_option::RequestOption) -> Self {
            // Store option if needed for future use
            self
        }

        fn build(self) -> TestRequest {
            TestRequest {
                id: self.id.unwrap_or_default(),
                message: self.message.unwrap_or_default(),
            }
        }
    }

    #[async_trait]
    impl ExecutableBuilder<MockService, TestRequest, TestResponse> for TestBuilder {
        fn build(self) -> TestRequest {
            self.build()
        }

        async fn execute(self, service: &MockService) -> crate::SDKResult<TestResponse>
        where
            Self: Sized,
        {
            let req = self.build();
            service.process_request(req).await
        }

        async fn execute_with_options(
            self,
            service: &MockService,
            option: crate::req_option::RequestOption,
        ) -> crate::SDKResult<TestResponse>
        where
            Self: Sized,
        {
            let req = self.build();
            service.process_request_with_options(req, option).await
        }
    }

    // 更复杂的测试Builder，支持链式调用
    #[derive(Debug, Default)]
    struct ChainableBuilder {
        request: TestRequest,
        option: Option<crate::req_option::RequestOption>,
    }

    impl ChainableBuilder {
        fn new() -> Self {
            Self::default()
        }

        fn id(mut self, id: impl ToString) -> Self {
            self.request.id = id.to_string();
            self
        }

        fn message(mut self, message: impl ToString) -> Self {
            self.request.message = message.to_string();
            self
        }

        fn with_option(mut self, option: crate::req_option::RequestOption) -> Self {
            self.option = Some(option);
            self
        }

        fn build(self) -> TestRequest {
            self.request
        }
    }

    #[async_trait]
    impl ExecutableBuilder<MockService, TestRequest, TestResponse> for ChainableBuilder {
        fn build(self) -> TestRequest {
            self.build()
        }

        async fn execute(self, service: &MockService) -> crate::SDKResult<TestResponse>
        where
            Self: Sized,
        {
            service.process_request(self.build()).await
        }

        async fn execute_with_options(
            self,
            service: &MockService,
            option: crate::req_option::RequestOption,
        ) -> crate::SDKResult<TestResponse>
        where
            Self: Sized,
        {
            let req = self.build();
            service.process_request_with_options(req, option).await
        }
    }

    // 测试泛型约束
    #[derive(Debug)]
    struct GenericService<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> GenericService<T> {
        fn new() -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }

        async fn handle_generic<U>(&self, _item: U) -> crate::SDKResult<String>
        where
            U: Send + Sync,
        {
            Ok("generic response".to_string())
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct GenericItem<T> {
        value: T,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[allow(dead_code)]
    struct GenericResponse {
        result: String,
    }

    // 泛型Builder实现
    #[derive(Debug)]
    struct GenericBuilder<T> {
        item: GenericItem<T>,
    }

    impl<T> GenericBuilder<T> {
        fn new(item: T) -> Self {
            Self {
                item: GenericItem { value: item },
            }
        }
    }

    #[async_trait]
    impl<T: Send + Sync> ExecutableBuilder<GenericService<T>, GenericItem<T>, String>
        for GenericBuilder<T>
    {
        fn build(self) -> GenericItem<T> {
            self.item
        }

        async fn execute(self, service: &GenericService<T>) -> crate::SDKResult<String>
        where
            Self: Sized,
        {
            service.handle_generic(self.item).await
        }

        async fn execute_with_options(
            self,
            service: &GenericService<T>,
            _option: crate::req_option::RequestOption,
        ) -> crate::SDKResult<String>
        where
            Self: Sized,
        {
            service.handle_generic(self.item).await
        }
    }

    #[tokio::test]
    async fn test_executable_builder_basic_execution() {
        let service = MockService::new().with_response("test_data".to_string());
        let builder = TestBuilder::new().id("test_id").message("test_message");

        let result = builder.execute(&service).await.unwrap();
        assert_eq!(result.id, "test_id");
        assert_eq!(result.message, "test_message");
        assert_eq!(result.data, Some("test_data".to_string()));
    }

    #[tokio::test]
    async fn test_executable_builder_with_options() {
        let service = MockService::new();
        let option = crate::req_option::RequestOption::builder()
            .tenant_key("test_tenant")
            .build();
        let builder = TestBuilder::new()
            .id("test_id")
            .message("test_message")
            .with_option(&option);

        let result = builder
            .execute_with_options(&service, option)
            .await
            .unwrap();
        assert_eq!(result.id, "test_id");
        assert_eq!(result.message, "test_message");
    }

    #[tokio::test]
    async fn test_executable_builder_failure_handling() {
        let service = MockService::new().with_failure();
        let builder = TestBuilder::new().id("test_id").message("test_message");

        let result = builder.execute(&service).await;
        assert!(result.is_err());
        let error_string = result.unwrap_err().to_string();
        assert!(error_string.contains("模拟失败"));
    }

    #[tokio::test]
    async fn test_executable_builder_chained_execution() {
        let service = MockService::new().with_response("chain_data".to_string());
        let option = crate::req_option::RequestOption::builder()
            .user_access_token("user_token")
            .build();

        let result = ChainableBuilder::new()
            .id("chain_id")
            .message("chain_message")
            .with_option(option)
            .execute(&service)
            .await
            .unwrap();

        assert_eq!(result.id, "chain_id");
        assert_eq!(result.message, "chain_message");
        assert_eq!(result.data, Some("chain_data".to_string()));
    }

    #[tokio::test]
    async fn test_executable_builder_generic_types() {
        let service = GenericService::new();
        let builder = GenericBuilder::new(42i32);

        let result = builder.execute(&service).await.unwrap();
        assert_eq!(result, "generic response");
    }

    #[test]
    fn test_executable_builder_build_method() {
        let builder = TestBuilder::new().id("build_test").message("build_message");

        let request = builder.build();
        assert_eq!(request.id, "build_test");
        assert_eq!(request.message, "build_message");
    }

    #[test]
    fn test_executable_builder_default_values() {
        let builder = TestBuilder::new();
        let request = builder.build();

        assert_eq!(request.id, "");
        assert_eq!(request.message, "");
    }

    #[tokio::test]
    async fn test_executable_builder_arc_service() {
        let service = Arc::new(MockService::new().with_response("arc_test".to_string()));
        let builder = TestBuilder::new().id("arc_id").message("arc_message");

        // 确保可以与 Arc 共享的服务一起工作
        let result = builder.execute(&*service).await.unwrap();
        assert_eq!(result.id, "arc_id");
        assert_eq!(result.message, "arc_message");
        assert_eq!(result.data, Some("arc_test".to_string()));
    }

    #[tokio::test]
    async fn test_executable_builder_concurrent_execution() {
        let service = Arc::new(MockService::new().with_response("concurrent_test".to_string()));

        // 创建多个builder并并发执行
        let builder1 = TestBuilder::new().id("id1").message("msg1");
        let builder2 = TestBuilder::new().id("id2").message("msg2");
        let builder3 = TestBuilder::new().id("id3").message("msg3");

        let (result1, result2, result3) = tokio::join!(
            builder1.execute(&*service),
            builder2.execute(&*service),
            builder3.execute(&*service)
        );

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        let r1 = result1.unwrap();
        let r2 = result2.unwrap();
        let r3 = result3.unwrap();

        assert_eq!(r1.id, "id1");
        assert_eq!(r2.id, "id2");
        assert_eq!(r3.id, "id3");
        assert_eq!(r1.message, "msg1");
        assert_eq!(r2.message, "msg2");
        assert_eq!(r3.message, "msg3");
    }

    #[tokio::test]
    async fn test_executable_builder_error_propagation() {
        // 测试错误传播
        let service = MockService::new().with_failure();
        let builder = TestBuilder::new().id("error_test").message("error_msg");

        // execute 方法应该传播错误
        let builder_clone = TestBuilder::new().id("error_test").message("error_msg");
        let execute_result = builder_clone.execute(&service).await;
        assert!(execute_result.is_err());

        // execute_with_options 方法也应该传播错误
        let option = crate::req_option::RequestOption::default();
        let execute_with_options_result = builder.execute_with_options(&service, option).await;
        assert!(execute_with_options_result.is_err());

        // 确保错误类型一致
        let execute_error = execute_result.unwrap_err();
        let options_error = execute_with_options_result.unwrap_err();
        assert_eq!(execute_error.to_string(), options_error.to_string());
    }

    #[test]
    fn test_executable_builder_trait_object_safety() {
        // 验证 trait 对象安全性
        fn takes_builder<B>(builder: B)
        where
            B: ExecutableBuilder<MockService, TestRequest, TestResponse>,
        {
            let _ = builder.build();
        }

        let builder = TestBuilder::new().id("trait_test").message("trait_msg");
        takes_builder(builder);
    }

    #[tokio::test]
    async fn test_executable_builder_complex_scenario() {
        // 测试复杂场景：多个步骤的构建和执行
        let service = MockService::new();

        // 步骤1：创建基础请求
        let base_request = TestBuilder::new().id("complex_id").build();

        // 步骤2：基于基础请求创建builder（模拟复杂构建逻辑）
        let complex_builder = TestBuilder::new()
            .id(base_request.id)
            .message("complex_message");

        // 步骤3：执行请求
        let result = complex_builder.execute(&service).await.unwrap();

        assert_eq!(result.id, "complex_id");
        assert_eq!(result.message, "complex_message");
    }

    #[tokio::test]
    async fn test_executable_builder_serialization_roundtrip() {
        // 测试序列化和反序列化的一致性
        let builder = TestBuilder::new().id("serde_test").message("serde_message");

        let request = builder.build();
        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: TestRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request, deserialized);

        // 验证builder构建的请求与序列化后的请求一致
        assert_eq!(request, deserialized);
    }

    #[test]
    fn test_executable_builder_send_sync_bounds() {
        // 验证 Send + Sync 约束
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}

        // MockService, TestRequest, TestResponse 都应该满足 Send + Sync
        assert_send::<MockService>();
        assert_sync::<MockService>();
        assert_send::<TestRequest>();
        assert_sync::<TestRequest>();
        assert_send::<TestResponse>();
        assert_sync::<TestResponse>();
        assert_send::<TestBuilder>();
        assert_sync::<TestBuilder>();
    }

    #[test]
    fn test_executable_builder_builder_pattern_consistency() {
        // 测试构建器模式的一致性
        let builder1 = TestBuilder::new()
            .id("pattern_test")
            .message("pattern_message");

        let builder2 = TestBuilder::new()
            .id("pattern_test")
            .message("pattern_message");

        assert_eq!(builder1.build(), builder2.build());

        // 测试链式调用的顺序独立性
        let builder3 = TestBuilder::new().message("msg_first").id("id_second");

        let builder4 = TestBuilder::new().id("id_second").message("msg_first");

        assert_eq!(builder3.build(), builder4.build());
    }

    #[tokio::test]
    async fn test_executable_builder_with_custom_options() {
        // 测试自定义 RequestOption
        let service = MockService::new();
        let custom_option = crate::req_option::RequestOption::builder()
            .tenant_key("custom_tenant")
            .user_access_token("custom_user_token")
            .request_id("custom_request_id")
            .add_header("X-Custom-Header", "custom_value")
            .build();

        let builder = TestBuilder::new()
            .id("custom_test")
            .message("custom_message");

        let result = builder
            .execute_with_options(&service, custom_option)
            .await
            .unwrap();

        assert_eq!(result.id, "custom_test");
        assert_eq!(result.message, "custom_message");
    }
}
