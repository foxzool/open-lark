//! 获取词条列表
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/entity/list

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::BaikeApiV1;

/// 获取词条列表响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListEntityResp {
    pub entities: Vec<Entity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条列表请求
pub struct ListEntityRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    provider: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl ListEntityRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            provider: None,
            user_id_type: None,
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

    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn send(self) -> SDKResult<ListEntityResp> {
        let mut api_request: ApiRequest<ListEntityResp> =
            ApiRequest::get(&BaikeApiV1::EntityList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<ListEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
