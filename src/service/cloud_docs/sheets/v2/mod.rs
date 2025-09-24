use crate::core::config::Config;

/// 数据操作功能
pub mod data_operation;
/// 行列操作功能
pub mod sheet_row_col;
/// 工作表功能
pub mod spreadsheet_sheet;

/// Sheets API v2版本服务集合
///
/// 提供电子表格的基础功能，包括数据读写、工作表管理等核心操作。
/// v2版本是稳定的基础API版本，适用于大多数常见的电子表格操作需求。
///
/// # 主要功能模块
///
/// - **spreadsheet**: 电子表格的基础管理功能
/// - **spreadsheet_sheet**: 工作表的创建、删除和基础操作
/// - **data_operation**: 数据的读写和基本操作（通过子模块实现）
/// - **sheet_row_col**: 行列的基础结构操作（通过子模块实现）
///
/// # API版本对比
///
/// v2版本相比v3版本功能较为基础，但具有以下优势：
/// - 🔧 API稳定，向后兼容性好
/// - ⚡ 响应速度快，资源占用少
/// - 🎯 功能专注，适合简单场景
/// - 📖 文档完善，示例丰富
///
/// # 使用示例
///
/// ```rust
/// use open_lark::prelude::*;
///
/// let client = LarkClient::builder("app_id", "app_secret")
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 使用v2版本API
/// let sheets_v2 = &client.sheets.v2;
///
/// // 基础工作表操作
/// // let sheet_ops = &sheets_v2.spreadsheet_sheet;
/// ```
///
/// # 适用场景
///
/// - 简单的数据读写操作
/// - 基础的工作表管理
/// - 对API稳定性要求高的场景
/// - 轻量级集成需求
pub struct V2 {
    /// 电子表格服务 - 提供表格级别的基础操作
    pub spreadsheet: SpreadsheetService,
    /// 工作表服务 - 管理工作表的基础功能
    pub spreadsheet_sheet: SpreadsheetSheetService,
}

impl V2 {
    /// 创建新的V2版本服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的V2服务实例，包含基础的服务模块
    pub fn new(config: Config) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(config.clone()),
            spreadsheet_sheet: SpreadsheetSheetService::new(config),
        }
    }
}

/// 电子表格服务（V2版本）
///
/// 提供电子表格的基础管理功能，包括表格的创建、获取和基本属性设置。
/// V2版本专注于核心功能，提供稳定可靠的API接口。
///
/// # 主要功能
///
/// - 📊 创建新的电子表格
/// - 📋 获取表格基本信息
/// - ✏️ 更新表格基础属性
/// - 🔍 查询表格状态
///
/// # 功能特点
///
/// - ✅ API稳定，兼容性强
/// - 🚀 响应快速，性能可靠
/// - 🎯 功能明确，易于使用
/// - 📚 文档完善，示例丰富
///
/// # 使用场景
///
/// - 基础表格管理系统
/// - 简单数据导入导出
/// - 轻量级报表生成
/// - 快速原型开发
///
/// # 与V3版本对比
///
/// V2版本更加简洁，适合基础需求；V3版本功能更强大，适合复杂场景。
/// 建议根据实际需求选择合适的版本。
pub struct SpreadsheetService {
    config: Config,
}

impl SpreadsheetService {
    /// 创建新的电子表格服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表服务（V2版本）
///
/// 管理电子表格内工作表的基础功能，支持工作表的创建、删除、
/// 重命名等常用操作。提供稳定可靠的工作表管理接口。
///
/// # 主要功能
///
/// - 📄 创建新工作表
/// - 📝 重命名工作表
/// - 🗑️ 删除工作表
/// - 📋 获取工作表信息
/// - 🎨 设置基础属性
///
/// # 功能特色
///
/// - 🔧 操作简单直观
/// - ⚡ 执行效率高
/// - 🛡️ 错误处理完善
/// - 📖 使用方式清晰
///
/// # 使用场景
///
/// - 多工作表数据管理
/// - 基础表格结构搭建
/// - 简单的工作表模板
/// - 数据分类整理
///
/// # 最佳实践
///
/// - 合理规划工作表结构
/// - 使用有意义的工作表名称
/// - 避免频繁的工作表操作
/// - 及时清理不需要的工作表
pub struct SpreadsheetSheetService {
    // 向后兼容：保留原始 Config 字段，便于现有测试/示例访问
    #[allow(dead_code)] // Backward compatibility field for tests/examples
    config: Config,
    // 试点：共享配置引用，后续逐步替换内部使用以降低 clone
    config_arc: Arc<Config>,
}

impl SpreadsheetSheetService {
    /// 创建新的工作表服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        let config_arc = Arc::new(config.clone());
        Self { config, config_arc }
    }
}

impl SpreadsheetSheetService {
    /// 获取共享配置的引用计数指针（试点接口）
    pub fn config_shared(&self) -> Arc<Config> {
        self.config_arc.clone()
    }
}
use std::sync::Arc;
