/// 获取知识空间节点信息
///
/// 获取知识空间节点信息。
/// 文档参考：https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV2;
use crate::wiki::v2::models::WikiSpaceNode;

/// 获取知识空间节点信息请求
pub struct GetWikiSpaceNodeRequest {
    config: Config,
}

/// 获取知识空间节点信息请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceNodeParams {
    /// 节点Token
    pub node_token: String,
    /// 知识空间ID
    pub space_id: String,
}

/// 获取知识空间节点信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWikiSpaceNodeResponse {
    /// 节点信息
    pub data: Option<WikiSpaceNode>,
}

impl ApiResponseTrait for GetWikiSpaceNodeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetWikiSpaceNodeRequest {
    /// 创建获取知识空间节点信息请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// API文档: https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-node/get_node
    pub async fn execute(
        self,
        params: GetWikiSpaceNodeParams,
    ) -> SDKResult<GetWikiSpaceNodeResponse> {
        // 验证必填字段
        validate_required!(params.node_token, "节点Token不能为空");
        validate_required!(params.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceGetNode;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<GetWikiSpaceNodeResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 设置查询参数
        api_request = api_request.query("node_token", &params.node_token);
        api_request = api_request.query("space_id", &params.space_id);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
