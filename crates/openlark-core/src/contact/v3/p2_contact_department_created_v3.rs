use crate::{
    api::ApiResponseTrait, config::Config,
};
use serde::{Deserialize, Serialize};

/// Department创建事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcontactUdepartmentUcreatedUv3Event {
    /// 事件ID
    pub event_id: String,
    /// Department ID
    pub department_id: String,
    /// 事件时间戳
    pub event_time: i64,
}

impl ApiResponseTrait for UcontactUdepartmentUcreatedUv3Event {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}
