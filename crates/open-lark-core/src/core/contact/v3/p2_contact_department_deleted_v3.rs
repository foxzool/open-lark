use crate::core::{
    api_resp::ApiResponseTrait, config::Config,
};
use serde::{Deserialize, Serialize};

/// Department删除事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcontactUdepartmentUdeletedUv3Event {
    /// 事件ID
    pub event_id: String,
    /// Department ID
    pub department_id: String,
    /// 事件时间戳
    pub event_time: i64,
}

impl ApiResponseTrait for UcontactUdepartmentUdeletedUv3Event {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}
