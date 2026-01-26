//! Mock 服务器配置和测试运行时
//!
//! 提供异步测试运行时的统一创建方式和 Mock 服务器配置。

use tokio::runtime::Runtime;

/// 异步测试运行时助手
///
/// 封装 Tokio Runtime 的创建，提供清晰的错误消息。
///
/// # 示例
///
/// ```rust,ignore
/// let rt = TestRuntime::new();
/// let result = rt.block_on(async { some_api().await });
/// ```
#[derive(Debug)]
pub struct TestRuntime {
    rt: Runtime,
}

impl TestRuntime {
    /// 创建新的测试运行时
    ///
    /// 如果创建失败，会 panic 并显示清晰的错误信息。
    pub fn new() -> Self {
        Self {
            rt: Runtime::new().expect(
                "Failed to create test runtime. This may indicate insufficient system resources.",
            ),
        }
    }

    /// 在运行时中执行异步代码
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: std::future::Future,
    {
        self.rt.block_on(future)
    }
}

impl Default for TestRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// 快捷方法：创建默认测试运行时
///
/// # 示例
///
/// ```rust,ignore
/// let rt = test_runtime();
/// ```
pub fn test_runtime() -> Runtime {
    Runtime::new()
        .expect("Failed to create test runtime. This may indicate insufficient system resources.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_runtime_creation() {
        let _rt = TestRuntime::new();
        // 如果到达这里，说明 Runtime 创建成功
    }

    #[test]
    fn test_test_runtime_block_on() {
        let rt = TestRuntime::new();
        let result = rt.block_on(async { 42 });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_test_runtime_default() {
        let rt = TestRuntime::default();
        let result = rt.block_on(async { "test".to_string() });
        assert_eq!(result, "test");
    }
}
