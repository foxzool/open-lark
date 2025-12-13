/// 单个范围读取服务
///
/// 提供飞书电子表格v2版本的单个范围数据读取功能，包括：
/// - 读取指定范围的单元格数据
/// - 支持多种数据格式和渲染选项
/// - 提供灵活的范围查询和过滤功能
/// - 高效的数据获取和解析
use serde_json::Value;

use serde::{Deserialize, Serialize};

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
