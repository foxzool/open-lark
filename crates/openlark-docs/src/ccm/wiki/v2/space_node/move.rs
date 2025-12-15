/// 移动知识空间节点
///
/// 此接口用于在知识空间中移动节点到新的父节点下，重新组织知识库的层级结构。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/move

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 移动知识空间节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSpaceNodeRequest {
    /// 空间ID
    pub space_id: String,
    /// 节点ID
    pub node_id: String,
    /// 目标父节点ID
    pub parent_node_id: String,
}

/// 移动后的节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovedNode {
    /// 节点ID
    pub node_id: String,
    /// 节点标题
    pub title: String,
    /// 原父节点ID
    pub old_parent_node_id: String,
    /// 新父节点ID
    pub new_parent_node_id: String,
    /// 节点类型
    pub node_type: String,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 移动知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveSpaceNodeResponse {
    /// 移动后的节点信息
    pub data: Option<MovedNode>,
}

impl ApiResponseTrait for MoveSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移动知识空间节点
///
/// 此接口用于在知识空间中移动节点到新的父节点下，重新组织知识库的层级结构。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/move
pub async fn move_space_node(
    request: MoveSpaceNodeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<MoveSpaceNodeResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "space_id": request.space_id,
        "node_id": request.node_id,
        "parent_node_id": request.parent_node_id
    });

    // 创建API请求
    let mut api_request: ApiRequest<MoveSpaceNodeResponse> =
        ApiRequest::patch("/open-apis/wiki/v2/spaces/nodes/move")
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
    fn test_move_space_node_request() {
        let request = MoveSpaceNodeRequest {
            space_id: "space_123".to_string(),
            node_id: "node_456".to_string(),
            parent_node_id: "node_789".to_string(),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_id, "node_456");
        assert_eq!(request.parent_node_id, "node_789");
    }

    #[test]
    fn test_moved_node() {
        let node = MovedNode {
            node_id: "node_456".to_string(),
            title: "移动的文档".to_string(),
            old_parent_node_id: "old_parent".to_string(),
            new_parent_node_id: "new_parent".to_string(),
            node_type: "document".to_string(),
            update_time: Some(1609459200),
        };

        assert_eq!(node.node_id, "node_456");
        assert_eq!(node.old_parent_node_id, "old_parent");
        assert_eq!(node.new_parent_node_id, "new_parent");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(MoveSpaceNodeResponse::data_format(), ResponseFormat::Data);
    }
}
