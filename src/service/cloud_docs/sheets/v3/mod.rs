use std::sync::Arc;

use crate::core::config::Config;

/// 条件格式化功能
pub mod condition_format;
/// 数据操作功能
pub mod data_operation;
/// 数据验证功能
pub mod data_validation;
/// 浮动图片功能
pub mod float_image;
/// 保护区域功能
pub mod protect_range;
/// 行列操作功能
pub mod sheet_row_col;
/// 电子表格功能
pub mod spreadsheet;
/// 工作表功能
pub mod spreadsheet_sheet;
/// 工作表筛选功能
pub mod spreadsheet_sheet_filter;
/// 工作表筛选视图功能
pub mod spreadsheet_sheet_filter_view;
/// 工作表筛选视图条件功能
pub mod spreadsheet_sheet_filter_view_condition;

/// Sheets API v3版本服务集合
///
/// 提供电子表格的完整功能集，包括数据操作、格式设置、筛选、保护等高级特性。
/// v3版本相比v2版本提供了更丰富的功能和更好的性能。
///
/// # 主要功能模块
///
/// - **spreadsheet**: 电子表格的创建、删除、获取等基础操作
/// - **spreadsheet_sheet**: 工作表的管理和操作
/// - **data_operation**: 数据的读写、查找、替换等操作
/// - **sheet_row_col**: 行列的插入、删除、调整等操作
/// - **spreadsheet_sheet_filter**: 工作表筛选功能
/// - **spreadsheet_sheet_filter_view**: 筛选视图管理
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
/// // 使用v3版本API
/// let sheets_v3 = &client.sheets.v3;
///
/// // 创建电子表格
/// // let create_req = CreateSpreadsheetRequest::builder()
/// //     .title("数据统计表")
/// //     .build();
/// // let response = sheets_v3.spreadsheet.create(create_req, None).await?;
/// ```
pub struct V3 {
    /// 电子表格服务 - 管理电子表格的生命周期
    pub spreadsheet: SpreadsheetService,
    /// 工作表服务 - 管理工作表的操作
    pub spreadsheet_sheet: SpreadsheetSheetService,
    /// 工作表筛选服务 - 提供数据筛选功能
    pub spreadsheet_sheet_filter: SpreadsheetSheetFilterService,
    /// 工作表筛选视图服务 - 管理筛选视图
    pub spreadsheet_sheet_filter_view: SpreadsheetSheetFilterViewService,
    /// 数据操作服务 - 处理数据的读写和操作
    pub data_operation: DataOperationService,
    /// 行列操作服务 - 管理行列的结构
    pub sheet_row_col: SheetRowColService,
}

impl V3 {
    /// 创建新的V3版本服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的V3服务实例，包含所有子服务模块
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            spreadsheet: SpreadsheetService::new(Arc::clone(&config)),
            spreadsheet_sheet: SpreadsheetSheetService::new(Arc::clone(&config)),
            spreadsheet_sheet_filter: SpreadsheetSheetFilterService::new(Arc::clone(&config)),
            spreadsheet_sheet_filter_view: SpreadsheetSheetFilterViewService::new(Arc::clone(&config)),
            data_operation: DataOperationService::new(Arc::clone(&config)),
            sheet_row_col: SheetRowColService::new(config),
        }
    }
}

/// 电子表格服务
///
/// 提供电子表格的完整生命周期管理，包括创建、获取、更新、删除等操作。
/// 支持表格属性设置、权限管理和元数据操作。
///
/// # 主要功能
///
/// - 📊 创建新的电子表格
/// - 📋 获取表格基本信息和元数据
/// - ✏️ 更新表格属性（标题、描述等）
/// - 🗑️ 删除电子表格
/// - 🔐 管理表格访问权限
/// - 📁 设置表格所属文件夹
///
/// # 使用场景
///
/// - 自动化报表生成
/// - 数据导入导出工具
/// - 批量表格管理
/// - 权限控制系统
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

