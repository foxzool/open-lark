/// 创建草稿
///
/// API文档: https://open.feishu.cn/document/lingo-v1/draft/create
///
/// 草稿并非词条，而是指通过 API 发起创建新词条或更新现有词条的申请。
/// 词典管理员审核通过后，草稿将变为新的词条或覆盖已有词条。

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

/// 创建草稿请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDraftRequest {
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 操作类型：create/update
    pub operation_type: String,
    /// 词条ID（更新时必填）
    pub entity_id: Option<String>,
    /// 分类ID列表
    pub classification_ids: Option<Vec<String>>,
    /// 词条别名
    pub aliases: Option<Vec<String>>,
    /// 词条封面图片
    pub cover: Option<EntityCover>,
    /// 扩展属性
    pub extra: Option<serde_json::Value>,
}

impl Default for CreateDraftRequest {
    fn default() -> Self {
        Self {
            title: String::new(),
            content: String::new(),
            operation_type: "create".to_string(),
            entity_id: None,
            classification_ids: None,
            aliases: None,
            cover: None,
            extra: None,
        }
    }
}

/// 创建草稿响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDraftResponse {
    /// 草稿ID
    pub draft_id: String,
    /// 创建时间
    pub create_time: String,
}

/// 创建草稿构建器
pub struct CreateDraftBuilder<'a> {
    config: &'a Config,
    request: CreateDraftRequest,
}

impl<'a> CreateDraftBuilder<'a> {
    /// 创建新的创建草稿构建器
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            request: CreateDraftRequest::default(),
        }
    }

    /// 设置草稿标题
    pub fn title(mut self, title: String) -> Self {
        self.request.title = title;
        self
    }

    /// 设置草稿内容
    pub fn content(mut self, content: String) -> Self {
        self.request.content = content;
        self
    }

    /// 设置操作类型
    pub fn operation_type(mut self, operation_type: String) -> Self {
        self.request.operation_type = operation_type;
        self
    }

    /// 设置词条ID（更新时使用）
    pub fn entity_id(mut self, entity_id: String) -> Self {
        self.request.entity_id = Some(entity_id);
        self
    }

    /// 设置分类ID列表
    pub fn classification_ids(mut self, classification_ids: Vec<String>) -> Self {
        self.request.classification_ids = Some(classification_ids);
        self
    }

    /// 设置词条别名
    pub fn aliases(mut self, aliases: Vec<String>) -> Self {
        self.request.aliases = Some(aliases);
        self
    }

    /// 设置词条封面
    pub fn cover(mut self, cover: EntityCover) -> Self {
        self.request.cover = Some(cover);
        self
    }

    /// 设置扩展属性
    pub fn extra(mut self, extra: serde_json::Value) -> Self {
        self.request.extra = Some(extra);
        self
    }

    /// 执行创建草稿操作
    pub async fn execute(self) -> SDKResult<CreateDraftResponse> {
        let mut api_request: ApiRequest<CreateDraftResponse> = ApiRequest::post("/open-apis/lingo/v1/drafts")
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