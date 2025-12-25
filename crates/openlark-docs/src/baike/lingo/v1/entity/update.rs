//! 更新免审词条
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/entity/update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::lingo::v1::models::{Entity, EntityInput, UserIdType};
use crate::common::api_endpoints::LingoApiV1;

/// 更新免审词条响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEntityResp {
    pub entity: Entity,
}

impl ApiResponseTrait for UpdateEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新免审词条请求
pub struct UpdateEntityRequest {
    config: Config,
    entity_id: String,
    body: EntityInput,
    user_id_type: Option<UserIdType>,
}

impl UpdateEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>, body: EntityInput) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            body,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn send(self) -> SDKResult<UpdateEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");
        validate_required!(self.body.main_keys, "main_keys 不能为空");

        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::serialization_error("序列化更新免审词条请求体失败", Some(e))
        })?;

        let mut api_request: ApiRequest<UpdateEntityResp> =
            ApiRequest::put(&LingoApiV1::EntityUpdate(self.entity_id).to_url()).body(body);
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<UpdateEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
