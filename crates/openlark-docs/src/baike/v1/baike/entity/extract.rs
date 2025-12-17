use crate::baike::models::*;
/// 提取潜在的词条
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/entity/extract
///
/// 提取文本中可能成为词条的词语，且不会过滤已经成为词条的词语。
/// 同时返回推荐的别名。
use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    constants::AccessTokenType,
    error::SDKResult,
    req_option::RequestOption,
    request_builder::UnifiedRequestBuilder,
};
use serde::{Deserialize, Serialize};

/// 提取潜在词条请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractEntityRequest {
    /// 需要分析的文本
    pub text: String,
    /// 分类ID过滤
    pub classification_id: Option<String>,
    /// 词典ID过滤
    pub repo_id: Option<String>,
    /// 最大提取数量
    pub max_extracts: Option<i32>,
    /// 最小置信度阈值
    pub min_confidence: Option<f64>,
}

/// 提取潜在词条响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractEntityResponse {
    /// 提取结果列表
    pub extracts: Vec<EntityExtractResult>,
    /// 原始文本
    pub text: String,
}

/// 提取潜在词条构建器
pub struct ExtractEntityBuilder<'a> {
    config: &'a Config,
    request: ExtractEntityRequest,
}

impl<'a> ExtractEntityBuilder<'a> {
    /// 创建新的提取潜在词条构建器
    pub fn new(config: &'a Config, text: String) -> Self {
        Self {
            config,
            request: ExtractEntityRequest {
                text,
                classification_id: None,
                repo_id: None,
                max_extracts: Some(10),
                min_confidence: Some(0.5),
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

    /// 设置最大提取数量
    pub fn max_extracts(mut self, max_extracts: i32) -> Self {
        self.request.max_extracts = Some(max_extracts);
        self
    }

    /// 设置最小置信度阈值
    pub fn min_confidence(mut self, min_confidence: f64) -> Self {
        self.request.min_confidence = Some(min_confidence);
        self
    }

    /// 执行提取潜在词条操作
    pub async fn execute(self) -> SDKResult<ExtractEntityResponse> {
        let mut api_request: ApiRequest<ExtractEntityResponse> =
            ApiRequest::post("/open-apis/baike/v1/entities/extract")
                .body(serde_json::to_value(&self.request)?);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        )
        .await?;

        let response = http_request.send().await?;
        let resp: Response<_> = response.json().await?;
        resp.into_result()
    }
}
