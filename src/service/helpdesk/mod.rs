//! 服务台（Helpdesk）服务
//!
//! 提供飞书服务台的完整功能集，支持工单管理、客户服务、知识库、
//! 自动化流程等企业级客户服务能力。是企业客户支持和服务的核心平台。
//!
//! # 核心功能
//!
//! ## 工单管理
//! - 📋 工单创建、分配和跟踪
//! - ⏰ 工单优先级和SLA管理
//! - 📊 工单状态和流程控制
//! - 🔄 工单转移和升级
//! - 📈 工单统计和报表
//!
//! ## 客户服务
//! - 👥 客户信息管理和查询
//! - 💬 多渠道客户沟通
//! - 📞 客服座席分配和管理
//! - 🎯 客户满意度调查
//! - 📊 服务质量监控
//!
//! ## 知识库管理
//! - 📚 知识文章创建和管理
//! - 🔍 智能知识搜索
//! - 📖 知识分类和标签
//! - 👥 知识共享和协作
//! - 📈 知识使用统计
//!
//! ## 自动化流程
//! - 🤖 工单自动分配规则
//! - 📧 自动回复和通知
//! - 🔄 流程自动化和触发器
//! - 📊 自动生成统计报表
//! - 🎯 智能推荐和建议
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
//! // 获取服务台服务
//! let helpdesk = &client.helpdesk;
//!
//! // 创建工单
//! // let ticket_request = CreateTicketRequest::builder()
//! //     .title("登录问题咨询")
//! //     .description("用户反馈无法正常登录系统")
//! //     .priority("high")
//! //     .customer_id("customer_123")
//! //     .build();
//! // let ticket = helpdesk.v1.ticket.create(ticket_request, None).await?;
//!
//! // 查询工单列表
//! // let list_request = ListTicketsRequest::builder()
//! //     .status("open")
//! //     .page_size(20)
//! //     .build();
//! // let tickets = helpdesk.v1.ticket.list(list_request, None).await?;
//!
//! // 更新工单状态
//! // let update_request = UpdateTicketRequest::builder()
//! //     .ticket_id("ticket_123")
//! //     .status("resolved")
//! //     .resolution("问题已解决，用户可以正常登录")
//! //     .build();
//! // helpdesk.v1.ticket.update(update_request, None).await?;
//!
//! // 添加工单评论
//! // let comment_request = CreateTicketCommentRequest::builder()
//! //     .ticket_id("ticket_123")
//! //     .content("已联系技术团队，正在处理中")
//! //     .build();
//! // helpdesk.v1.ticket.add_comment(comment_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的服务台功能：
//! - 工单全生命周期管理
//! - 客户服务和支持
//! - 知识库管理
//! - 基础的自动化功能
//!
//! # 服务台特性
//!
//! - 📱 多渠道客户接入
//! - 🤖 智能客服和机器人
//! - 📊 实时监控和报表
//! - 🔔 及时通知和提醒
//! - 🎯 个性化服务体验
//!
//! # 服务管理
//!
//! - 📈 服务质量监控和分析
//! - 👥 客服团队管理和培训
//! - 🎯 SLA目标设定和监控
//! - 📊 客户满意度评估
//! - 🔄 持续改进和优化

/// 数据模型定义
pub mod models;
/// 服务台服务 v1 版本
pub mod v1;

use crate::core::config::Config;

/// 服务台服务
///
/// 企业级客户服务的统一入口，提供工单管理、客户服务、
/// 知识库、自动化流程等完整的客户支持能力。
///
/// # 服务架构
///
/// - **v1**: 服务台API v1版本，提供基础功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 📋 完整的工单管理系统
/// - 👥 专业的客户服务支持
/// - 📚 智能的知识库管理
/// - 🤖 灵活的自动化流程
/// - 📊 全面的数据分析
///
/// # 适用场景
///
/// - 企业客户服务支持
/// - 内部IT服务台
/// - 技术支持和咨询
/// - 客户投诉处理
/// - 知识管理和共享
///
/// # 最佳实践
///
/// - 建立清晰的服务流程
/// - 设定合理的SLA目标
/// - 维护丰富的知识库
/// - 持续优化服务质量
/// - 重视客户反馈和满意度
pub struct HelpdeskService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl HelpdeskService {
    /// 创建新的服务台服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的服务台服务实例
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
    fn test_helpdesk_service_creation() {
        let config = Config::default();
        let service = HelpdeskService::new(config.clone());

        // Verify all 11 sub-services are configured correctly
        assert_eq!(service.v1.agent.config.app_id, config.app_id);
        assert_eq!(service.v1.agent.config.app_secret, config.app_secret);
        assert_eq!(service.v1.agent_schedule.config.app_id, config.app_id);
        assert_eq!(service.v1.agent_skill.config.app_id, config.app_id);
        assert_eq!(service.v1.agent_skill_rule.config.app_id, config.app_id);
        assert_eq!(service.v1.category.config.app_id, config.app_id);
        assert_eq!(service.v1.event.config.app_id, config.app_id);
        assert_eq!(service.v1.faq.config.app_id, config.app_id);
        assert_eq!(service.v1.notification.config.app_id, config.app_id);
        assert_eq!(service.v1.ticket.config.app_id, config.app_id);
        assert_eq!(
            service.v1.ticket_customized_field.config.app_id,
            config.app_id
        );
        assert_eq!(service.v1.ticket_message.config.app_id, config.app_id);
    }

