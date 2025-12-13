#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// 向单个范围写入数据 - Single Range Write Service
///
/// 提供飞书电子表格v2版本的单个范围数据写入API。
/// 支持向指定工作表的特定范围写入数据，包括：
/// - 单元格值写入和格式化
/// - 数据类型自动转换和验证
/// - 写入结果状态和详细信息
/// - 错误处理和重试机制
/// - 多种数据格式支持（数字、文本、布尔值、公式等）
use serde_json::Value;
use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::endpoints::Endpoints;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
