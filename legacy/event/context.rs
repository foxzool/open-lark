use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventContext {
    pub ts: Option<String>,
    pub uuid: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub schema: Option<String>,
    pub header: Option<EventHeader>,
    pub event: HashMap<String, Value>,
}

/// 事件头
#[derive(Debug, Serialize, Deserialize)]
pub struct EventHeader {
    /// 事件 ID
    pub event_id: Option<String>,
    /// 事件类型
    pub event_type: Option<String>,
    /// 事件创建时间戳（单位：毫秒）
    pub create_time: Option<String>,
    /// 事件 Token
    pub token: Option<String>,
    /// 应用 ID
    pub app_id: Option<String>,
    /// 租户 Key
    pub tenant_key: Option<String>,
}