    #[test]
    fn test_helpdesk_service_with_custom_config() {
        let config = Config::builder()
            .app_id("helpdesk_test_app")
            .app_secret("helpdesk_test_secret")
            .req_timeout(Duration::from_secs(470))
            .build();

        let service = HelpdeskService::new(config.clone());

        assert_eq!(service.v1.agent.config.app_id, "helpdesk_test_app");
        assert_eq!(service.v1.agent.config.app_secret, "helpdesk_test_secret");
        assert_eq!(
            service.v1.agent.config.req_timeout,
            Some(Duration::from_secs(470))
        );
        assert_eq!(service.v1.agent_schedule.config.app_id, "helpdesk_test_app");
        assert_eq!(
            service.v1.agent_skill.config.req_timeout,
            Some(Duration::from_secs(470))
        );
        assert_eq!(
            service.v1.agent_skill_rule.config.app_id,
            "helpdesk_test_app"
        );
        assert_eq!(
            service.v1.category.config.req_timeout,
            Some(Duration::from_secs(470))
        );
        assert_eq!(service.v1.event.config.app_id, "helpdesk_test_app");
        assert_eq!(
            service.v1.faq.config.req_timeout,
            Some(Duration::from_secs(470))
        );
        assert_eq!(service.v1.notification.config.app_id, "helpdesk_test_app");
        assert_eq!(
            service.v1.ticket.config.req_timeout,
            Some(Duration::from_secs(470))
        );
        assert_eq!(
            service.v1.ticket_customized_field.config.app_id,
            "helpdesk_test_app"
        );
        assert_eq!(
            service.v1.ticket_message.config.req_timeout,
            Some(Duration::from_secs(470))
        );
    }

    #[test]
    fn test_helpdesk_service_config_independence() {
        let config1 = Config::builder().app_id("helpdesk_app_1").build();

        let config2 = Config::builder().app_id("helpdesk_app_2").build();

        let service1 = HelpdeskService::new(config1);
        let service2 = HelpdeskService::new(config2);

        assert_eq!(service1.v1.agent.config.app_id, "helpdesk_app_1");
        assert_eq!(service2.v1.agent.config.app_id, "helpdesk_app_2");
        assert_ne!(
            service1.v1.agent.config.app_id,
            service2.v1.agent.config.app_id
        );
        assert_ne!(
            service1.v1.agent_schedule.config.app_id,
            service2.v1.agent_schedule.config.app_id
        );
        assert_ne!(
            service1.v1.agent_skill.config.app_id,
            service2.v1.agent_skill.config.app_id
        );
        assert_ne!(
            service1.v1.agent_skill_rule.config.app_id,
            service2.v1.agent_skill_rule.config.app_id
        );
        assert_ne!(
            service1.v1.category.config.app_id,
            service2.v1.category.config.app_id
        );
        assert_ne!(
            service1.v1.event.config.app_id,
            service2.v1.event.config.app_id
        );
        assert_ne!(service1.v1.faq.config.app_id, service2.v1.faq.config.app_id);
        assert_ne!(
            service1.v1.notification.config.app_id,
            service2.v1.notification.config.app_id
        );
        assert_ne!(
            service1.v1.ticket.config.app_id,
            service2.v1.ticket.config.app_id
        );
        assert_ne!(
            service1.v1.ticket_customized_field.config.app_id,
            service2.v1.ticket_customized_field.config.app_id
        );
        assert_ne!(
            service1.v1.ticket_message.config.app_id,
            service2.v1.ticket_message.config.app_id
        );
    }

