/// 单元格合并服务
///
/// 提供飞书电子表格v2版本的单元格合并功能，包括：
/// - 合并指定范围的单元格
/// - 取消单元格合并
/// - 查询合并状态
/// - 支持水平、垂直和矩形区域合并
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
