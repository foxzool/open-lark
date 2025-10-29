// leaves - 假期管理服务
//,
// 提供假期管理相关的所有功能，包括：
// - 假期类型管理
// - 假期余额查询
// - 休假申请管理
// - 假期授予记录管理
use crate::prelude::*;
use crate::service::feishu_people::leaves::v1::LeavesV1Service;
/// 假期管理服务
#[derive(.*?)]
pub struct LeavesService {
    /// v1版本API服务
    pub v1: LeavesV1Service,
}
impl LeavesService {
    /// 创建新的假期管理服务实例
pub fn new() -> Self {
        Self {
            v1: LeavesV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;