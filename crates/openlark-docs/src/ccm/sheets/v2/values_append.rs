use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 数据追加服务
///
/// 提供电子表格数据追加功能，包括：
/// - 向指定范围之前插入数据行
/// - 遇到空行时覆盖或追加数据行
/// - 大数据量的高效追加操作
/// - 自动行检测和智能追加
use serde_json::Value;
