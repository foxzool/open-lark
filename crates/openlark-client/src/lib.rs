//! OpenLark Client Library
//!
//! 高级客户端库，提供全新的统一客户端接口，让所有飞书服务都有一致的调用体验。
//!
//! ## 主要功能
//!
//! - **统一API接口**: 所有服务使用相同的调用模式
//! - **类型安全**: 编译时验证的API调用和配置
//! - **异步支持**: 完全异步的客户端实现
//! - **构建器模式**: 现代化的客户端构建方式
//! - **批量操作**: 支持批量API调用以提高效率
//! - **中间件支持**: 可插拔的中间件系统
//!
//! ## 快速开始
//!
//! ### 使用高级统一API（推荐）
//!
//! ```rust
//! use openlark_client::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> UnifiedResult<()> {
//!     // 从环境变量创建客户端
//!     let client = OpenLarkClient::from_env().await?;
//!
//!     // 发送文本消息
//!     let result = client.send_text_message(
//!         "user_open_id",
//!         "open_id",
//!         "Hello from OpenLark!"
//!     ).await?;
//!     println!("消息发送成功: {}", result.message_id);
//!
//!     // 获取员工列表
//!     let employees = client.list_employees(Some("open_id"), Some(50), None).await?;
//!     for employee in employees.employees {
//!         println!("员工: {} ({})", employee.name, employee.user_id);
//!     }
//!
//!     // 创建电子表格
//!     let spreadsheet = client.create_spreadsheet("我的新表格", None).await?;
//!     println!("表格创建成功: {}", spreadsheet.url);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 使用构建器模式
//!
//! ```rust
//! use openlark_client::prelude::*;
//!
//! let client = OpenLarkClientBuilder::new()
//!     .from_env()?
//!     .timeout(Duration::from_secs(30))
//!     .retries(3)
//!     .build()
//!     .await?;
//! ```
//!
//! ### 服务特定API
//!
//! ```rust
//! use openlark_client::prelude::*;
//!
//! let client = OpenLarkClient::from_env().await?;
//!
//! // 获取通信服务API
//! let comm = client.communication()?;
//! let result = comm.send_text_message("user_id", "open_id", "Hello").await?;
//!
//! // 获取HR服务API
//! let hr = client.hr()?;
//! let employees = hr.list_employees(Some("open_id"), Some(20), None).await?;
//!
//! // 获取文档服务API
//! let docs = client.docs()?;
//! let sheet = docs.create_spreadsheet("新表格", None).await?;
//!
//! // 获取AI服务API
//! let ai = client.ai()?;
//! let text = ai.generate_text("写一首诗", None, Some(0.7), Some(100)).await?;
//!
//! // 获取认证服务API
//! let auth = client.auth()?;
//! let token = auth.get_app_access_token().await?;
//! ```
//!
//! ## 批量操作
//!
//! ```rust
//! use openlark_client::prelude::*;
//!
//! let client = OpenLarkClient::from_env().await?;
//!
//! // 批量发送消息
//! let messages = vec![
//!     ("user1".to_string(), "open_id".to_string(), "Hello 1".to_string()),
//!     ("user2".to_string(), "open_id".to_string(), "Hello 2".to_string()),
//! ];
//! let results = client.batch_send_text_messages(messages).await?;
//!
//! // 批量获取员工信息
//! let user_ids = vec!["user1".to_string(), "user2".to_string()];
//! let employees = client.batch_get_employees(user_ids, Some("open_id")).await?;
//! ```

#![allow(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// Include macros first
#[macro_use]
mod macros;

// New unified client architecture (recommended)
pub mod unified;

// Legacy modules (for backward compatibility)
pub mod accessors;
pub mod services;
pub mod ws_client;

// Internal legacy modules
mod client;
mod prelude;
pub mod registry;
pub mod traits;

// Re-export OpenLark client as the main interface (recommended)
pub use unified::{
    OpenLarkClient, OpenLarkClientBuilder,
    UnifiedClient, UnifiedClientBuilder, UnifiedConfig, ConfigBuilder,
    UnifiedService, UnifiedError, UnifiedResult, ServiceRegistry,
    TransportLayer, Middleware, MiddlewareChain, APICall, ServiceLifecycle,
    APIRequest, APIResponse, UnifiedAPIClient,
    CommunicationAPI, HRAPI, DocsAPI, AIAPI, AuthAPI,
};

// Re-export legacy client for backward compatibility
pub use client::{LarkClient, LarkClientBuilder};

// Type alias for backward compatibility
/// Default LarkClient type for backward compatibility
pub type DefaultLarkClient = LarkClient;

/// 为服务crate提供便利的重新导出，与openlark_core::client保持兼容
pub mod compatibility {
    pub use super::{LarkClient, LarkClientBuilder, DefaultLarkClient};

    // 重新导出常用的相关类型，确保服务crate迁移时无需修改import
    pub use openlark_core::{SDKResult, config::Config, constants::AppType};
}

// Re-export core types
pub use openlark_core::{SDKResult, config::Config as CoreConfig};