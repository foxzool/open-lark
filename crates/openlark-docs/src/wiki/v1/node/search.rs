//! 搜索Wiki
//!
//! 搜索Wiki，用户通过关键词查询Wiki，只能查找自己可见的wiki。
//! 文档参考：https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/wiki-v1/search_wiki

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::WikiApiV1;
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
    /// 空间ID列表
    pub space_ids: Option<Vec<String>>,
    /// 节点类型过滤
    pub node_type: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 页面标记
    pub page_token: Option<String>,
}

/// 搜索Wiki响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchWikiResponse {
    /// 搜索结果列表
    pub data: Option<Vec<WikiSearchResult>>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 页面标记
    pub page_token: Option<String>,
    /// 总数
    pub total: Option<i32>,
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
    /// API文档: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/wiki-v1/search_wiki
    pub async fn execute(self, params: SearchWikiParams) -> SDKResult<SearchWikiResponse> {
        // 验证必填字段
        validate_required!(params.query, "搜索关键词不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV1::NodeSearch;

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<SearchWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url());

        // 设置请求体
        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(
            &params,
        )?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
