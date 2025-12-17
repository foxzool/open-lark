/// CCM Sheet API 模块
///
/// 电子表格操作API实现，包含全面的表格功能：
///
/// ## 数据读写API (8个)
/// - read_single_range: 读取单个范围
/// - read_multiple_ranges: 读取多个范围
/// - write_single_range: 写入单个范围
/// - batch_write_ranges: 批量写入多个范围
/// - append_values: 追加数据
/// - insert_values: 插入数据
/// - values_image: 写入图片
///
/// ## 表格操作API (7个)
/// - delete_range: 删除范围
/// - insert_dimension: 插入行列
/// - move_dimension: 移动行列
/// - replace_range: 替换范围
/// - find_replace: 查找替换
/// - merge_cells: 合并单元格
/// - unmerge_cells: 取消合并单元格
///
/// ## 筛选功能API (4个)
/// - create_filter: 创建筛选
/// - get_filter: 获取筛选
/// - update_filter: 更新筛选
/// - delete_filter: 删除筛选
///
/// ## 浮图功能API (4个)
/// - create_float_image: 创建浮图
/// - get_float_image: 获取浮图
/// - update_float_image: 更新浮图
/// - delete_float_image: 删除浮图
///
/// ## 表格基础API (3个)
/// - get_spreadsheet: 获取表格信息
/// - create_spreadsheet: 创建表格
/// - update_spreadsheet: 更新表格
///
/// ## 工作表API (4个)
/// - add_sheet: 添加工作表
/// - get_sheet: 获取工作表信息
/// - update_sheet: 更新工作表
/// - delete_sheet: 删除工作表
use openlark_core::config::Config;

/// CCM Sheet 服务
#[derive(Debug, Clone)]
pub struct CcmSheetService {
    config: Config,
}

impl CcmSheetService {
    /// 创建新的Ccm Sheet服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取V2版本API
    pub fn v2(&self) -> CcmSheetV2 {
        CcmSheetV2::new(self.config.clone())
    }

    /// 获取旧版版本API（兼容性保留）
    pub fn old(&self) -> crate::ccm::ccm_sheet::old::CcmSheetOldService {
        crate::ccm::ccm_sheet::old::CcmSheetOldService::new(self.config.clone())
    }
}

/// CCM Sheet V2 API访问器
#[derive(Debug, Clone)]
pub struct CcmSheetV2 {
    config: Config,
}

impl CcmSheetV2 {
    /// 创建新的V2 API访问器实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

/// Old版本API (v2)
pub mod old;

/// V2版本API
pub mod v2;

// 重新导出主要类型
pub use models::*;
pub use old::v2::CcmSheetOldV2;
