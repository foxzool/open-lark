#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]

/// Sheets v2 图片写入服务
///
/// 提供飞书电子表格v2版本的图片写入功能，包括：
/// - 向电子表格中插入图片
/// - 灵活的图片位置和尺寸设置
/// - Excel风格的单元格引用定位
/// - 企业级错误处理和数据验证
/// - 多种图片定位方式支持
// use futures_util::future;
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
