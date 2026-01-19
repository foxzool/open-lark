//! 搜索空间操作状态
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/space_operation_status

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 操作状态信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpaceOperationStatus {
    /// 操作 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 进度
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i32>,
}

/// 搜索空间操作状态响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSpaceOperationStatusResp {
    /// 操作状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<SpaceOperationStatus>,
}

impl ApiResponseTrait for SearchSpaceOperationStatusResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索空间操作状态请求
pub struct SearchSpaceOperationStatusRequest {
    config: Config,
    space_id: String,
    operation_id: String,
    user_id_type: Option<UserIdType>,
}

impl SearchSpaceOperationStatusRequest {
    pub fn new(
        config: Config,
        space_id: impl Into<String>,
        operation_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            space_id: space_id.into(),
            operation_id: operation_id.into(),
            user_id_type: None,
        }
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchSpaceOperationStatusResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchSpaceOperationStatusResp> {
        validate_required!(self.space_id, "space_id 不能为空");
        validate_required!(self.operation_id, "operation_id 不能为空");

        let mut api_request: ApiRequest<SearchSpaceOperationStatusResp> =
            ApiRequest::get(&BaikeApiV1::SearchSpaceOperationStatus.to_url())
                .query("space_id", &self.space_id)
                .query("operation_id", &self.operation_id);

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchSpaceOperationStatusResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
