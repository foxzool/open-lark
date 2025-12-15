/// 精准搜索词条
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/match
///
/// 将关键词与词条名、别名精准匹配，并返回对应的词条ID。

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

/// 精准搜索词条请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntityRequest {
    /// 搜索关键词
    pub query: String,
    /// 分类ID过滤
    pub classification_id: Option<String>,
    /// 词典ID过滤
    pub repo_id: Option<String>,
    /// 匹配类型限制：name/alias/all
    pub match_type: Option<String>,
}

/// 精准搜索词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEntityResponse {
    /// 匹配结果列表
    pub matches: Vec<EntityMatchResult>,
    /// 搜索关键词
    pub query: String,
}

/// 精准搜索词条构建器
pub struct MatchEntityBuilder<'a> {
    config: &'a Config,
    request: MatchEntityRequest,
}

impl<'a> MatchEntityBuilder<'a> {
    /// 创建新的精准搜索词条构建器
    pub fn new(config: &'a Config, query: String) -> Self {
        Self {
            config,
            request: MatchEntityRequest {
                query,
                classification_id: None,
                repo_id: None,
                match_type: None,
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

    /// 设置匹配类型限制
    pub fn match_type(mut self, match_type: String) -> Self {
        self.request.match_type = Some(match_type);
        self
    }

    /// 执行精准搜索词条操作
    pub async fn execute(self) -> SDKResult<MatchEntityResponse> {
        let mut api_request = ApiRequest::post("/open-apis/lingo/v1/entities/match")
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