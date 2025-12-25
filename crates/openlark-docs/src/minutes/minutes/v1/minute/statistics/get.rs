/// 获取妙记统计数据
///
/// 通过这个接口，可以获得妙记的访问情况统计，包含用户浏览数、页面浏览数与用户浏览明细。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-statistics/get
/// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::minutes::v1::minute::models::{MinuteStatistics, UserIdType};

/// 获取妙记统计数据请求
pub struct GetMinuteStatisticsRequest {
    minute_token: String,
    user_id_type: Option<UserIdType>,
    config: Config,
}

/// 获取妙记统计数据响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteStatisticsResponse {
    /// 统计数据
    pub statistics: MinuteStatistics,
}

impl ApiResponseTrait for GetMinuteStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMinuteStatisticsRequest {
    /// 创建获取妙记统计数据请求
    pub fn new(config: Config) -> Self {
        Self {
            minute_token: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置妙记Token
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 设置用户 ID 类型（query: user_id_type）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 发送请求
    pub async fn send(self) -> SDKResult<GetMinuteStatisticsResponse> {
        self.execute().await
    }

    /// 执行请求
    ///
    /// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get
    pub async fn execute(self) -> SDKResult<GetMinuteStatisticsResponse> {
        validate_required!(self.minute_token, "妙记Token不能为空");

        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::StatisticsGet(self.minute_token.clone());

        let mut api_request: ApiRequest<GetMinuteStatisticsResponse> =
            ApiRequest::get(&api_endpoint.to_url());
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取妙记统计数据")
    }
}
