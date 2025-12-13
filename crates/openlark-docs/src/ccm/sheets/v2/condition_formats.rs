/// Sheets v2 条件格式管理服务
///
/// 提供飞书电子表格v2版本的条件格式管理功能，支持：
/// - 批量创建条件格式
/// - 批量更新条件格式
/// - 批量获取条件格式
/// - 批量删除条件格式
/// - 多种条件格式类型（数据条、色阶、图标集等）
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