/// 工作表服务
///
/// 管理电子表格内的工作表（Sheet），支持工作表的创建、删除、重命名、
/// 复制等操作，以及工作表属性的设置。
///
/// # 主要功能
///
/// - 📄 创建新工作表
/// - 📝 重命名工作表
/// - 📋 复制工作表
/// - 🗑️ 删除工作表
/// - 🎨 设置工作表属性（颜色、保护等）
/// - 📐 调整工作表大小
/// - 🔄 移动工作表位置
///
/// # 使用场景
///
/// - 多Sheet数据分类管理
/// - 模板工作表复制
/// - 动态工作表结构调整
/// - 工作表权限控制
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    /// 创建新的工作表服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表筛选服务
///
/// 提供工作表的数据筛选功能，支持创建、管理和应用筛选条件。
/// 帮助用户快速定位和查看符合条件的数据。
///
/// # 主要功能
///
/// - 🔍 创建数据筛选器
/// - ⚙️ 设置筛选条件
/// - 🔄 更新筛选规则
/// - 🗑️ 删除筛选器
/// - 📋 获取筛选状态
/// - 🎯 应用多条件筛选
///
/// # 筛选类型
///
/// - 数值筛选（大于、小于、等于等）
/// - 文本筛选（包含、开始于、结束于等）
/// - 日期筛选（日期范围、相对日期等）
/// - 自定义筛选（正则表达式等）
///
/// # 使用场景
///
/// - 数据分析和报表
/// - 大量数据快速筛选
/// - 动态数据视图
/// - 条件数据导出
pub struct SpreadsheetSheetFilterService {
    config: Config,
}

impl SpreadsheetSheetFilterService {
    /// 创建新的工作表筛选服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 工作表筛选视图服务
///
/// 管理工作表的筛选视图，允许用户创建、保存和切换不同的筛选配置。
/// 筛选视图是保存的筛选器配置，可以快速应用和分享。
///
/// # 主要功能
///
/// - 👁️ 创建筛选视图
/// - 💾 保存筛选配置
/// - 🔄 切换筛选视图
/// - 📝 重命名筛选视图
/// - 🗑️ 删除筛选视图
/// - 👥 分享筛选视图
/// - 📋 获取视图列表
///
/// # 视图特性
///
/// - 保存完整的筛选条件
/// - 包含排序规则
/// - 支持隐藏列配置
/// - 可设置视图权限
/// - 支持视图协作
///
/// # 使用场景
///
/// - 团队协作数据分析
/// - 多维度数据查看
/// - 数据视图模板化
/// - 复杂筛选条件保存
pub struct SpreadsheetSheetFilterViewService {
    config: Config,
}

impl SpreadsheetSheetFilterViewService {
    /// 创建新的工作表筛选视图服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 数据操作服务
///
/// 提供电子表格数据的核心操作功能，包括数据读写、查找替换、
/// 样式设置、单元格合并等全面的数据处理能力。
///
/// # 主要功能
///
/// ## 数据读写
/// - 📖 读取单个或多个范围数据
/// - ✏️ 写入数据到指定范围
/// - ➕ 在末尾追加数据
/// - ⬆️ 在开头插入数据
///
/// ## 数据操作
/// - 🔍 查找和替换数据
/// - 🔗 合并和拆分单元格
/// - 🎨 设置单元格样式
/// - 🖼️ 插入图片和浮动图片
///
/// ## 批量操作
/// - 📊 批量写入多个范围
/// - 🎨 批量设置样式
/// - 🔄 批量数据转换
///
/// # 支持的数据类型
///
/// - 文本和数字
/// - 日期和时间
/// - 布尔值
/// - 公式
/// - 超链接
/// - 图片
///
/// # 使用场景
///
/// - 数据导入导出
/// - 报表自动生成
/// - 数据清洗和处理
/// - 批量格式设置
/// - 图表数据准备
pub struct DataOperationService {
    config: Config,
}

impl DataOperationService {
    /// 创建新的数据操作服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 行列操作服务
///
/// 管理工作表的行列结构，支持插入、删除、调整行列大小等操作。
/// 提供灵活的表格布局控制能力。
///
/// # 主要功能
///
/// ## 行操作
/// - ➕ 插入新行
/// - 🗑️ 删除指定行
/// - 📏 调整行高
/// - 👁️ 隐藏/显示行
/// - 🔒 冻结行
///
/// ## 列操作
/// - ➕ 插入新列
/// - 🗑️ 删除指定列
/// - 📐 调整列宽
/// - 👁️ 隐藏/显示列
/// - 🔒 冻结列
///
/// ## 批量操作
/// - 📊 批量调整尺寸
/// - 🔄 批量隐藏/显示
/// - ➕ 批量插入
/// - 🗑️ 批量删除
///
/// # 使用场景
///
/// - 动态表格结构调整
/// - 数据展示优化
/// - 表格模板创建
/// - 大数据表格管理
/// - 用户界面布局控制
pub struct SheetRowColService {
    #[allow(dead_code)]
    config: Config,
}

impl SheetRowColService {
    /// 创建新的行列操作服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
