/// Sheets v2 保护范围管理服务
///
/// 提供飞书电子表格v2版本的保护范围管理功能，支持：
/// - 批量创建保护范围
/// - 批量修改保护范围
/// - 批量获取保护范围信息
/// - 批量删除保护范围
/// - 权限验证和安全机制
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
