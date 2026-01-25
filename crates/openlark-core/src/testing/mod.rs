//! 现代化测试基础设施模块
//!
//! 提供类型安全的测试工具，消除测试中的 unwrap() 调用，建立清晰的测试模式。
//!
//! # 模块结构
//!
//! - [`assertions`]：类型安全的断言宏系统
//! - [`fixtures`]：统一的测试夹具和配置构建器
//! - [`mock_context`]：Mock 服务器配置和测试运行时
//!
//! # 使用示例
//!
//! ```rust,ignore
//! use openlark_core::testing::prelude::*;
//!
//! #[test]
//! fn test_example() {
//!     let config = TestConfigBuilder::new().build();
//!     let rt = TestRuntime::new();
//!
//!     let result = rt.block_on(async { some_api().await });
//!     let response = assert_res_ok!(result, "test_example");
//!     assert_eq!(response.id, "123");
//! }
//! ```

pub mod assertions;
pub mod fixtures;
pub mod mock_context;

/// 预置导入模块
/// 包含所有常用的测试工具和宏
pub mod prelude {
    pub use super::fixtures::{test_config, TestConfigBuilder};
    pub use super::mock_context::{test_runtime, TestRuntime};

    // 宏通过 #[macro_export] 自动导出，不需要 pub use
}
