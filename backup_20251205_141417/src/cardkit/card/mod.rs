//! Card 卡片实体API模块
//!
//! 提供卡片实体管理相关的功能，包括：
//! - 卡片的创建、更新、配置管理
//! - 批量更新操作
//! - 卡片ID类型转换
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::cardkit::card::{CardService, CreateCardRequest};
//!
//! let service = CardService::new(config);
//!
//! // 创建卡片
//! let response = service
//!     .create_card_builder()
//!     .card_content(card_content)
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(card_id) = response.card_id {
//!     println!("创建的卡片ID: {}", card_id);
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    BatchUpdateCardRequestBuilder, CardService, ConvertCardIdRequestBuilder,
    CreateCardRequestBuilder, UpdateCardRequestBuilder, UpdateCardSettingsRequestBuilder,
};
