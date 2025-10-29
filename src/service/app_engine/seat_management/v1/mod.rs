// seat_management v1 - 席位管理v1版本API
//
// 包含席位管理的完整功能

use crate::prelude::*;

/// 席位管理v1版本服务
#[derive(Debug, Clone)]
pub struct SeatManagementV1Service {
    client: std::sync::Arc<crate::client::LarkClient>,
}

impl SeatManagementV1Service {
    pub fn new(client: std::sync::Arc<crate::client::LarkClient>) -> Self {
        Self { client }
    }
}