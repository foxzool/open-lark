/// 获取妙记信息
///
/// 通过这个接口，可以得到一篇妙记的基础概述信息，包含 owner_id、create_time、标题、封面、时长和 URL。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute/get
/// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use super::models::{MinuteInfo, UserIdType};

/// 获取妙记信息请求
pub struct GetMinuteRequest {
    minute_token: String,
    user_id_type: Option<UserIdType>,
    config: Config,
}

/// 获取妙记信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteResponse {
    /// 妙记基础信息
    pub minute: MinuteInfo,
}

impl ApiResponseTrait for GetMinuteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMinuteRequest {
    /// 创建获取妙记信息请求
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
    pub async fn send(self) -> SDKResult<GetMinuteResponse> {
        self.execute().await
    }

    /// 执行请求
    ///
    /// doc: https://open.feishu.cn/document/server-docs/minutes-v1/minute/get
    pub async fn execute(self) -> SDKResult<GetMinuteResponse> {
        validate_required!(self.minute_token, "妙记Token不能为空");

        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::Get(self.minute_token.clone());

        let mut api_request: ApiRequest<GetMinuteResponse> =
            ApiRequest::get(&api_endpoint.to_url());
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type.as_str());
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取妙记信息")
    }
}
