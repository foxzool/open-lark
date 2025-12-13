/// 工作表管理服务 v2
///
/// 提供飞书电子表格v2版本的工作表管理功能，包括：
/// - 批量更新工作表属性
/// - 工作表重命名和属性修改
/// - 工作表隐藏/显示操作
/// - 工作表索引调整
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
