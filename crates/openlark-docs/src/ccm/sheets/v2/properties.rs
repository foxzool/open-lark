/// Sheets v2 表格属性管理服务
///
/// 提供飞书电子表格v2版本的表格属性管理功能，支持：
/// - 更新表格标题
/// - 修改表格描述
/// - 更新表格其他属性
/// - 属性变更同步
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
