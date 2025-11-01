// drive explorer - 资源浏览器API
//,
// 提供云空间资源浏览器相关的功能
use crate::prelude::*;
/// 资源浏览器服务
#[derive(Debug, Clone)]
pub struct DriveExplorerService {
    client: std::sync::Arc<LarkClient>,
}
impl DriveExplorerService {
    pub fn new(config: Config) -> Self {
        Self { config }
}
}