//! 审批拒绝词条
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/reject

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::Entity;
use crate::common::api_endpoints::BaikeApiV1;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RejectEntityReq {
    /// 拒绝原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 审批拒绝词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RejectEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for RejectEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 审批拒绝词条请求
pub struct RejectEntityRequest {
    config: Config,
    entity_id: String,
    req: RejectEntityReq,
}

impl RejectEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>, req: RejectEntityReq) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            req,
        }
    }

    pub async fn execute(self) -> SDKResult<RejectEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");

        let api_request: ApiRequest<RejectEntityResp> =
            ApiRequest::post(&BaikeApiV1::EntityReject(self.entity_id).to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<RejectEntityResp> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
