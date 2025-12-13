use serde::{Deserialize, Serialize};
use serde_json::Value;
/// 表格元数据服务
///
/// 提供飞书电子表格v2版本的表格元数据获取功能，包括：
/// - 获取电子表格的基本信息
/// - 查询工作表列表和属性
/// - 获取电子表格权限信息
/// - 查询创建和修改时间
/// - 获取电子表格版本和状态信息
use std::collections::HashMap;

use openlark_core::endpoints::Endpoints;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
