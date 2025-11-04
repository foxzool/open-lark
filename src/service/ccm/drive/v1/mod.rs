#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
// drive v1 - 云空间文件管理v1版本API
//,
// 包含云空间文件管理的核心功能，这是使用最广泛的版本
use crate::prelude::*;
use crate::service::ccm::drive::v1::file::FileService;
use crate::service::ccm::drive::v1::meta::MetaService;
use crate::service::ccm::drive::v1::statistics::StatisticsService;
use crate::service::ccm::drive::v1::view_record::ViewRecordService;
/// 云空间文件管理v1版本服务
#[derive(Debug, Clone)]
pub struct DriveV1Service {
    client: std::sync::Arc<LarkClient>,
    /// 文件操作服务
    pub file: FileService,
    /// 元数据服务
    pub meta: MetaService,
    /// 统计服务
    pub statistics: StatisticsService,
    /// 访问记录服务
pub view_record: ViewRecordService,
}
impl DriveV1Service {
pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            client,
            file: FileService::new(),
            meta: MetaService::new(),
            statistics: StatisticsService::new(),
            view_record: ViewRecordService::new(),
}
/// 文件操作服务
pub mod file;
/// 元数据服务
pub mod meta;
/// 统计服务
pub mod statistics;
/// 访问记录服务
pub mod view_record;
}
}