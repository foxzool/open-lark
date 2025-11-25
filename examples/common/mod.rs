//! Examples共享工具库
//!
//! 提供所有示例代码的通用功能和工具函数，
//! 包括客户端设置、认证辅助、错误处理等。

pub mod client_setup;
// pub mod auth_helpers;  // 暂时注释掉，因为auth feature导入有问题
pub mod config;
pub mod utils;

// 重新导出常用功能，方便其他示例使用
pub use client_setup::{create_client, create_test_client};
// pub use auth_helpers::{setup_app_auth, create_auth_service};  // 暂时注释掉
pub use config::{
    create_client_with_config, generate_env_template, load_config_with_diagnostics,
    run_config_diagnostics, ConfigDiagnostics, ConfigError, ConfigLoadResult,
};
pub use utils::{
    check_env_vars_enhanced, handle_result, print_error, print_info, print_success, EnvCheckResult,
};

/// 示例程序的通用入口点宏
#[macro_export]
macro_rules! example_main {
    ($client_setup:block $logic:block) => {
        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error>> {
            // 设置日志
            env_logger::init();

            println!("🚀 开始执行示例程序...");

            // 客户端设置
            let client = $client_setup;

            // 主要逻辑
            $logic;

            println!("✅ 示例程序执行完成");
            Ok(())
        }
    };
}
