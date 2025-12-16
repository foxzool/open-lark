/// 词条高亮
///
/// API文档: https://open.feishu.cn/document/lingo-v1/entity/highlight
///
/// 传入一句话，智能识别句中对应的词条，并返回词条位置和 entity_id，
/// 可在外部系统中快速实现词条智能高亮。

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

/// 词条高亮请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityRequest {
    /// 需要高亮的文本
    pub text: String,
    /// 分类ID过滤
    pub classification_id: Option<String>,
    /// 词典ID过滤
    pub repo_id: Option<String>,
    /// 最大高亮数量
    pub max_highlights: Option<i32>,
}

/// 词条高亮响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightEntityResponse {
    /// 高亮结果列表
    pub highlights: Vec<HighlightInfo>,
    /// 原始文本
    pub text: String,
}

/// 词条高亮构建器
pub struct HighlightEntityBuilder<'a> {
    config: &'a Config,
    request: HighlightEntityRequest,
}

impl<'a> HighlightEntityBuilder<'a> {
    /// 创建新的词条高亮构建器
    pub fn new(config: &'a Config, text: String) -> Self {
        Self {
            config,
            request: HighlightEntityRequest {
                text,
                classification_id: None,
                repo_id: None,
                max_highlights: Some(10),
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

    /// 设置最大高亮数量
    pub fn max_highlights(mut self, max_highlights: i32) -> Self {
        self.request.max_highlights = Some(max_highlights);
        self
    }

    /// 执行词条高亮操作
    pub async fn execute(self) -> SDKResult<HighlightEntityResponse> {
        let mut api_request: ApiRequest<HighlightEntityResponse> = ApiRequest::post("/open-apis/lingo/v1/entities/highlight")
            .body(serde_json::to_value(&self.request)?);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = http_request.send().await?;
        let resp: Response<_> = response.json().await?;
        resp.into_result()
    }
}