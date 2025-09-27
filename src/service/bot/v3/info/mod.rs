use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::bot::models::Bot,
};

/// 机器人信息服务
pub struct InfoService {
    pub config: Config,
}

/// 获取机器人信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetBotInfoResponse {
    /// 机器人信息
    pub bot: Bot,
}

impl ApiResponseTrait for GetBotInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl InfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取机器人信息
    ///
    /// 该接口用于获取机器人的基本信息，包括机器人名称、头像、IP白名单等信息。
    ///
    /// 注意事项：
    /// - 需要启用机器人能力
    /// - 需要申请 获取机器人信息 权限
    ///
    /// # 错误
    ///
    /// - 1011003: 参数无效
    /// - 1011002: 无法获取应用信息，请联系应用管理员开启机器人能力
    /// - 230002: 权限不足，请联系应用管理员开启对应权限范围
    pub async fn get(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetBotInfoResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/bot/v3/info".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

impl Service for InfoService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "bot_info"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
