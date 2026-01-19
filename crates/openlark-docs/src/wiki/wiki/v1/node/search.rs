//! 搜索Wiki节点
//!
//! docPath: https://open.feishu.cn/document/server-docs/wiki-v2/wiki-node/search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV1;

/// 搜索Wiki节点请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchNodeRequest {
    /// 查询关键字
    pub query: String,
}

/// 搜索Wiki节点响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchNodeResponse {
    /// 节点列表
    #[serde(default)]
    pub items: Vec<NodeItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SearchNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 节点项
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NodeItem {
    /// 节点Token
    pub node_token: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 父节点Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

/// 搜索Wiki节点
///
/// 根据关键字搜索Wiki节点。
///
/// # 参数
/// * `config` - SDK 配置
/// * `request` - 搜索请求
///
/// # 返回
/// 返回搜索到的节点列表
pub async fn search_node(
    config: &Config,
    request: SearchNodeRequest,
) -> SDKResult<SearchNodeResponse> {
    search_node_with_options(config, request, RequestOption::default()).await
}

/// 搜索Wiki节点（支持自定义选项）
///
/// 根据关键字搜索Wiki节点。
///
/// # 参数
/// * `config` - SDK 配置
/// * `request` - 搜索请求
/// * `option` - 请求选项
///
/// # 返回
/// 返回搜索到的节点列表
pub async fn search_node_with_options(
    config: &Config,
    request: SearchNodeRequest,
    option: RequestOption,
) -> SDKResult<SearchNodeResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV1::NodeSearch;

    // 创建 API 请求
    let api_request: ApiRequest<SearchNodeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