    #[test]
    fn test_helpdesk_service_sub_services_accessible() {
        let config = Config::default();
        let service = HelpdeskService::new(config.clone());

        // Test that all 11 sub-services are accessible
        assert_eq!(service.v1.agent.config.app_id, config.app_id);
        assert_eq!(service.v1.agent_schedule.config.app_id, config.app_id);
        assert_eq!(service.v1.agent_skill.config.app_id, config.app_id);
        assert_eq!(service.v1.agent_skill_rule.config.app_id, config.app_id);
        assert_eq!(service.v1.category.config.app_id, config.app_id);
        assert_eq!(service.v1.event.config.app_id, config.app_id);
        assert_eq!(service.v1.faq.config.app_id, config.app_id);
        assert_eq!(service.v1.notification.config.app_id, config.app_id);
        assert_eq!(service.v1.ticket.config.app_id, config.app_id);
        assert_eq!(
            service.v1.ticket_customized_field.config.app_id,
            config.app_id
        );
        assert_eq!(service.v1.ticket_message.config.app_id, config.app_id);
    }

    #[test]
    fn test_helpdesk_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = HelpdeskService::new(config.clone());

        assert_eq!(service.v1.agent.config.app_id, "clone_test_app");
        assert_eq!(service.v1.agent.config.app_secret, "clone_test_secret");
        assert_eq!(
            service.v1.agent_schedule.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v1.agent_skill.config.app_id, "clone_test_app");
        assert_eq!(
            service.v1.agent_skill_rule.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v1.category.config.app_id, "clone_test_app");
        assert_eq!(service.v1.event.config.app_secret, "clone_test_secret");
        assert_eq!(service.v1.faq.config.app_id, "clone_test_app");
        assert_eq!(
            service.v1.notification.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v1.ticket.config.app_id, "clone_test_app");
        assert_eq!(
            service.v1.ticket_customized_field.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.v1.ticket_message.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_helpdesk_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(480))
            .build();

        let service = HelpdeskService::new(config);

        // Verify timeout is propagated to all 11 sub-services
        assert_eq!(
            service.v1.agent.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.agent_schedule.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.agent_skill.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.agent_skill_rule.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.category.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.event.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.faq.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.notification.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.ticket.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.ticket_customized_field.config.req_timeout,
            Some(Duration::from_secs(480))
        );
        assert_eq!(
            service.v1.ticket_message.config.req_timeout,
            Some(Duration::from_secs(480))
        );
    }

    #[test]
    fn test_helpdesk_service_multiple_instances() {
        let config = Config::default();

        let service1 = HelpdeskService::new(config.clone());
        let service2 = HelpdeskService::new(config.clone());

        // Both services should have the same config values
        assert_eq!(
            service1.v1.agent.config.app_id,
            service2.v1.agent.config.app_id
        );
        assert_eq!(
            service1.v1.agent.config.app_secret,
            service2.v1.agent.config.app_secret
        );
        assert_eq!(
            service1.v1.agent_schedule.config.app_id,
            service2.v1.agent_schedule.config.app_id
        );
        assert_eq!(
            service1.v1.agent_skill.config.app_secret,
            service2.v1.agent_skill.config.app_secret
        );
        assert_eq!(
            service1.v1.agent_skill_rule.config.app_id,
            service2.v1.agent_skill_rule.config.app_id
        );
        assert_eq!(
            service1.v1.category.config.app_secret,
            service2.v1.category.config.app_secret
        );
        assert_eq!(
            service1.v1.event.config.app_id,
            service2.v1.event.config.app_id
        );
        assert_eq!(
            service1.v1.faq.config.app_secret,
            service2.v1.faq.config.app_secret
        );
        assert_eq!(
            service1.v1.notification.config.app_id,
            service2.v1.notification.config.app_id
        );
        assert_eq!(
            service1.v1.ticket.config.app_secret,
            service2.v1.ticket.config.app_secret
        );
        assert_eq!(
            service1.v1.ticket_customized_field.config.app_id,
            service2.v1.ticket_customized_field.config.app_id
        );
        assert_eq!(
            service1.v1.ticket_message.config.app_secret,
            service2.v1.ticket_message.config.app_secret
        );
    }

    #[test]
    fn test_helpdesk_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(490))
            .build();

        let service = HelpdeskService::new(config);

        // Verify all 11 sub-services have consistent configurations
        let configs = [
            &service.v1.agent.config,
            &service.v1.agent_schedule.config,
            &service.v1.agent_skill.config,
            &service.v1.agent_skill_rule.config,
            &service.v1.category.config,
            &service.v1.event.config,
            &service.v1.faq.config,
            &service.v1.notification.config,
            &service.v1.ticket.config,
            &service.v1.ticket_customized_field.config,
            &service.v1.ticket_message.config,
        ];

        for config in &configs {
            assert_eq!(config.app_id, "consistency_test");
            assert_eq!(config.app_secret, "consistency_secret");
            assert_eq!(config.req_timeout, Some(Duration::from_secs(490)));
        }
    }
}
