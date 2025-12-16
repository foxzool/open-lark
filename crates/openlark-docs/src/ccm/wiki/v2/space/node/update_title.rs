/// 更新知识空间节点标题
///
/// 此接口用于更新知识空间节点的标题，支持修改文档或文件夹的名称。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/update_title

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新知识空间节点标题请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceNodeTitleRequest {
    /// 空间ID
    pub space_id: String,
    /// 节点ID
    pub node_id: String,
    /// 新标题
    pub title: String,
}

/// 更新后的节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatedNode {
    /// 节点ID
    pub node_id: String,
    /// 新标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 更新知识空间节点标题响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSpaceNodeTitleResponse {
    /// 更新后的节点信息
    pub data: Option<UpdatedNode>,
}

impl ApiResponseTrait for UpdateSpaceNodeTitleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新知识空间节点标题
///
/// 此接口用于更新知识空间节点的标题，支持修改文档或文件夹的名称。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/update_title
pub async fn update_space_node_title(
    request: UpdateSpaceNodeTitleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateSpaceNodeTitleResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "space_id": request.space_id,
        "node_id": request.node_id,
        "title": request.title
    });

    // 创建API请求
    let mut api_request: ApiRequest<UpdateSpaceNodeTitleResponse> =
        ApiRequest::patch("/open-apis/wiki/v2/spaces/nodes/title")
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_space_node_title_request() {
        let request = UpdateSpaceNodeTitleRequest {
            space_id: "space_123".to_string(),
            node_id: "node_456".to_string(),
            title: "新标题".to_string(),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_id, "node_456");
        assert_eq!(request.title, "新标题");
    }

    #[test]
    fn test_updated_node() {
        let node = UpdatedNode {
            node_id: "node_456".to_string(),
            title: "新标题".to_string(),
            node_type: "document".to_string(),
            update_time: Some(1609459200),
        };

        assert_eq!(node.node_id, "node_456");
        assert_eq!(node.title, "新标题");
        assert_eq!(node.node_type, "document");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateSpaceNodeTitleResponse::data_format(), ResponseFormat::Data);
    }
}
