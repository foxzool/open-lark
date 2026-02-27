//! ExecutableBuilder trait
#![allow(async_fn_in_trait)]
//!
//! 为“Builder → Request → execute”模式提供统一的 trait 约束，
//! 以便在各业务 crate 里通过宏批量实现。

use crate::req_option::RequestOption;
use crate::SDKResult;

/// Builder 可执行抽象
pub trait ExecutableBuilder<S: Sync, Req, Resp>: Sized {
    /// 构建请求
    fn build(self) -> Req;

    /// 执行请求（默认不带 option）
    async fn execute(self, service: &S) -> SDKResult<Resp>;

    /// 执行请求（可选传入 RequestOption）
    ///
    /// 默认实现退化为 `execute`，以兼容仅实现基础 execute 的场景。
    async fn execute_with_options(self, service: &S, _option: RequestOption) -> SDKResult<Resp> {
        self.execute(service).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试用的 mock 类型
    struct MockService;
    struct MockRequest {
        value: String,
    }
    struct MockResponse {
        result: String,
    }

    struct MockBuilder {
        value: String,
    }

    impl ExecutableBuilder<MockService, MockRequest, MockResponse> for MockBuilder {
        fn build(self) -> MockRequest {
            MockRequest { value: self.value }
        }

        async fn execute(self, _service: &MockService) -> SDKResult<MockResponse> {
            Ok(MockResponse {
                result: "success".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_executable_builder_build() {
        let builder = MockBuilder {
            value: "test".to_string(),
        };
        let request = builder.build();
        assert_eq!(request.value, "test");
    }

    #[tokio::test]
    async fn test_executable_builder_execute() {
        let builder = MockBuilder {
            value: "test".to_string(),
        };
        let service = MockService;
        let result = builder.execute(&service).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().result, "success");
    }

    #[tokio::test]
    async fn test_executable_builder_execute_with_options() {
        let builder = MockBuilder {
            value: "test".to_string(),
        };
        let service = MockService;
        let option = RequestOption::default();
        let result = builder.execute_with_options(&service, option).await;
        assert!(result.is_ok());
    }
}
