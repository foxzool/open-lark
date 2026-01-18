//! Event (事件系统) 端点定义
//!
//! 事件系统 - 事件订阅、处理、分发机制
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::event::*;
//!
//! let subscriptions_endpoint = EVENT_V1_SUBSCRIPTIONS;
//! let history_endpoint = EVENT_V1_HISTORY;
//! ```

// ==================== Event (事件系统) v1 ====================
// 事件系统 - 事件订阅、处理、分发机制

/// Event事件订阅
/// 系统事件的订阅和管理
pub const EVENT_V1_SUBSCRIPTIONS: &str = "/open-apis/event/v1/subscriptions";
pub const EVENT_V1_SUBSCRIPTION_CREATE: &str = "/open-apis/event/v1/subscriptions/create";
pub const EVENT_V1_SUBSCRIPTION_GET: &str = "/open-apis/event/v1/subscriptions/{subscription_id}";
pub const EVENT_V1_SUBSCRIPTION_DELETE: &str =
    "/open-apis/event/v1/subscriptions/{subscription_id}";
pub const EVENT_V1_SUBSCRIPTION_UPDATE: &str =
    "/open-apis/event/v1/subscriptions/{subscription_id}";

/// Event事件历史
/// 历史事件查询和重放
pub const EVENT_V1_HISTORY: &str = "/open-apis/event/v1/history";
pub const EVENT_V1_HISTORY_REPLAY: &str = "/open-apis/event/v1/history/replay";

/// Event事件分发
/// 事件的分发和路由
pub const EVENT_V1_DISPATCHER: &str = "/open-apis/event/v1/dispatcher";
pub const EVENT_V1_DISPATCHER_STATUS: &str = "/open-apis/event/v1/dispatcher/status";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_endpoints() {
        assert!(EVENT_V1_SUBSCRIPTIONS.starts_with("/open-apis/event/v1/"));
        assert!(EVENT_V1_SUBSCRIPTIONS.contains("subscriptions"));
        assert!(EVENT_V1_HISTORY.contains("history"));
        assert!(EVENT_V1_DISPATCHER.contains("dispatcher"));
        assert!(EVENT_V1_SUBSCRIPTION_CREATE.contains("create"));
        assert!(EVENT_V1_SUBSCRIPTION_DELETE.contains("{subscription_id}"));
    }
}
