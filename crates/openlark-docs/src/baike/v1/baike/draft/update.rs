/// 更新草稿
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/draft/update
///
/// 根据 draft_id 更新草稿内容，已审批的草稿无法编辑。

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

/// 更新草稿请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDraftRequest {
    /// 草稿ID
    pub draft_id: String,
    /// 草稿标题
    pub title: String,
    /// 草稿内容
    pub content: String,
    /// 分类ID列表
    pub classification_ids: Option<Vec<String>>,
    /// 词条别名
    pub aliases: Option<Vec<String>>,
    /// 词条封面图片
    pub cover: Option<EntityCover>,
    /// 扩展属性
    pub extra: Option<serde_json::Value>,
}

/// 更新草稿响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDraftResponse {
    /// 草稿ID
    pub draft_id: String,
    /// 更新时间
    pub update_time: String,
}

/// 更新草稿构建器
pub struct UpdateDraftBuilder<'a> {
    config: &'a Config,
    request: UpdateDraftRequest,
}

impl<'a> UpdateDraftBuilder<'a> {
    /// 创建新的更新草稿构建器
    pub fn new(config: &'a Config, draft_id: String) -> Self {
        Self {
            config,
            request: UpdateDraftRequest {
                draft_id,
                title: String::new(),
                content: String::new(),
                classification_ids: None,
                aliases: None,
                cover: None,
                extra: None,
            },
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

    /// 执行更新草稿操作
    pub async fn execute(self) -> SDKResult<UpdateDraftResponse> {
        let path = format!("/open-apis/baike/v1/drafts/{}", self.request.draft_id);
        let mut api_request = ApiRequest::put(&path)
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