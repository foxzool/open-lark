// basic_data - 基础数据管理服务
//,
// 提供基础数据管理相关的所有功能，包括：
// - 人员类型管理
// - 工时制度管理
// - 地点管理
// - 国家证件类型管理
// - 自定义字段管理
use crate::prelude::*;
use crate::service::feishu_people::basic_data::v1::BasicDataV1Service;
/// 基础数据管理服务
#[derive(.*?)]
pub struct BasicDataService {
    /// v1版本API服务
    pub v1: BasicDataV1Service,
}
impl BasicDataService {
    /// 创建新的基础数据管理服务实例
pub fn new() -> Self {
        Self {
            v1: BasicDataV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;