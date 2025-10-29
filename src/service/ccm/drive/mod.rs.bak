// drive - 云空间文件管理服务
//,
// 提供云空间文件管理相关的所有功能，包括：
// - 文件和文件夹的CRUD操作
// - 文件上传和下载
// - 文件复制和移动
// - 文件权限管理
// - 文件搜索和统计
//
// 覆盖55个API接口
use crate::prelude::*;
use crate::service::ccm::drive::v1::DriveV1Service;
use crate::service::ccm::drive::v2::DriveV2Service;
use crate::service::ccm::drive::explorer::DriveExplorerService;
/// 云空间文件管理服务
#[derive(.*?)]
pub struct DriveService {
    /// v1版本API服务
    pub v1: DriveV1Service,
    /// v2版本API服务
    pub v2: DriveV2Service,
    /// 资源浏览器服务
    pub explorer: DriveExplorerService,
}
impl DriveService {
    /// 创建新的云空间文件管理服务实例
pub fn new() -> Self {
        Self {
            v1: DriveV1Service::new(client.clone()),
            v2: DriveV2Service::new(client.clone()),
            explorer: DriveExplorerService::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;
/// v2版本API
pub mod v2;
/// 资源浏览器API
pub mod explorer;