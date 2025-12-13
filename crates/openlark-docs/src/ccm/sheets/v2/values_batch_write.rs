/// Sheets v2 批量范围写入服务
///
/// 提供飞书电子表格v2版本的批量范围写入功能，包括：
/// - 一次性向多个单元格范围写入数据，提高效率
/// - 支持Excel风格的范围格式（如 Sheet1!A1:B10）
/// - 灵活的值输入选项和数据解析策略
/// - 智能的数据类型识别和转换
/// - 详细的写入结果，包括每个范围的状态
/// - 企业级错误处理和数据验证
/// - 构建器模式支持，提供流畅API设计
use serde_json::Value;
use std::collections::HashMap;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
