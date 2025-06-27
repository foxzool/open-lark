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
