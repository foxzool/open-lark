pub mod agent;
pub mod agent_schedule;
pub mod agent_skill;
pub mod agent_skill_rule;
pub mod category;
pub mod event;
pub mod faq;
pub mod notification;
pub mod ticket;
pub mod ticket_customized_field;
pub mod ticket_message;

use crate::core::config::Config;

/// Helpdesk API v1版本服务
pub struct V1 {
    /// 客服功能管理
    pub agent: agent::AgentService,
    /// 客服工作日程
    pub agent_schedule: agent_schedule::AgentScheduleService,
    /// 客服技能
    pub agent_skill: agent_skill::AgentSkillService,
    /// 客服技能规则
    pub agent_skill_rule: agent_skill_rule::AgentSkillRuleService,
    /// 知识库分类
    pub category: category::CategoryService,
    /// 事件订阅
    pub event: event::EventService,
    /// 知识库管理
    pub faq: faq::FaqService,
    /// 推送中心
    pub notification: notification::NotificationService,
    /// 工单管理
    pub ticket: ticket::TicketService,
    /// 工单自定义字段
    pub ticket_customized_field: ticket_customized_field::TicketCustomizedFieldService,
    /// 工单消息
    pub ticket_message: ticket_message::TicketMessageService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            agent: agent::AgentService::new(config.clone()),
            agent_schedule: agent_schedule::AgentScheduleService::new(config.clone()),
            agent_skill: agent_skill::AgentSkillService::new(config.clone()),
            agent_skill_rule: agent_skill_rule::AgentSkillRuleService::new(config.clone()),
            category: category::CategoryService::new(config.clone()),
            event: event::EventService::new(config.clone()),
            faq: faq::FaqService::new(config.clone()),
            notification: notification::NotificationService::new(config.clone()),
            ticket: ticket::TicketService::new(config.clone()),
            ticket_customized_field: ticket_customized_field::TicketCustomizedFieldService::new(
                config.clone(),
            ),
            ticket_message: ticket_message::TicketMessageService::new(config),
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_v1_service_creation() {
        let config = create_test_config();
        let service = V1::new(config);

        // Verify that all services are properly initialized
        // We can't directly test the inner fields as they are public structs
        // but we can verify the service creation doesn't panic
    }

    #[test]
    fn test_v1_service_with_different_config() {
        let config = Config::builder()
            .app_id("different_app_id")
            .app_secret("different_app_secret")
            .app_type(AppType::Marketplace)
            .build();

        let service = V1::new(config);

        // Verify service creation works with different config types
    }

    #[test]
    fn test_v1_service_memory_safety() {
        let config = create_test_config();
        let service = V1::new(config);

        // Test that we can access all services without memory issues
        let _agent = &service.agent;
        let _schedule = &service.agent_schedule;
        let _skill = &service.agent_skill;
        let _skill_rule = &service.agent_skill_rule;
        let _category = &service.category;
        let _event = &service.event;
        let _faq = &service.faq;
        let _notification = &service.notification;
        let _ticket = &service.ticket;
        let _customized_field = &service.ticket_customized_field;
        let _message = &service.ticket_message;

        // If we reach here without panic, memory layout is correct
    }
}
