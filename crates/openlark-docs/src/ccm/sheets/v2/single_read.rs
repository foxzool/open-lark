#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// Sheets v2 单个范围读取服务
///
/// 提供飞书电子表格v2版本的单个范围读取功能，包括：
/// - 读取单个单元格范围的数据
/// - 支持Excel风格的范围格式
/// - 灵活的数据渲染选项
/// - 企业级错误处理和数据验证
use serde_json::Value;

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
