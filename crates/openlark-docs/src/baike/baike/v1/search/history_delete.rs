//! 删除搜索历史
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/history_delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::baike::baike::v1::models::UserIdType;
use crate::common::api_endpoints::BaikeApiV1;

/// 删除搜索历史响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteSearchHistoryResp {}

impl ApiResponseTrait for DeleteSearchHistoryResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除搜索历史请求
pub struct DeleteSearchHistoryRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
}

impl DeleteSearchHistoryRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
        }
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<DeleteSearchHistoryResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteSearchHistoryResp> {
        let mut api_request: ApiRequest<DeleteSearchHistoryResp> =
            ApiRequest::delete(&BaikeApiV1::SearchHistoryDelete.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<DeleteSearchHistoryResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
