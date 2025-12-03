//! 标准用户访问令牌获取API实现
//!
//! 对应 meta.project=authen, meta.resource=access_token, meta.name=create

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::models::authen::{UserAccessTokenV1Request, UserAccessTokenResponse};

// 类型别名
pub type AuthenResult<T> = SDKResult<T>;

/// 用户访问令牌构建器（v1版本）
#[derive(Debug)]
pub struct UserAccessTokenV1Builder {
    config: Config,
    request: UserAccessTokenV1Request,
}

impl UserAccessTokenV1Builder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: UserAccessTokenV1Request {
                grant_code: String::new(),
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    /// 设置授权码
    pub fn grant_code(mut self, grant_code: impl Into<String>) -> Self {
        self.request.grant_code = grant_code.into();
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    /// 发送请求获取用户访问令牌
    pub async fn send(self) -> AuthenResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/access_token", self.config.base_url);

        let request: ApiRequest<UserAccessTokenResponse> = ApiRequest::post(&url)
            .header("Content-Type", "application/json")
            .body(openlark_core::api::RequestData::Json(serde_json::to_value(&self.request)?));

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            // 映射飞书错误码
            let error_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match error_code {
                99991663 => Err(api_error(400, "/open-apis/authen/v1/access_token", "授权码无效", None::<String>)),
                99991669 => Err(api_error(400, "/open-apis/authen/v1/access_token", "用户身份解析失败", None::<String>)),
                99991671 => Err(api_error(400, "/open-apis/authen/v1/access_token", "用户访问令牌无效", None::<String>)),
                99991674 => Err(api_error(400, "/open-apis/authen/v1/access_token", "用户类型不支持", None::<String>)),
                99991675 => Err(api_error(400, "/open-apis/authen/v1/access_token", "身份不匹配", None::<String>)),
                _ => Err(api_error(error_code as u16, "/open-apis/authen/v1/access_token", error_message, None::<String>)),
            }
        }
    }
}