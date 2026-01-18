//! 模糊搜索词条
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/entity/search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 分类筛选（classification_filter）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationFilter {
    /// 需要获取的分类
    pub include: Option<Vec<String>>,
    /// 需要排除的分类
    pub exclude: Option<Vec<String>>,
}

/// 模糊搜索词条请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityBody {
    /// 搜索关键词
    pub query: Option<String>,
    /// 分类筛选
    pub classification_filter: Option<ClassificationFilter>,
    /// 词条的创建来源，1：用户主动创建，2：批量导入，3：官方词，4：OpenAPI 创建
    pub sources: Option<Vec<i32>>,
    /// 创建者
    pub creators: Option<Vec<String>>,
}

/// 模糊搜索词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchEntityResp {
    /// 词条列表
    #[serde(default)]
    pub entities: Vec<Entity>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SearchEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 模糊搜索词条请求
pub struct SearchEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    repo_id: Option<String>,
    user_id_type: Option<UserIdType>,
    body: SearchEntityBody,
}

impl SearchEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            repo_id: None,
            user_id_type: None,
            body: SearchEntityBody {
                query: None,
                classification_filter: None,
                sources: None,
                creators: None,
            },
        }
    }

    /// 搜索关键词
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = Some(query.into());
        self
    }

    /// 分类筛选
    pub fn classification_filter(mut self, filter: ClassificationFilter) -> Self {
        self.body.classification_filter = Some(filter);
        self
    }

    /// 创建来源过滤
    pub fn sources(mut self, sources: Vec<i32>) -> Self {
        self.body.sources = Some(sources);
        self
    }

    /// 创建者过滤
    pub fn creators(mut self, creators: Vec<String>) -> Self {
        self.body.creators = Some(creators);
        self
    }

    /// 每页返回的词条量（默认 20，范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 词库ID（不传时默认在全员词库内搜索）
    pub fn repo_id(mut self, repo_id: impl Into<String>) -> Self {
        self.repo_id = Some(repo_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchEntityResp> {
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化模糊搜索词条请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<SearchEntityResp> =
            ApiRequest::post(&LingoApiV1::EntitySearch.to_url()).body(body);
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(repo_id) = &self.repo_id {
            api_request = api_request.query("repo_id", repo_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
