/// 移动知识空间节点
///
/// 在知识空间中移动节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/move
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 移动知识空间节点请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveSpaceNodeRequest {
    /// 目标父节点token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

/// 移动知识空间节点响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveSpaceNodeResponse {
    /// 节点token
    pub node_token: String,
    /// 是否成功
    pub success: bool,
}

impl ApiResponseTrait for MoveSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动知识空间节点
///
/// 在知识空间中移动节点。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `node_token` - 节点token
/// * `request` - 移动节点请求
///
/// # 返回
/// 返回移动结果
pub async fn move_space_node(
    config: &Config,
    space_id: &str,
    node_token: &str,
    request: MoveSpaceNodeRequest,
) -> SDKResult<MoveSpaceNodeResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeMove(space_id.to_string(), node_token.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<MoveSpaceNodeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, Some(option)).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
