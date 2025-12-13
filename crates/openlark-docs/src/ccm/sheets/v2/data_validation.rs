use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 数据验证服务
///
/// 提供飞书电子表格v2版本的数据验证功能，包括：
/// - 设置下拉列表验证
/// - 更新数据验证规则
/// - 查询数据验证设置
/// - 删除数据验证规则
/// - 支持多种验证类型和条件
use serde_json::Value;
