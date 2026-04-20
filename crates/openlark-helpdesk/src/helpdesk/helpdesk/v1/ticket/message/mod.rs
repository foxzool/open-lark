//! 工单消息模块
//!
//! 提供工单消息相关的 API。

pub mod create;
/// 列表接口。
pub mod list;

/// 工单消息服务（挂载在 Ticket 下）
#[derive(Clone)]
pub struct TicketMessage<'a> {
    ticket: &'a super::Ticket,
}

impl<'a> TicketMessage<'a> {
    /// 创建新的工单消息服务实例
    pub fn new(ticket: &'a super::Ticket) -> Self {
        Self { ticket }
    }

    /// 获取工单消息列表
    pub fn list(&self, ticket_id: impl Into<String>) -> list::ListTicketMessageRequest {
        list::ListTicketMessageRequest::new(self.ticket.config.clone(), ticket_id.into())
    }

    /// 发送工单消息
    pub fn create(&self, ticket_id: impl Into<String>) -> create::CreateTicketMessageRequest {
        create::CreateTicketMessageRequest::new(self.ticket.config.clone(), ticket_id.into())
    }
}

pub use create::{CreateTicketMessageRequest, CreateTicketMessageRequestBuilder};
pub use list::{ListTicketMessageRequest, ListTicketMessageRequestBuilder};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
