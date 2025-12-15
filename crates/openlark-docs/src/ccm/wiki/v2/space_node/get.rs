/// 获取知识空间节点详情
///
/// 此接口用于获取知识空间节点的详细信息，包括节点标题、类型、创建时间等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use super::list::CreatorInfo;

/// 获取知识空间节点请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceNodeRequest {
    /// 空间ID
    pub space_id: String,
    /// 节点ID
    pub node_id: String,
    /// 需要获取的字段，多个字段用逗号分隔
    pub fields: Option<String>,
}

/// 知识空间节点详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceNode {
    /// 节点ID
    pub node_id: String,
    /// 节点标题
    pub title: String,
    /// 节点类型：document(文档)、folder(文件夹)、sheet(表格)、mindnote(思维导图)、file(文件)、bitable(多维表格)
    pub node_type: String,
    /// 父节点ID
    pub parent_node_id: String,
    /// 子节点数量（仅文件夹有此字段）
    pub child_node_count: Option<i32>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
    /// 创建者信息
    pub creator: Option<CreatorInfo>,
}

/// 获取知识空间节点响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpaceNodeResponse {
    /// 节点详情
    pub data: Option<SpaceNode>,
}

impl ApiResponseTrait for GetSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间节点详情
///
/// 此接口用于获取知识空间节点的详细信息，包括节点标题、类型、创建时间等。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/get
pub async fn get_space_node(
    request: GetSpaceNodeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetSpaceNodeResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id,
        "node_id": request.node_id
    });

    if let Some(fields) = request.fields {
        body["fields"] = serde_json::json!(fields);
    }

    // 创建API请求
    let mut api_request: ApiRequest<GetSpaceNodeResponse> =
        ApiRequest::get("/open-apis/wiki/v2/spaces/nodes/get")
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
    fn test_get_space_node_request() {
        let request = GetSpaceNodeRequest {
            space_id: "space_123".to_string(),
            node_id: "node_456".to_string(),
            fields: Some("title,node_type,create_time".to_string()),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.node_id, "node_456");
        assert_eq!(request.fields, Some("title,node_type,create_time".to_string()));
    }

    #[test]
    fn test_space_node() {
        let node = SpaceNode {
            node_id: "node_456".to_string(),
            title: "测试文档".to_string(),
            node_type: "document".to_string(),
            parent_node_id: "0".to_string(),
            child_node_count: None,
            create_time: Some(1609459200),
            update_time: Some(1609459200),
            creator: Some(CreatorInfo {
                user_id: Some("user_123".to_string()),
                name: Some("张三".to_string()),
            }),
        };

        assert_eq!(node.node_id, "node_456");
        assert_eq!(node.node_type, "document");
        assert_eq!(node.parent_node_id, "0");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetSpaceNodeResponse::data_format(), ResponseFormat::Data);
    }
}
