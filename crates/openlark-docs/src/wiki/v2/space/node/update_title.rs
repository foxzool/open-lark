/// 更新知识空间节点标题
///
/// 更新知识空间节点的标题。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/update_title
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 更新知识空间节点标题请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpaceNodeTitleRequest {
    /// 新标题
    pub title: String,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpaceNodeTitleResponse {
    /// 节点token
    pub node_token: String,
    /// 节点标题
    pub title: String,
}

impl ApiResponseTrait for UpdateSpaceNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间节点标题
///
/// 更新知识空间节点的标题。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `node_token` - 节点token
/// * `request` - 更新标题请求
///
/// # 返回
/// 返回更新后的节点信息
pub async fn update_space_node_title(
    config: &Config,
    space_id: &str,
    node_token: &str,
    request: UpdateSpaceNodeTitleRequest,
) -> SDKResult<UpdateSpaceNodeTitleResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeUpdateTitle(space_id.to_string(), node_token.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<UpdateSpaceNodeTitleResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
