/// Sheets API 标准模板
///
/// 使用 rust-api-mapper 工具可识别的标准模板格式
/// 基于 metainfo.rs 的现代实现模式
use serde::{Deserialize, Serialize};

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
