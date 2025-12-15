/// 模糊搜索词条
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/entity/search
///
/// 传入关键词，与词条名、别名、释义等信息进行模糊匹配，返回搜到的词条信息。

use openlark_core::{
    error::SDKResult,
    config::Config,
    request_builder::UnifiedRequestBuilder,
    constants::AccessTokenType,
    api::{ApiRequest, Response},
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use crate::baike::models::*;

/// 模糊搜索词条请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityRequest {
    /// 搜索关键词
    pub query: String,
    /// 分类ID过滤
    pub classification_id: Option<String>,
    /// 词典ID过滤
    pub repo_id: Option<String>,
    /// 每页大小
    pub page_size: Option<i32>,
    /// 分页token
    pub page_token: Option<String>,
}

/// 模糊搜索词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityResponse {
    /// 搜索结果列表
    pub items: Vec<EntitySearchResult>,
    /// 分页token
    pub page_token: Option<String>,
    /// 是否有下一页
    pub has_more: bool,
    /// 搜索关键词
    pub query: String,
}

/// 模糊搜索词条构建器
pub struct SearchEntityBuilder<'a> {
    config: &'a Config,
    request: SearchEntityRequest,
}

impl<'a> SearchEntityBuilder<'a> {
    /// 创建新的模糊搜索词条构建器
    pub fn new(config: &'a Config, query: String) -> Self {
        Self {
            config,
            request: SearchEntityRequest {
                query,
                classification_id: None,
                repo_id: None,
                page_size: Some(20),
                page_token: None,
            },
        }
    }

    /// 设置分类ID过滤
    pub fn classification_id(mut self, classification_id: String) -> Self {
        self.request.classification_id = Some(classification_id);
        self
    }

    /// 设置词典ID过滤
    pub fn repo_id(mut self, repo_id: String) -> Self {
        self.request.repo_id = Some(repo_id);
        self
    }

    /// 设置每页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页token
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request.page_token = Some(page_token);
        self
    }

    /// 执行模糊搜索词条操作
    pub async fn execute(self) -> SDKResult<SearchEntityResponse> {
        let mut api_request = ApiRequest::post("/open-apis/baike/v1/entities/search")
            .body(serde_json::to_value(&self.request)?);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = self.config.http_client().execute(http_request).await?;
        let raw_response = Response::from_reqwest_response(response).await?;

        raw_response.into_result()
    }
}