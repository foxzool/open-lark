/// 工作表批量更新服务
///
/// 提供SHEETS v2工作表批量更新功能，支持：
/// - 批量更新工作表属性
/// - 添加和删除工作表
/// - 修改工作表标题和索引
/// - 设置工作表颜色和隐藏状态
/// - 调整工作表顺序
use serde_json::Value;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
