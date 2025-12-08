//! 获取知识空间节点列表
//!
//! 获取知识空间的节点列表。
//! 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-nodes/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::wiki::v2::models::WikiSpaceNode;
use crate::common::api_endpoints::WikiApiV2;

/// 获取知识空间节点列表请求
pub struct ListWikiSpaceNodesRequest {
    space_id: String,
    config: Config,
}

/// 获取知识空间节点列表请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceNodesParams {
    /// 父节点Token (可选，获取指定节点的子节点)
    pub parent_node_token: Option<String>,
    /// 节点类型过滤 (可选)
    pub node_type: Option<String>,
    /// 每页大小 (默认: 20, 最大: 100)
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

/// 获取知识空间节点列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWikiSpaceNodesResponse {
    /// 节点列表
    pub data: Option<Vec<WikiSpaceNode>>,
    /// 分页信息
    pub page_token: Option<String>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListWikiSpaceNodesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWikiSpaceNodesRequest {
    /// 创建获取知识空间节点列表请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-nodes/list
    pub async fn execute(self, params: Option<ListWikiSpaceNodesParams>) -> SDKResult<ListWikiSpaceNodesResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceNodeList(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListWikiSpaceNodesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        if let Some(params) = params {
            if let Some(parent_node_token) = params.parent_node_token {
                api_request = api_request.query("parent_node_token", &parent_node_token);
            }
            if let Some(node_type) = params.node_type {
                api_request = api_request.query("node_type", &node_type);
            }
            if let Some(page_size) = params.page_size {
                api_request = api_request.query("page_size", &page_size.to_string());
            }
            if let Some(page_token) = params.page_token {
                api_request = api_request.query("page_token", &page_token);
            }
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}