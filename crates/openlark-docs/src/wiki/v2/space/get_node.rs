/// 获取知识空间节点信息
///
/// 根据节点Token获取知识空间节点的详细信息。
/// docPath: https://open.feishu.cn/document/server-docs/wiki-v2/space/get_node
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间节点信息请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpaceNodeRequest {
    /// 空间ID
    pub space_id: String,
    /// 节点Token
    pub node_token: String,
}

/// 获取知识空间节点信息响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetSpaceNodeResponse {
    /// 节点Token
    pub node_token: String,
    /// 节点类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    /// 节点标题
    pub title: String,
    /// 父节点Token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 空间ID
    pub space_id: String,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 子节点数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_node_count: Option<i32>,
}

impl ApiResponseTrait for GetSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间节点信息
///
/// 根据节点Token获取知识空间节点的详细信息。
///
/// # 参数
/// * `config` - SDK 配置
/// * `request` - 获取节点信息请求
///
/// # 返回
/// 返回知识空间节点详细信息
pub async fn get_space_node(
    config: &Config,
    request: GetSpaceNodeRequest,
) -> SDKResult<GetSpaceNodeResponse> {
    get_space_node_with_options(config, request, RequestOption::default()).await
}

/// 获取知识空间节点信息（支持自定义选项）
///
/// 根据节点Token获取知识空间节点的详细信息。
///
/// # 参数
/// * `config` - SDK 配置
/// * `request` - 获取节点信息请求
/// * `option` - 请求选项
///
/// # 返回
/// 返回知识空间节点详细信息
pub async fn get_space_node_with_options(
    config: &Config,
    request: GetSpaceNodeRequest,
    option: RequestOption,
) -> SDKResult<GetSpaceNodeResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceGetNode;

    // 创建 API 请求，通过 query 参数传递 space_id 和 node_token
    let mut api_request: ApiRequest<GetSpaceNodeResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query("space_id", &request.space_id)
            .query("node_token", &request.node_token);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
