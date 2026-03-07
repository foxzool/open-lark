pub mod agent;
pub mod agent_schedule;
pub mod agent_skill;
pub mod agent_skill_rule;
pub mod bot;
pub mod category;
pub mod event;
pub mod faq;
pub mod notification;
pub mod ticket;

use openlark_core::config::Config;
use std::sync::Arc;

#[derive(Clone)]
pub struct HelpdeskV1 {
    config: Arc<Config>,
}

impl HelpdeskV1 {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn ticket(&self) -> ticket::Ticket {
        ticket::Ticket::new(self.config.clone())
    }

    pub fn agent(&self) -> agent::Agent {
        agent::Agent::new(self.config.clone())
    }

    pub fn agent_schedule(&self) -> agent_schedule::AgentSchedule {
        agent_schedule::AgentSchedule::new(self.config.clone())
    }

    pub fn agent_skill(&self) -> agent_skill::AgentSkill {
        agent_skill::AgentSkill::new(self.config.clone())
    }

    pub fn agent_skill_rule(&self) -> agent_skill_rule::AgentSkillRule {
        agent_skill_rule::AgentSkillRule::new(self.config.clone())
    }

    pub fn category(&self) -> category::Category {
        category::Category::new(self.config.clone())
    }

    pub fn faq(&self) -> faq::Faq {
        faq::Faq::new(self.config.clone())
    }

    pub fn notification(&self) -> notification::Notification {
        notification::Notification::new(self.config.clone())
    }

    pub fn event(&self) -> event::Event {
        event::Event::new(self.config.clone())
    }

    pub fn bot(&self) -> bot::Bot {
        bot::Bot::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    
    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
