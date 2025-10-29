// seat_management - 席位管理服务
//
// 提供席位管理相关的功能

use crate::prelude::*;
use crate::service::app_engine::seat_management::v1::SeatManagementV1Service;

/// 席位管理服务
#[derive(Debug, Clone)]
pub struct SeatManagementService {
    /// v1版本API服务
    pub v1: SeatManagementV1Service,
}

impl SeatManagementService {
    /// 创建新的席位管理服务实例
    pub fn new(client: std::sync::Arc<crate::client::LarkClient>) -> Self {
        Self {
            v1: SeatManagementV1Service::new(client.clone()),
        }
    }
}

/// v1版本API
pub mod v1;