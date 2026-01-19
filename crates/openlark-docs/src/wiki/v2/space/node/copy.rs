/// 创建知识空间节点副本
///
/// 创建知识空间节点的副本。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/copy
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 创建知识空间节点副本请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySpaceNodeRequest {
    /// 新节点标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 父节点token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

/// 创建知识空间节点副本响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CopySpaceNodeResponse {
    /// 节点token
    pub node_token: String,
    /// 节点标题
    pub title: String,
}

impl ApiResponseTrait for CopySpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间节点副本
///
/// 创建知识空间节点的副本。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `node_token` - 节点token
/// * `request` - 复制节点请求
///
/// # 返回
/// 返回复制的节点信息
pub async fn copy_space_node(
    config: &Config,
    space_id: &str,
    node_token: &str,
    request: CopySpaceNodeRequest,
) -> SDKResult<CopySpaceNodeResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeCopy(space_id.to_string(), node_token.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<CopySpaceNodeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
