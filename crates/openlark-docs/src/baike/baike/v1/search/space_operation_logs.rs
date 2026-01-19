//! 搜索空间操作日志
//!
//! docPath: https://open.feishu.cn/document/server-docs/baike-v1/search/space_operation_logs

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

/// 操作日志条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpaceOperationLogItem {
    /// 操作 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    /// 操作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// 操作时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

/// 搜索空间操作日志响应（data）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchSpaceOperationLogsResp {
    /// 操作日志列表
    #[serde(default)]
    pub items: Vec<SpaceOperationLogItem>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for SearchSpaceOperationLogsResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索空间操作日志请求
pub struct SearchSpaceOperationLogsRequest {
    config: Config,
    space_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<UserIdType>,
}

impl SearchSpaceOperationLogsRequest {
    pub fn new(config: Config, space_id: impl Into<String>) -> Self {
        Self {
            config,
            space_id: space_id.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchSpaceOperationLogsResp> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<SearchSpaceOperationLogsResp> {
        if let Some(page_size) = self.page_size {
            if !(1..=100).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~100",
                ));
            }
        }

        let mut api_request: ApiRequest<SearchSpaceOperationLogsResp> =
            ApiRequest::get(&BaikeApiV1::SearchSpaceOperationLog.to_url())
                .query("space_id", &self.space_id);

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response: Response<SearchSpaceOperationLogsResp> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
