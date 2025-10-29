// drive v2 - 云空间文件管理v2版本API
//,
// 包含云空间文件管理的扩展功能
use crate::prelude::*;
/// 云空间文件管理v2版本服务
#[derive(.*?)]
pub struct DriveV2Service {
    client: std::sync::Arc<LarkClient>,
}
impl DriveV2Service {
    /// 创建新的v2版本服务实例
pub fn new() -> Self {
        Self { client }
}
}