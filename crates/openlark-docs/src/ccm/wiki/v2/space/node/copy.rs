/// 复制知识空间节点
///
/// 此接口用于复制知识空间节点到指定位置，包括节点内容和子节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/copy

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 复制知识空间节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopySpaceNodeRequest {
    /// 空间ID
    pub space_id: String,
    /// 节点ID
    pub node_id: String,
    /// 目标父节点ID
    pub parent_node_id: String,
    /// 复制后的节点标题
    pub title: Option<String>,
    /// 是否复制子节点
    pub recursive: Option<bool>,
}

/// 复制后的节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopiedNode {
    /// 新节点ID
    pub node_id: String,
    /// 节点标题
    pub title: String,
    /// 父节点ID
    pub parent_node_id: String,
    /// 节点类型
    pub node_type: String,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 复制知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopySpaceNodeResponse {
    /// 复制后的节点信息
    pub data: Option<CopiedNode>,
}

impl ApiResponseTrait for CopySpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 复制知识空间节点
///
/// 此接口用于复制知识空间节点到指定位置，包括节点内容和子节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/copy
pub async fn copy_space_node(
    request: CopySpaceNodeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CopySpaceNodeResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id,
        "node_id": request.node_id,
        "parent_node_id": request.parent_node_id
    });

    if let Some(title) = request.title {
        body["title"] = serde_json::json!(title);
    }
    if let Some(recursive) = request.recursive {
        body["recursive"] = serde_json::json!(recursive);
    }

    // 创建API请求
    let mut api_request: ApiRequest<CopySpaceNodeResponse> =
        ApiRequest::post("/open-apis/wiki/v2/spaces/nodes/copy")
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
    fn test_copy_space_node_request() {
        let request = CopySpaceNodeRequest {
            space_id: "space_123".to_string(),
            node_id: "node_456".to_string(),
            parent_node_id: "node_789".to_string(),
            title: Some("复制的文档".to_string()),
            recursive: Some(true),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_id, "node_456");
        assert_eq!(request.title, Some("复制的文档".to_string()));
        assert_eq!(request.recursive, Some(true));
    }

    #[test]
    fn test_copied_node() {
        let node = CopiedNode {
            node_id: "new_node_456".to_string(),
            title: "复制的文档".to_string(),
            parent_node_id: "node_789".to_string(),
            node_type: "document".to_string(),
            create_time: Some(1609459200),
            update_time: Some(1609459200),
        };

        assert_eq!(node.node_id, "new_node_456");
        assert_eq!(node.title, "复制的文档");
        assert_eq!(node.node_type, "document");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CopySpaceNodeResponse::data_format(), ResponseFormat::Data);
    }
}
