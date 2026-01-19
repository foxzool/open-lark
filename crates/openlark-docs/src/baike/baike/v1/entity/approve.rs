//! 审批通过词条
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/entity/approve

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
pub struct ApproveEntityReq {
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 审批通过词条响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApproveEntityResp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Entity>,
}

impl ApiResponseTrait for ApproveEntityResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 审批通过词条请求
pub struct ApproveEntityRequest {
    config: Config,
    entity_id: String,
    req: ApproveEntityReq,
}

impl ApproveEntityRequest {
    pub fn new(config: Config, entity_id: impl Into<String>, req: ApproveEntityReq) -> Self {
        Self {
            config,
            entity_id: entity_id.into(),
            req,
        }
    }

    pub async fn execute(self) -> SDKResult<ApproveEntityResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ApproveEntityResp> {
        validate_required!(self.entity_id, "entity_id 不能为空");

        let api_request: ApiRequest<ApproveEntityResp> =
            ApiRequest::post(&BaikeApiV1::EntityApprove(self.entity_id).to_url())
                .body(serde_json::to_value(&self.req)?);

        let response: Response<ApproveEntityResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
