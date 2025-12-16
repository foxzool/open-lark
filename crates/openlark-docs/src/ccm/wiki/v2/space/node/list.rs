/// 获取知识空间节点列表
///
/// 此接口用于获取知识空间下的节点列表，包括文件夹和文档。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取知识空间节点列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceNodesRequest {
    /// 空间ID
    pub space_id: String,
    /// 父节点ID，根节点为0
    pub parent_node_id: String,
    /// 页面大小，默认20，最大100
    pub page_size: Option<i32>,
    /// 页码，从1开始
    pub page_token: Option<String>,
    /// 节点类型过滤：document(文档)、folder(文件夹)、sheet(表格)、mindnote(思维导图)、file(文件)、bitable(多维表格)
    pub node_type: Option<String>,
}

/// 知识空间节点信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceNodeItem {
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

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: Option<String>,
    /// 用户名称
    pub name: Option<String>,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTokenInfo {
    /// 是否还有下一页
    pub has_more: Option<bool>,
    /// 页码
    pub page_token: Option<String>,
}

/// 获取知识空间节点列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceNodesResponse {
    /// 节点列表数据
    pub data: Option<ListSpaceNodesData>,
}

/// 节点列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpaceNodesData {
    /// 节点列表
    pub items: Option<Vec<SpaceNodeItem>>,
    /// 分页信息
    pub page_token: Option<PageTokenInfo>,
}

impl ApiResponseTrait for ListSpaceNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取知识空间节点列表
///
/// 此接口用于获取知识空间下的节点列表，包括文件夹和文档。
/// docPath: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space_node/list
pub async fn list_space_nodes(
    request: ListSpaceNodesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ListSpaceNodesResponse>> {
    // 构建请求体
    let mut body = serde_json::json!({
        "space_id": request.space_id,
        "parent_node_id": request.parent_node_id
    });

    if let Some(page_size) = request.page_size {
        body["page_size"] = serde_json::json!(page_size);
    }
    if let Some(page_token) = request.page_token {
        body["page_token"] = serde_json::json!(page_token);
    }
    if let Some(node_type) = request.node_type {
        body["node_type"] = serde_json::json!(node_type);
    }

    // 创建API请求
    let mut api_request: ApiRequest<ListSpaceNodesResponse> =
        ApiRequest::get("/open-apis/wiki/v2/spaces/nodes")
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
    fn test_list_space_nodes_request() {
        let request = ListSpaceNodesRequest {
            space_id: "space_123".to_string(),
            parent_node_id: "0".to_string(),
            page_size: Some(20),
            page_token: Some("token_456".to_string()),
            node_type: Some("document".to_string()),
        };

        assert_eq!(request.space_id, "space_123");
        assert_eq!(request.parent_node_id, "0");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.node_type, Some("document"));
    }

    #[test]
    fn test_space_node_item() {
        let node = SpaceNodeItem {
            node_id: "node_789".to_string(),
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

        assert_eq!(node.node_id, "node_789");
        assert_eq!(node.node_type, "document");
        assert_eq!(node.parent_node_id, "0");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListSpaceNodesResponse::data_format(), ResponseFormat::Data);
    }
}
