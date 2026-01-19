/// 获取知识空间子节点列表
///
/// 获取指定知识空间的子节点列表。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间子节点列表响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListSpaceNodesResponse {
    /// 节点列表
    pub items: Vec<NodeItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 节点信息
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NodeItem {
    /// 节点token
    pub node_token: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

impl ApiResponseTrait for ListSpaceNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间子节点列表
///
/// 获取指定知识空间的子节点列表。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
///
/// # 返回
/// 返回节点列表
pub async fn list_space_nodes(
    config: &Config,
    space_id: &str,
) -> SDKResult<ListSpaceNodesResponse> {
    list_space_nodes_with_options(config, space_id, RequestOption::default()).await
}

/// 获取知识空间子节点列表（支持自定义选项）
///
/// 获取指定知识空间的子节点列表。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `option` - 请求选项
///
/// # 返回
/// 返回节点列表
pub async fn list_space_nodes_with_options(
    config: &Config,
    space_id: &str,
    option: RequestOption,
) -> SDKResult<ListSpaceNodesResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeList(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<ListSpaceNodesResponse> = ApiRequest::get(&api_endpoint.to_url());

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
