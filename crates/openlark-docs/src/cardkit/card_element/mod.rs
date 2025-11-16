//! Card Element 卡片组件API模块
//!
//! 提供卡片组件管理相关的功能，包括：
//! - 卡片组件的创建、更新、删除
//! - 组件属性的局部更新
//! - 流式文本更新
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::cardkit::card_element::{CardElementService, CreateCardElementRequest};
//!
//! let service = CardElementService::new(config);
//!
//! // 创建卡片组件
//! let response = service
//!     .create_card_element_builder()
//!     .card_id("card_token_xxx")
//!     .element(element_data)
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(element_id) = response.element_id {
//!     println!("创建的组件ID: {}", element_id);
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    CardElementService, CreateCardElementRequestBuilder, DeleteCardElementRequestBuilder,
    PatchCardElementRequestBuilder, UpdateCardElementContentRequestBuilder,
    UpdateCardElementRequestBuilder,
};
