/// 搜索Wiki
///
/// 搜索Wiki，用户通过关键词查询Wiki，只能查找自己可见的wiki。
/// docPath: /document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki
/// doc: https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV1;
use crate::common::api_utils::*;
use crate::wiki::v2::models::WikiSearchResult;

/// 搜索Wiki请求
pub struct SearchWikiRequest {
    config: Config,
}

/// 搜索Wiki请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiParams {
    /// 搜索关键词
    pub query: String,
    /// 空间ID（可选）
    pub space_id: Option<String>,
    /// 节点ID（可选）
    pub node_id: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 搜索Wiki响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiResponse {
    /// 搜索结果列表
    #[serde(default)]
    pub items: Vec<WikiSearchResult>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 页面标记
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SearchWikiRequest {
    /// 创建搜索Wiki请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uEzN0YjLxcDN24SM3QjN/search_wiki
    /// doc: https://open.feishu.cn/document/server-docs/docs/wiki-v2/search_wiki
    pub async fn execute(self, params: SearchWikiParams) -> SDKResult<SearchWikiResponse> {
        // 验证必填字段
        validate_required!(params.query, "搜索关键词不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV1::NodeSearch;

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<SearchWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "搜索Wiki")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "搜索Wiki")
    }
}
