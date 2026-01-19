//! 获取妙记统计数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::MinutesApiV1, api_utils::*};

#[derive(Debug, Clone)]
pub struct GetMinuteStatisticsRequest {
    config: Config,
    minute_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetMinuteStatisticsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
            user_id_type: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetMinuteStatisticsResponse> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<GetMinuteStatisticsResponse> {

        let minute_token = self.minute_token.ok_or_else(|| {
            openlark_core::error::validation_error("minute_token", "minute_token 不能为空")
        })?;
        if minute_token.chars().count() != 24 {
            return Err(openlark_core::error::validation_error(
                "minute_token",
                "minute_token 长度必须为 24 字符",
            ));
        }
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        let api_endpoint = MinutesApiV1::StatisticsGet(minute_token);
        let mut api_request: ApiRequest<GetMinuteStatisticsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        
            let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
        }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteStatisticsResponse {
    pub statistics: MinuteStatistics,
}

impl ApiResponseTrait for GetMinuteStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStatistics {
    pub user_view_count: String,
    pub page_view_count: String,
    #[serde(default)]
    pub user_view_list: Vec<UserViewDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserViewDetail {
    pub user_id: String,
    /// 用户的最近查看时间 timestamp（ms 级别）
    pub view_time: String,
}
