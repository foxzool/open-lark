#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// seat_management - 席位管理服务
//,
// 提供席位管理相关的功能
use crate::prelude::*;
use crate::service::app_engine::seat_management::v1::SeatManagementV1Service;
/// 席位管理服务
#[derive(Debug, Clone)]
pub struct SeatManagementService {
}

impl SeatManagementService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}