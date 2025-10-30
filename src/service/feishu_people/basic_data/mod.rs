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
#[derive(Debug, Clone)]
pub struct BasicDataService {
}

impl BasicDataService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}
/// v1版本API
pub mod v1;
}