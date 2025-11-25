//! 客户端初始化和设置工具
//!
//! 提供标准化的客户端创建和配置功能。

use openlark_client::prelude::*;
use std::env;

/// 创建标准客户端
///
/// 从环境变量读取配置信息并创建客户端实例
///
/// # 环境变量
/// - `OPENLARK_APP_ID`: 飞书应用ID
/// - `OPENLARK_APP_SECRET`: 飞书应用密钥
///
/// # 返回值
/// - `Ok(Client)`: 配置好的客户端实例
/// - `Err(Error)`: 配置错误信息
///
/// # 示例
/// ```rust,no_run
/// use openlark_examples_common::create_client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = create_client()?;
///     println!("客户端创建成功");
///     Ok(())
/// }
/// ```
pub fn create_client() -> std::result::Result<Client, Error> {
    Client::from_env()
}

/// 创建测试客户端
///
/// 使用测试环境的配置创建客户端实例
///
/// # 返回值
/// - `Ok(Client)`: 测试客户端实例
/// - `Err(Error)`: 配置错误信息
pub fn create_test_client() -> std::result::Result<Client, Error> {
    // 尝试使用测试环境变量，如果不存在则使用默认值
    let app_id = env::var("TEST_APP_ID").unwrap_or_else(|_| "test_app_id".to_string());
    let app_secret = env::var("TEST_APP_SECRET").unwrap_or_else(|_| "test_app_secret".to_string());

    Client::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build()
}

/// 创建自定义配置的客户端
///
/// # 参数
/// - `app_id`: 应用ID
/// - `app_secret`: 应用密钥
/// - `base_url`: API基础URL（可选）
///
/// # 返回值
/// - `Ok(Client)`: 配置好的客户端实例
/// - `Err(Error)`: 配置错误信息
pub fn create_custom_client(
    app_id: &str,
    app_secret: &str,
    _base_url: Option<&str>, // TODO: 需要在ClientBuilder中添加base_url方法
) -> std::result::Result<Client, Error> {
    Client::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build()
}
