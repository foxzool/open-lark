// sheets - 电子表格服务
//
// 提供电子表格相关的所有功能，包括：
// - 表格的创建、读取、更新、删除
// - 单元格操作（读写、样式设置、数据验证）
// - 工作表管理
// - 数据处理（导入、导出、筛选、排序）
// - 图表和图片操作
// - 条件格式和保护范围
//
// 覆盖60个API接口（sheets/v2: 33个，sheets/v3: 27个）

use crate::prelude::*;
use crate::service::ccm::sheets::v2::SheetsV2Service;
use crate::service::ccm::sheets::v3::SheetsV3Service;

/// 电子表格服务
#[derive(Debug, Clone)]
pub struct SheetsService {
    /// v2版本API服务
    pub v2: SheetsV2Service,
    /// v3版本API服务
    pub v3: SheetsV3Service,
}

impl SheetsService {
    /// 创建新的电子表格服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            v2: SheetsV2Service::new(client.clone()),
            v3: SheetsV3Service::new(client.clone()),
        }
    }
}

/// v2版本API
pub mod v2;
/// v3版本API
pub mod v3;