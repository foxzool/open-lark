#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// Sheets v2 批量范围读取服务
///
/// 提供飞书电子表格v2版本的批量范围读取功能，包括：
/// - 一次性读取多个单元格范围
/// - 支持Excel风格的范围格式
/// - 高效的批量数据获取
/// - 企业级错误处理和数据验证
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use openlark_core::endpoints::Endpoints;
use openlark_core::impl_executable_builder_owned;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
