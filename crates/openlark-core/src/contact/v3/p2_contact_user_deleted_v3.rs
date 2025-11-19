use crate::{
    api::ApiResponseTrait, config::Config,
};
use serde::{Deserialize, Serialize};

/// User删除事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UcontactUuserUdeletedUv3Event {
    /// 事件ID
    pub event_id: String,
    /// User ID
    pub user_id: String,
    /// 事件时间戳
    pub event_time: i64,
}

impl ApiResponseTrait for UcontactUuserUdeletedUv3Event {
    fn data_format() -> crate::api::ResponseFormat {
        crate::api::ResponseFormat::Data
    }
}
