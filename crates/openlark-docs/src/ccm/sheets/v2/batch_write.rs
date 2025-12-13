#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// Sheets v2 批量范围写入服务
///
/// 提供飞书电子表格v2版本的批量范围写入功能，包括：
/// - 一次性向多个单元格范围写入数据
/// - 支持Excel风格的范围格式
/// - 高效的批量数据更新
/// - 企业级错误处理和数据验证
/// - 多种数据类型支持（文本、数字、公式、布尔值等）
use serde_json::Value;
use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use openlark_core::endpoints::Endpoints;
use openlark_core::impl_executable_builder_owned;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
