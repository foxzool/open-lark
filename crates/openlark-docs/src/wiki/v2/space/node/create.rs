/// 创建知识空间节点
///
/// 在指定知识空间创建新节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 创建知识空间节点请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSpaceNodeRequest {
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
    /// 源节点token（用于复制）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_node_token: Option<String>,
}

/// 创建知识空间节点响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateSpaceNodeResponse {
    /// 节点token
    pub node_token: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
}

impl ApiResponseTrait for CreateSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间节点
///
/// 在指定知识空间创建新节点。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `request` - 创建节点请求
///
/// # 返回
/// 返回创建的节点信息
pub async fn create_space_node(
    config: &Config,
    space_id: &str,
    request: CreateSpaceNodeRequest,
) -> SDKResult<CreateSpaceNodeResponse> {
    create_space_node_with_options(config, space_id, request, RequestOption::default()).await
}

/// 创建知识空间节点（支持自定义选项）
pub async fn create_space_node_with_options(
    config: &Config,
    space_id: &str,
    request: CreateSpaceNodeRequest,
    option: RequestOption,
) -> SDKResult<CreateSpaceNodeResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeCreate(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<CreateSpaceNodeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
