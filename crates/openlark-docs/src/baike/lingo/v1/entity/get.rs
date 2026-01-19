//! 获取词条详情
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/entity/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 获取词条详情响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for GetEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取词条详情请求
pub struct GetEntityRequest {
    config: Config,
    entity_id: String,
    provider: Option<String>,
    outer_id: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl GetEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            provider: None,
            outer_id: None,
            user_id_type: None,
        }
    }

    /// 外部系统（可选）
    pub fn provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// 词条在外部系统中对应的唯一 ID（可选）
    pub fn outer_id(mut self, outer_id: impl Into<String>) -> Self {
        self.outer_id = Some(outer_id.into());
        self
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<GetEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<GetEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");

        let mut api_request: ApiRequest<GetEntityResp> =
            ApiRequest::get(&LingoApiV1::EntityGet(self.entity_id).to_url());
        if let Some(provider) = &self.provider {
            api_request = api_request.query("provider", provider);
        }
        if let Some(outer_id) = &self.outer_id {
            api_request = api_request.query("outer_id", outer_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<GetEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
