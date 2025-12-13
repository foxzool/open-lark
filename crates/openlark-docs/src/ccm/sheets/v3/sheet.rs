/// 工作表管理服务 v3
///
/// 提供飞书电子表格v3版本的工作表管理功能，包括：
/// - 获取电子表格下的所有工作表
/// - 查询特定工作表的属性信息
/// - 工作表属性管理和操作
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
