//! 卡片组件（Cardkit）服务
//!
//! 提供飞书卡片组件的完整功能集，支持卡片创建、组件管理、交互设计、
//! 动态渲染等企业级卡片开发能力。是构建丰富交互界面的核心工具。
//!
//! # 核心功能
//!
//! ## 卡片管理
//! - 🎨 卡片模板创建和管理
//! - 📝 卡片内容动态更新
//! - 🖼️ 卡片样式和布局设计
//! - 📊 卡片数据绑定和展示
//! - 🔄 卡片版本控制管理
//!
//! ## 组件系统
//! - 🧩 丰富的UI组件库
//! - 🎛️ 交互组件配置
//! - 📱 响应式组件布局
//! - 🎨 组件样式定制
//! - 🔗 组件数据绑定
//!
//! ## 交互设计
//! - ⚡ 用户交互事件处理
//! - 🔘 按钮和表单组件
//! - 📊 数据可视化组件
//! - 🎯 条件显示逻辑
//! - 📝 表单验证机制
//!
//! ## 动态渲染
//! - 🖥️ 实时内容渲染
//! - 📊 数据驱动界面更新
//! - 🎨 主题和样式切换
//! - 📱 多端适配支持
//! - ⚡ 高性能渲染引擎
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取卡片组件服务
//! let cardkit = &client.cardkit;
//!
//! // 创建卡片
//! // let card_request = CreateCardRequest::builder()
//! //     .template_id("template_123")
//! //     .data(serde_json::json!({
//! //         "title": "项目报告",
//! //         "content": "本月项目进展汇总",
//! //         "progress": 85
//! //     }))
//! //     .build();
//! // let new_card = cardkit.v1.card.create(card_request, None).await?;
//!
//! // 更新卡片内容
//! // let update_request = UpdateCardRequest::builder()
//! //     .card_id("card_456")
//! //     .data(serde_json::json!({
//! //         "progress": 90,
//! //         "status": "进行中"
//! //     }))
//! //     .build();
//! // cardkit.v1.card.update(update_request, None).await?;
//!
//! // 添加卡片组件
//! // let element_request = CreateCardElementRequest::builder()
//! //     .card_id("card_456")
//! //     .element_type("button")
//! //     .properties(serde_json::json!({
//! //         "text": "查看详情",
//! //         "action": "open_url",
//! //         "url": "https://project.company.com/report"
//! //     }))
//! //     .build();
//! // cardkit.v1.card_element.create(element_request, None).await?;
//!
//! // 处理卡片交互事件
//! // cardkit.v1.events.on_card_action(|event| {
//! //     println!("卡片交互事件: {:?}", event);
//! // });
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供完整的卡片功能：
//! - 卡片全生命周期管理
//! - 丰富的组件库支持
//! - 灵活的交互设计
//! - 强大的动态渲染能力
//!
//! # 卡片开发特性
//!
//! - 🎨 所见即所得的设计器
//! - 📊 数据驱动的界面开发
//! - 🔗 与企业系统深度集成
//! - 📱 跨平台一致性体验
//! - ⚡ 高性能实时更新
//!
//! # 应用场景
//!
//! - 📊 数据报表展示
//! - 📋 工作流程审批
//! - 📝 表单收集和处理
//! - 🎯 任务状态跟踪
//! - 📈 业务指标监控

use crate::core::config::Config;

pub mod v1;

/// 飞书卡片服务
///
/// 提供飞书卡片相关功能，包括卡片创建、更新、组件管理等
///
/// # 功能模块
/// - v1: 卡片 v1 API
///
/// # 示例
/// ```rust,ignore
/// use open_lark::LarkClient;
///
/// let client = LarkClient::builder("app_id", "app_secret").build();
///
/// // 创建卡片
/// let response = client.cardkit.v1.card.create(request, None).await?;
///
/// // 新增组件
/// let response = client.cardkit.v1.card_element.create(request, None).await?;
/// ```
pub struct CardkitService {
    /// v1 API
    pub v1: v1::V1,
}

impl CardkitService {
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cardkit_service_creation() {
        let config = Config::default();
        let service = CardkitService::new(config.clone());

        assert_eq!(service.v1.card.config.app_id, config.app_id);
        assert_eq!(service.v1.card.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_cardkit_service_with_custom_config() {
        let config = Config::builder()
            .app_id("cardkit_test_app")
            .app_secret("cardkit_test_secret")
            .req_timeout(Duration::from_secs(90))
            .build();

        let service = CardkitService::new(config.clone());

        assert_eq!(service.v1.card.config.app_id, "cardkit_test_app");
        assert_eq!(service.v1.card.config.app_secret, "cardkit_test_secret");
        assert_eq!(
            service.v1.card.config.req_timeout,
            Some(Duration::from_secs(90))
        );
    }

    #[test]
    fn test_cardkit_service_config_independence() {
        let config1 = Config::builder().app_id("cardkit_app_1").build();

        let config2 = Config::builder().app_id("cardkit_app_2").build();

        let service1 = CardkitService::new(config1);
        let service2 = CardkitService::new(config2);

        assert_eq!(service1.v1.card.config.app_id, "cardkit_app_1");
        assert_eq!(service2.v1.card.config.app_id, "cardkit_app_2");
        assert_ne!(
            service1.v1.card.config.app_id,
            service2.v1.card.config.app_id
        );
    }

    #[test]
    fn test_cardkit_service_v1_accessible() {
        let config = Config::default();
        let service = CardkitService::new(config.clone());

        assert_eq!(service.v1.card.config.app_id, config.app_id);
    }

    #[test]
    fn test_cardkit_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = CardkitService::new(config.clone());

        assert_eq!(service.v1.card.config.app_id, "clone_test_app");
        assert_eq!(service.v1.card.config.app_secret, "clone_test_secret");
    }

    #[test]
    fn test_cardkit_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = CardkitService::new(config);

        assert_eq!(
            service.v1.card.config.req_timeout,
            Some(Duration::from_secs(200))
        );
    }

    #[test]
    fn test_cardkit_service_multiple_instances() {
        let config = Config::default();

        let service1 = CardkitService::new(config.clone());
        let service2 = CardkitService::new(config.clone());

        assert_eq!(
            service1.v1.card.config.app_id,
            service2.v1.card.config.app_id
        );
        assert_eq!(
            service1.v1.card.config.app_secret,
            service2.v1.card.config.app_secret
        );
    }

    #[test]
    fn test_cardkit_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(120))
            .build();

        let service = CardkitService::new(config);

        assert_eq!(service.v1.card.config.app_id, "consistency_test");
        assert_eq!(service.v1.card.config.app_secret, "consistency_secret");
        assert_eq!(
            service.v1.card.config.req_timeout,
            Some(Duration::from_secs(120))
        );
    }
}
