/// 创建知识空间节点
///
/// 此接口用于在知识空间中创建新的文档节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/create

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建知识空间节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceNodeRequest {
    /// 空间ID
    pub space_id: String,
    /// 父节点ID，根节点为0
    pub parent_node_id: String,
    /// 节点类型：document(文档)、folder(文件夹)、sheet(表格)、mindnote(思维导图)、file(文件)、bitable(多维表格)
    pub node_type: String,
    /// 节点标题
    pub title: String,
    /// 原始文档连接（当node_type为document时需要）
    pub origin_token: Option<String>,
    /// 原始文档类型（当node_type为document时需要）
    pub origin_type: Option<String>,
}

/// 创建的知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedNode {
    /// 节点ID
    pub node_id: String,
    /// 节点标题
    pub title: String,
    /// 节点类型
    pub node_type: String,
    /// 父节点ID
    pub parent_node_id: String,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 创建知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpaceNodeResponse {
    /// 创建的节点信息
    pub data: Option<CreatedNode>,
}

impl ApiResponseTrait for CreateSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建知识空间节点
///
/// 此接口用于在知识空间中创建新的文档节点。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/create
pub async fn create_space_node(
    request: CreateSpaceNodeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<CreateSpaceNodeResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id,
        "parent_node_id": request.parent_node_id,
        "node_type": request.node_type,
        "title": request.title
    });

    if let Some(origin_token) = request.origin_token {
        body["origin_token"] = serde_json::json!(origin_token);
    }
    if let Some(origin_type) = request.origin_type {
        body["origin_type"] = serde_json::json!(origin_type);
    }

    // 创建API请求
    let mut api_request: ApiRequest<CreateSpaceNodeResponse> =
        ApiRequest::post("/open-apis/wiki/v2/spaces/nodes")
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
    fn test_create_space_node_request() {
        let request = CreateSpaceNodeRequest {
            space_id: "space_123".to_string(),
            parent_node_id: "0".to_string(),
            node_type: "document".to_string(),
            title: "测试文档".to_string(),
            origin_token: Some("token_456".to_string()),
            origin_type: Some("docx".to_string()),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_type, "document");
        assert_eq!(request.title, "测试文档");
    }

    #[test]
    fn test_created_node() {
        let node = CreatedNode {
            node_id: "node_789".to_string(),
            title: "测试文档".to_string(),
            node_type: "document".to_string(),
            parent_node_id: "0".to_string(),
            create_time: Some(1609459200),
            update_time: Some(1609459200),
        };

        assert_eq!(node.node_id, "node_789");
        assert_eq!(node.node_type, "document");
        assert_eq!(node.parent_node_id, "0");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateSpaceNodeResponse::data_format(), ResponseFormat::Data);
    }
}
