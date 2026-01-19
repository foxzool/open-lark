/// 移动云空间文档至知识空间
///
/// 将云空间的文档移动到知识空间。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};
use crate::common::api_endpoints::WikiApiV2;

/// 移动云空间文档至知识空间请求
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveDocsToWikiRequest {
    /// 文档token列表
    pub obj_tokens: Vec<String>,
    /// 父节点token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_node_token: Option<String>,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoveDocsToWikiResponse {
    /// 任务ID
    pub task_id: String,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动云空间文档至知识空间
///
/// 将云空间的文档移动到知识空间。
///
/// # 参数
/// * `config` - SDK 配置
/// * `space_id` - 空间ID
/// * `request` - 移动文档请求
///
/// # 返回
/// 返回任务ID
pub async fn move_docs_to_wiki(
    config: &Config,
    space_id: &str,
    request: MoveDocsToWikiRequest,
) -> SDKResult<MoveDocsToWikiResponse> {
    // 使用 ApiEndpoint 枚举生成 URL
    let api_endpoint = WikiApiV2::SpaceNodeMoveDocsToWiki(space_id.to_string());

    // 创建 API 请求
    let api_request: ApiRequest<MoveDocsToWikiResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(&request)?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    Ok(response.data.ok_or_else(|| {
        openlark_core::error::CoreError::validation_msg("响应数据为空")
    })?)
}
