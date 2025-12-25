//! 模糊搜索词条
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/search

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityReqBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_filter: Option<ClassificationFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creators: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchEntityResponse {
    pub entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchEntityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ClassificationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

use crate::baike::baike::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

/// 模糊搜索词条请求
pub struct SearchEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
    req: SearchEntityReqBody,
}

impl SearchEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            req: SearchEntityReqBody::default(),
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.req.query = Some(query.into());
        self
    }

    pub fn classification_filter(mut self, filter: ClassificationFilter) -> Self {
        self.req.classification_filter = Some(filter);
        self
    }

    pub fn sources(mut self, sources: Vec<i32>) -> Self {
        self.req.sources = Some(sources);
        self
    }

    pub fn creators(mut self, creators: Vec<String>) -> Self {
        self.req.creators = Some(creators);
        self
    }

    pub async fn send(self) -> SDKResult<SearchEntityResponse> {
        let mut api_request: ApiRequest<SearchEntityResponse> =
            ApiRequest::post(&BaikeApiV1::EntitySearch.to_url()).body(serde_json::to_value(&self.req)?);

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchEntityResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
