/// Sheets v2 单个范围写入服务
///
/// 提供飞书电子表格v2版本的单个范围数据写入功能，包括：
/// - 向指定范围写入数据，支持多种数据格式
/// - 灵活的值输入选项（原始值、用户输入格式、公式）
/// - 智能数据类型识别和转换
/// - 写入结果详细信息，包括更新范围和单元格数量
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
